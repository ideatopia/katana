use std::cmp::min;
use crate::filetype::FileType;
use crate::http::{HttpStatus, HttpVersion};
use crate::request::Request;
use crate::templates::{Templates, TemplatesPage};
use crate::utils::Utils;
use std::collections::HashMap;
use std::fs::File;
use std::io::{Error, Read, Seek, SeekFrom, Write};
use std::net::TcpStream;
use std::path::PathBuf;
use crate::logger::Logger;

#[derive(Debug)]
pub struct Response {
    pub request: Request,
    pub templates: Templates,
    pub http_version: HttpVersion,
    pub status_code: HttpStatus,
    pub headers: Vec<(String, String)>,
    pub cookies: Vec<(String, String)>,
    pub body: Vec<u8>,
    pub _size: usize,
    pub _path: PathBuf,
    pub _need_stream: bool,
    pub _is_compiled: bool,
}

impl Response {
    pub const CHUNK_SIZE: usize = 1024; // 1 KB
    pub const MAX_SIZE_ALL_AT_ONCE: usize = 1048576; // 1MB

    pub fn new(request: Request, templates: Templates) -> Option<Self> {
        let response = Self {
            request: request.clone(),
            templates,
            http_version: HttpVersion::Http11, // default to HTTP/1.1
            status_code: HttpStatus::Ok,       // default to 200 OK
            headers: Vec::new(),
            cookies: Vec::new(),
            body: Vec::new(),
            _size: 0,
            _path: PathBuf::new(),
            _need_stream: false,
            _is_compiled: false,
        };

        Some(response)
    }

    pub fn serve(&mut self, root_dir: &PathBuf) -> &mut Response {
        let file_path = root_dir.join(&self.request.path[1..]); // Remove leading "/"

        if file_path.is_dir() {
            let index_html = file_path.join("index.html");
            if index_html.is_file() {
                self.serve_file(root_dir, index_html);
            } else {
                self.serve_directory(root_dir, file_path);
            }
        } else if file_path.is_file() {
            self.serve_file(root_dir, file_path);
        } else {
            self.serve_error_response(HttpStatus::NotFound);
        }

        self
    }

    fn serve_file(&mut self, root_path: &PathBuf, path: PathBuf) {
        let name = path.file_name().unwrap().to_string_lossy().to_string();

        let root_dir = root_path.to_str().unwrap();

        let relative_path = match path.strip_prefix(root_dir) {
            Ok(relative) => relative.to_string_lossy().to_string(),
            Err(_) => String::from("/"), // fallback in case of error
        };

        if (relative_path.starts_with("/.") || relative_path.starts_with('.'))
            && !relative_path.contains(".well-known")
        {
            self.serve_error_response(HttpStatus::Forbidden);
            return;
        }

        // do not serve files starting with dot "." except those with ".well-known" in the name
        if name.starts_with('.') && name != ".well-known" {
            self.serve_error_response(HttpStatus::Forbidden);
            return;
        }

        self._path = path.to_owned();

        match File::open(&path) {
            Ok(_file) => {
                let extension = path.extension().unwrap().to_str().unwrap();

                let file_type = FileType::from_extension(extension)
                    .unwrap_or_else(|| FileType::new("bin", "application/octet-stream"));

                // @see: https://developer.mozilla.org/fr/docs/Web/HTTP/Headers/Content-Disposition
                let content_disposition = file_type.content_disposition();

                // get file size without reading
                let metadata = std::fs::metadata(&path).expect("Unable to read metadata"); // self.body.len().to_string()
                let file_size = metadata.len();
                let is_readable = metadata.permissions().readonly();

                if !is_readable {
                    self.serve_error_response(HttpStatus::InternalServerError);
                }

                self._size = file_size as usize;

                if self._size > Response::MAX_SIZE_ALL_AT_ONCE {
                    self._need_stream = true;
                }

                self.status_code = HttpStatus::Ok;
                self.headers.clear();
                self.headers.push((
                    "Content-Type".to_string(),
                    file_type.content_type.to_string(),
                ));
                self.headers.push((
                    "Content-Disposition".to_string(),
                    content_disposition.to_string(),
                ));
            }
            Err(_) => self.serve_error_response(HttpStatus::NotFound),
        }
    }

