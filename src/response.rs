use crate::filetype::FileType;
use crate::http::{HttpStatus, HttpVersion};
use crate::request::Request;
use crate::templates::{Templates, TemplatesPage};
use crate::utils::Utils;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Response {
    pub request: Request,
    pub templates: Templates,
    pub http_version: HttpVersion,
    pub status_code: HttpStatus,
    pub headers: Vec<(String, String)>,
    pub cookies: Vec<(String, String)>,
    pub body: Vec<u8>,
}

impl Response {
    pub fn new(request: Request, templates: Templates) -> Option<Self> {
        let response = Self {
            request: request.clone(),
            templates,
            http_version: HttpVersion::Http11, // default to HTTP/1.1
            status_code: HttpStatus::Ok,       // default to 200 OK
            headers: Vec::new(),
            cookies: Vec::new(),
            body: Vec::new(),
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

        match File::open(&path) {
            Ok(mut file) => {
                let extension = path.extension().unwrap().to_str().unwrap();

                let file_type = FileType::from_extension(extension)
                    .unwrap_or_else(|| FileType::new("bin", "application/octet-stream"));

                // @see: https://developer.mozilla.org/fr/docs/Web/HTTP/Headers/Content-Disposition
                let content_disposition = file_type.content_disposition();

                let mut content = Vec::new();
                if file.read_to_end(&mut content).is_ok() {
                    self.body = content;
                    self.status_code = HttpStatus::Ok;
                    self.headers.clear();
                    self.headers.push((
                        "Content-Type".to_string(),
                        file_type.content_type.to_string(),
                    ));
                    self.headers
                        .push(("Content-Length".to_string(), self.body.len().to_string()));
                    self.headers.push((
                        "Content-Disposition".to_string(),
                        content_disposition.to_string(),
                    ));
                } else {
                    self.serve_error_response(HttpStatus::InternalServerError);
                }
            }
            Err(_) => self.serve_error_response(HttpStatus::NotFound),
        }
    }

    fn serve_directory(&mut self, root_path: &PathBuf, path: PathBuf) {
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
        self.headers
            .push(("Content-Length".to_string(), self.body.len().to_string()));
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
        self.headers
            .push(("Content-Length".to_string(), self.body.len().to_string()));
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
}
