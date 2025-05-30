use crate::core::server::filetype::FileType;
use crate::core::server::http::{HttpStatus, HttpVersion};
use crate::core::utils::keyval::KeyVal;
use crate::core::utils::logger::Logger;
use crate::core::server::request::Request;
use crate::core::resources::templates::{Templates, TemplatesPage};
use crate::core::utils::utils::Utils;
use std::cmp::min;
use std::collections::HashMap;
use std::fs::File;
use std::io::{Error, Read, Seek, SeekFrom, Write};
use std::net::TcpStream;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct Response {
    pub request: Request,
    pub templates: Templates,
    pub http_version: HttpVersion,
    pub status_code: HttpStatus,
    pub headers: KeyVal,
    pub cookies: KeyVal,
    pub body: Vec<u8>,
    pub size: usize,
    pub _path: PathBuf,
    _need_stream: bool,
    _is_compiled: bool,
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
            headers: KeyVal::new(),
            cookies: KeyVal::new(),
            body: Vec::new(),
            size: 0,
            _path: PathBuf::new(),
            _need_stream: false,
            _is_compiled: false,
        };

        Some(response)
    }

    pub fn serve(&mut self, root_dir: &Path) -> &mut Response {
        Logger::debug(
            format!("[Response] Serving request for path: {}", self.request.path).as_str(),
        );

        let file_path = root_dir.join(&self.request.path[1..]);

        file_path.clone_into(&mut self._path);

        if file_path.is_dir() {
            let index_html = file_path.join("index.html");
            if index_html.is_file() {
                Logger::debug("[Response] Serving index.html from directory");
                self.serve_file(root_dir, index_html);
            } else {
                Logger::debug("[Response] Serving directory listing");
                self.serve_directory(root_dir, file_path);
            }
        } else if file_path.is_file() {
            Logger::debug("[Response] Serving file");
            self.serve_file(root_dir, file_path);
        } else {
            let display_path = Utils::path_prettifier(file_path.clone());
            Logger::warn(format!("[Response] Path not found: {}", display_path).as_str());
            self.serve_error_response(HttpStatus::NotFound);
        }

        self
    }

    fn serve_file(&mut self, root_path: &Path, path: PathBuf) {
        let display_path = Utils::path_prettifier(path.clone());
        Logger::debug(format!("[Response] Attempting to serve file: {}", display_path).as_str());

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

        path.clone_into(&mut self._path);

        match File::open(&path) {
            Ok(_file) => {
                let extension = path.extension().and_then(|ext| ext.to_str()).unwrap_or("");

                let file_type = FileType::from_extension(extension)
                    .unwrap_or_else(|| FileType::new("bin", "application/octet-stream"));

                // @see: https://developer.mozilla.org/fr/docs/Web/HTTP/Headers/Content-Disposition
                let content_disposition = file_type.content_disposition();

                // get file size without reading
                let metadata = std::fs::metadata(&path).expect("Unable to read metadata"); // self.body.len().to_string()
                let file_size = metadata.len();
                let is_readable = Utils::is_readable_from_metadata(metadata.clone());

                if !is_readable {
                    Logger::error(
                        format!("[Response] File not readable: {}", path.display()).as_str(),
                    );
                    self.serve_error_response(HttpStatus::InternalServerError);
                }

                self.size = file_size as usize;

                if self.size > Response::MAX_SIZE_ALL_AT_ONCE {
                    Logger::debug(
                        format!(
                            "[Response] File size {} exceeds MAX_SIZE_ALL_AT_ONCE, will stream",
                            self.size
                        )
                        .as_str(),
                    );
                    self._need_stream = true;
                }

                self.status_code = HttpStatus::Ok;
                self.headers.clear();

                // @see: https://stackoverflow.com/a/28652339/13158370
                if extension == "" {
                    Logger::debug("[Response] No extension found, not using content type");
                    return;
                }

                Logger::debug(format!("[Response] Found extension: {}", extension).as_str());

                self.headers.add(
                    "Content-Type".to_string(),
                    file_type.content_type.to_string(),
                );
                self.headers.add(
                    "Content-Disposition".to_string(),
                    content_disposition.to_string(),
                );
            }
            Err(_) => self.serve_error_response(HttpStatus::NotFound),
        }
    }

    fn serve_directory(&mut self, root_path: &Path, path: PathBuf) {
        Logger::debug(
            format!(
                "[Response] Serving directory listing for: {}",
                path.display()
            )
            .as_str(),
        );

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

        self._path.clone_from(&path);

        let entries = Utils::walk_dir(&path);
        Logger::debug(format!("[Response] Found {} entries in directory", entries.len()).as_str());

        let mut folders = KeyVal::new();
        let mut files = KeyVal::new();

        for (entry_type, entry_name, entry_path) in &entries {
            if entry_type == "directory" {
                folders.add(entry_name.to_string(), entry_path.to_string());
            } else {
                files.add(entry_name.to_string(), entry_path.to_string());
            }
        }

        if relative_path != "/" {
            listing_html.push_str("<li><a href='../'>..</a></li>");
        }

        if entries.is_empty() {
            listing_html.push_str("<li><b>Empty Folder</b></li>");
        }

        for (entry_name, entry_path) in folders.iter() {
            let li_href = entry_path.strip_prefix(root_dir_normalized).unwrap();
            listing_html.push_str(&format!(
                "<li><a href='{}'>{}</a></li>",
                li_href, entry_name
            ));
        }

        for (entry_name, entry_path) in files.iter() {
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
            .add("Content-Type".to_string(), "text/html".to_string());

        self.size = self.body.len()
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
            .add("Content-Type".to_string(), "text/html".to_string());

        self.size = self.body.len()
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

        // Use iter() instead of map()
        for (key, value) in self.headers.iter() {
            result.push_str(&format!("{}: {}\r\n", key.trim(), value.trim()));
        }

        for (key, value) in self.cookies.iter() {
            result.push_str(&format!("Set-Cookie: {}={}\r\n", key.trim(), value.trim()));
        }

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
        Logger::debug("[Response] Starting stream response");

        self.headers
            .add("Content-Length".to_string(), self.size.to_string());

        if self._is_compiled {
            if self.body.is_empty() {
                Logger::error("[Response] Body is empty while expecting compiled content");
                self.serve_error_response(HttpStatus::InternalServerError);
                stream.write_all(self.to_bytes().as_slice())?;
                stream.flush()?;
                return Ok(());
            }

            Logger::debug("[Response] Streaming compiled content");
            stream.write_all(self.to_bytes().as_slice())?;
            stream.flush()?;
            return Ok(());
        }

        if !self._need_stream {
            let mut file = match File::open(&self._path) {
                Ok(file) => file,
                Err(_) => {
                    let display_path = Utils::path_prettifier(self._path.clone());
                    Logger::error(
                        format!("Failed to open file: {}", display_path).as_str(),
                    );
                    self.serve_error_response(HttpStatus::NotFound);
                    stream.write_all(self.to_bytes().as_slice())?;
                    stream.flush()?;
                    return Ok(());
                }
            };

            // read into a buffer
            let mut buffer = vec![0; self.size];
            file.read_exact(&mut buffer)?;
            self.body = buffer;

            stream.write_all(self.to_bytes().as_slice()).unwrap();
            stream.flush()?;
            return Ok(());
        }

        let _: Result<(), Error> = match self.stream_by_chunk(stream) {
            Ok(_) => Ok(()),
            Err(error) => {
                Logger::error(
                    format!("[Response] Error while streaming by chunk: {}", error).as_str(),
                );
                self.serve_error_response(HttpStatus::InternalServerError);
                stream.write_all(self.to_bytes().as_slice())?;
                return Ok(());
            }
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
                let display_path = Utils::path_prettifier(self._path.clone());
                Logger::error(format!("Failed to open file: {}", display_path).as_str());
                self.serve_error_response(HttpStatus::NotFound);
                stream.write_all(self.to_bytes().as_slice())?;
                stream.flush()?;
                return Ok(());
            }
        };

        Logger::debug(
            format!(
                "[Response] Sending response in chunks with size: {}",
                self.size
            )
            .as_str(),
        );

        self.headers
            .add("Content-Length".to_string(), self.size.to_string());

        // @see: https://datatracker.ietf.org/doc/html/rfc7233
        self.headers
            .add("Accept-Ranges".to_string(), "bytes".to_string());

        // check if range header is present
        if let Some(range) = self
            .request
            .headers
            .iter()
            .find(|(k, _)| k.to_lowercase() == *"range")
            .map(|(_, v)| v)
        {
            Logger::debug(format!("[Response] Processing range request: {}", range).as_str());

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
            let end = range_values[1].parse::<usize>().unwrap_or(self.size - 1);

            if start >= self.size || end >= self.size || start > end {
                // return http 416 Range Not Satisfiable
                // @see: https://http.dev/416
                self.status_code = HttpStatus::RangeNotSatisfiable;
                self.headers.add(
                    "Content-Range".to_string(),
                    format!("bytes */{}", self.size),
                );
                stream.write_all(self.http_description().as_bytes())?;
                stream.write_all(b"\r\n")?;
                stream.flush()?;
                return Ok(());
            }

            // set status code for response to 206
            self.status_code = HttpStatus::PartialContent;
            self.headers.add(
                "Content-Range".to_string(),
                format!("bytes {}-{}/{}", start, end, self.size),
            );
            self.headers
                .add("Content-Length".to_string(), (end - start + 1).to_string());

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