    fn serve_directory(&mut self, root_path: &PathBuf, path: PathBuf) {
        self._is_compiled = true;

        let mut listing_html = String::new();

        let root_dir = root_path.to_str().unwrap();
        let binding = root_dir.replace('\\', "/");
        let root_dir_normalized = binding.trim();

        let mut relative_path = match path.strip_prefix(root_dir) {
            Ok(relative) => relative.to_string_lossy().to_string(),
            Err(_) => String::from("/"), // fallback in case of error
        };

        relative_path.insert(0, '/'); // append / to navigate easily to parent folder

        if relative_path.starts_with("/.") || relative_path.starts_with('.') {
            self.serve_error_response(HttpStatus::Forbidden);
            return;
        }

        self._path = path.to_owned();

        let entries = Utils::walk_dir(&path);
        let mut folders = Vec::new();
        let mut files = Vec::new();

        for (entry_type, entry_name, entry_path) in &entries {
            if entry_type == "directory" {
                folders.push((entry_name, entry_path));
            } else {
                files.push((entry_name, entry_path));
            }
        }

        if relative_path != "/" {
            listing_html.push_str("<li><a href='../'>..</a></li>");
        }

        if entries.is_empty() {
            listing_html.push_str("<li><b>Empty Folder</b></li>");
        }

        for (entry_name, entry_path) in folders {
            let li_href = entry_path.strip_prefix(root_dir_normalized).unwrap();
            listing_html.push_str(&format!(
                "<li><a href='{}'>{}</a></li>",
                li_href, entry_name
            ));
        }

        for (entry_name, entry_path) in files {
            let li_href = entry_path.strip_prefix(root_dir_normalized).unwrap();
            listing_html.push_str(&format!(
                "<li><a href='{}'>{}</a></li>",
                li_href, entry_name
            ));
        }

        let mut params = HashMap::new();
        params.insert("folder".to_string(), relative_path);
        params.insert("directory_content".to_string(), listing_html);

        self.body = self
            .templates
            .render(TemplatesPage::DIRECTORY, params)
            .into_bytes();
        self.status_code = HttpStatus::Ok;
        self.headers.clear();
        self.headers
            .push(("Content-Type".to_string(), "text/html".to_string()));

        self._size = self.body.len()
    }

    fn serve_error_response(&mut self, status: HttpStatus) {
        let mut params = HashMap::new();
        params.insert("status_code".to_string(), status.to_code().to_string());
        params.insert("status_text".to_string(), status.to_message().to_string());
        params.insert(
            "error_message".to_string(),
            "Something went wrong !".to_string(),
        ); //

        self.status_code = status;
        self.body = self
            .templates
            .render(TemplatesPage::ERROR, params)
            .into_bytes();
        self.headers.clear();
        self.headers
            .push(("Content-Type".to_string(), "text/html".to_string()));

        self._size = self.body.len()
    }

    pub fn http_description(&self) -> String {
        let mut result = String::new();

        // format the status line
        result.push_str(&format!(
            "{} {} {}\r\n",
            self.http_version.as_str(),
            self.status_code.to_code(),
            self.status_code.to_message()
        ));

        // format headers
        let headers = self
            .headers
            .iter()
            .map(|(k, v)| format!("{}: {}\r\n", k.trim(), v.trim()))
            .collect::<String>();
        result.push_str(&headers);

        // format cookies
        let cookies = self
            .cookies
            .iter()
            .map(|(k, v)| format!("Set-Cookie: {}={}\r\n", k.trim(), v.trim()))
            .collect::<String>();
        result.push_str(&cookies);

        result
    }

    pub fn to_string(&self) -> String {
        let mut result = self.http_description();

        result.push_str("\r\n"); // add a blank line between headers and body
        result.push_str(String::from_utf8_lossy(self.body.as_slice()).as_ref());

        result
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        bytes.extend_from_slice(self.http_description().as_bytes());
        bytes.extend_from_slice("\r\n".as_bytes()); // add a blank line between headers and body
        bytes.extend_from_slice(&self.body);

        bytes
    }

    pub fn stream(&mut self, stream: &mut TcpStream) -> Result<(), Error> {
        self.headers.push(("Content-Length".to_string(), self._size.to_string()));

        if self._is_compiled {
            if self.body.len() == 0 {
                Logger::error("Body is empty while expecting body to have compiled content");
                self.serve_error_response(HttpStatus::InternalServerError);
                stream.write_all(self.to_bytes().as_slice())?;
                stream.flush()?;
                return Ok(());
            }

            stream.write_all(self.to_bytes().as_slice())?;
            stream.flush()?;
            return Ok(());
        }

        if !self._need_stream {
            let mut file = match File::open(&self._path) {
                Ok(file) => file,
                Err(_) => {
                    Logger::error(format!("Failed to open file: {}", self._path.display()).as_str());
                    self.serve_error_response(HttpStatus::NotFound);
                    stream.write_all(self.to_bytes().as_slice())?;
                    stream.flush()?;
                    return Ok(());
                }
            };

            // read into a buffer
            let mut buffer = vec![0; self._size];
            file.read_exact(&mut buffer)?;
            self.body = buffer;

            stream.write_all(self.to_bytes().as_slice()).unwrap();
            stream.flush()?;
            return Ok(());
        }

        let _ : Result<(), Error>  = match self.stream_by_chunk(stream) {
            Ok(_) => Ok(()),
            Err(error) => {
                Logger::error(format!("Error while streaming by chunk: {}", error).as_str());
                self.serve_error_response(HttpStatus::InternalServerError);
                stream.write_all(self.to_bytes().as_slice())?;
                return Ok(());
            },
        };

        Ok(())
    }

