use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use crate::request::Request;
use crate::http::{HttpVersion, HttpStatus};

#[derive(Debug)]
pub struct Response {
    pub request: Request,
    pub http_version: HttpVersion,
    pub status_code: HttpStatus,
    pub headers: Vec<(String, String)>,
    pub cookies: Vec<(String, String)>,
    pub body: String,
}

impl Response {
    pub fn new(request: Request) -> Option<Self> {
        let response = Self {
            request: request.clone(),
            http_version: HttpVersion::Http11, // default to HTTP/1.1
            status_code: HttpStatus::Ok, // default to 200 OK
            headers: Vec::new(),
            cookies: Vec::new(),
            body: String::new(),
        };

        Some(response)
    }

    pub fn serve(&mut self, root_dir: &PathBuf) -> &mut Response {
        let file_path = root_dir.join(&self.request.path[1..]); // Remove leading "/"

        if file_path.is_dir() {
            let index_html = file_path.join("index.html");
            if index_html.is_file() {
                self.serve_file(index_html);
            } else {
                self.serve_directory(file_path);
            }
        } else if file_path.is_file() {
            self.serve_file(file_path);
        } else {
            self.serve_error_response(HttpStatus::NotFound);
        }

        return self;
    }

    fn serve_file(&mut self, path: PathBuf) {
        match File::open(&path) {
            Ok(mut file) => {
                let mut content = String::new();
                if file.read_to_string(&mut content).is_ok() {
                    self.body = content;
                    self.status_code = HttpStatus::Ok;
                    self.headers.clear();
                    self.headers.push(("Content-Type".to_string(), "text/html".to_string()));
                    self.headers.push(("Content-Length".to_string(), self.body.len().to_string()));
                } else {
                    self.serve_error_response(HttpStatus::InternalServerError);
                }
            }
            Err(_) => self.serve_error_response(HttpStatus::NotFound),
        }
    }

    fn serve_directory(&mut self, path: PathBuf) {
        self.body = format!("<html><body><h1>Directory listing for {}</h1></body></html>", path.display());
        self.status_code = HttpStatus::Ok;
        self.headers.clear();
        self.headers.push(("Content-Type".to_string(), "text/html".to_string()));
        self.headers.push(("Content-Length".to_string(), self.body.len().to_string()));
    }

    fn serve_error_response(&mut self, status: HttpStatus) {
        self.status_code = status;
        self.body = format!("<html><body><h1>{}</h1></body></html>", self.status_code.to_message());
        self.headers.clear();
        self.headers.push(("Content-Type".to_string(), "text/html".to_string()));
        self.headers.push(("Content-Length".to_string(), self.body.len().to_string()));
    }

    pub fn to_string(&self) -> String {
        let mut result = String::new();

        // format the status line
        result.push_str(&format!(
            "{} {} {}\r\n",
            self.http_version.as_str(),
            self.status_code.to_code(),
            self.status_code.to_message()
        ));

        // format headers
        let headers = self.headers.iter()
            .map(|(k, v)| format!("{}: {}\r\n", k.trim(), v.trim()))
            .collect::<String>();
        result.push_str(&headers);

        // format cookies
        let cookies = self.cookies.iter()
            .map(|(k, v)| format!("Set-Cookie: {}={}\r\n", k.trim(), v.trim()))
            .collect::<String>();
        result.push_str(&cookies);

        // format body
        result.push_str("\r\n"); // add a blank line between headers and body
        result.push_str(&self.body);

        return result;
    }
}