    fn stream_by_chunk(&mut self, stream: &mut TcpStream) -> Result<(), Error> {
        // @see: https://developer.mozilla.org/fr/docs/Web/HTTP/Reference/Status/206
        // @see: https://www.rfc-editor.org/rfc/rfc2616.html#section-14.35

        // handle file opening with proper error handling
        let mut file = match File::open(&self._path) {
            Ok(file) => file,
            Err(_) => {
                Logger::error(format!("Failed to open file: {}", self._path.display()).as_str());
                self.serve_error_response(HttpStatus::NotFound);
                stream.write_all(self.to_bytes().as_slice())?;
                stream.flush()?;
                return Ok(());
            }
        };

        Logger::debug(format!("[Response] Sending response in chunks with size: {}", self._size).as_str());

        self.headers.push(("Content-Length".to_string(), self._size.to_string()));

        // @see: https://datatracker.ietf.org/doc/html/rfc7233
        self.headers.push(("Accept-Ranges".to_string(), "bytes".to_string()));

        // check if range header is present
        if let Some(range) = self.request.headers.iter().find(|(k, _)| k == "Range").map(|(_, v)| v) {
            // parse range header value and extract bytes start, end
            if !range.starts_with("bytes=") {
                self.serve_error_response(HttpStatus::BadRequest);
                stream.write_all(self.to_bytes().as_slice())?;
                stream.flush()?;
                return Ok(());
            }

            let range_values: Vec<&str> = range[6..].split('-').collect();
            if range_values.len() != 2 {
                self.serve_error_response(HttpStatus::BadRequest);
                stream.write_all(self.to_bytes().as_slice())?;
                stream.flush()?;
                return Ok(());
            }

            let start = range_values[0].parse::<usize>().unwrap_or(0);
            let end = range_values[1].parse::<usize>().unwrap_or(self._size - 1);

            if start >= self._size || end >= self._size || start > end {
                // return http 416 Range Not Satisfiable
                // @see: https://http.dev/416
                self.status_code = HttpStatus::RangeNotSatisfiable;
                self.headers.push(("Content-Range".to_string(), format!("bytes */{}", self._size)));
                stream.write_all(self.http_description().as_bytes())?;
                stream.write_all(b"\r\n")?;
                stream.flush()?;
                return Ok(());
            }

            // set status code for response to 206
            self.status_code = HttpStatus::PartialContent;
            self.headers.push(("Content-Range".to_string(),
                               format!("bytes {}-{}/{}", start, end, self._size)));
            self.headers.push(("Content-Length".to_string(),
                               (end - start + 1).to_string()));

            stream.write_all(self.http_description().as_bytes())?;
            stream.write_all(b"\r\n")?;

            // set start position to avoid reading the whole file
            file.seek(SeekFrom::Start(start as u64))?;

            // stream the requested range in chunks
            let mut remaining = end - start + 1;
            let mut buffer = vec![0; min(Response::CHUNK_SIZE, remaining)];

            while remaining > 0 {
                let to_read = min(buffer.len(), remaining);
                let bytes_read = file.read(&mut buffer[..to_read])?;
                if bytes_read == 0 {
                    break;
                }
                stream.write_all(&buffer[..bytes_read])?;
                remaining -= bytes_read;
            }
        } else {
            // no range header, stream entire file
            stream.write_all(self.http_description().as_bytes())?;
            stream.write_all(b"\r\n")?;

            // stream the file in chunks
            let mut buffer = vec![0; Response::CHUNK_SIZE];
            loop {
                let bytes_read = file.read(&mut buffer)?;
                if bytes_read == 0 {
                    break;
                }
                stream.write_all(&buffer[..bytes_read])?;
            }
        }

        stream.flush()?;

        Ok(())
    }
}
