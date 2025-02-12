use std::io::Write;
use std::net::{TcpListener, TcpStream};
use std::thread;
use crate::config::Config;
use crate::http::{HttpMethod, HttpStatus};
use crate::logger::{Logger, LogLevel};
use crate::request::Request;
use crate::response::Response;
use crate::templates::Templates;
use crate::utils::Utils;

pub struct Server {
    config: Config,
    templates: Templates,
}

impl Server {
    const SERVER_NAME: &'static str = "Katana";
    const SERVER_VERSION: &'static str = "0.1.0";
    pub const SUPPORTED_HTTP_METHODS: &'static[HttpMethod] = &[
        HttpMethod::GET,
        HttpMethod::HEAD,
        HttpMethod::OPTIONS,
        HttpMethod::TRACE,
    ];


    pub fn new(config: Config, templates: Templates) -> Self {
        let http_server = Self {
            config,
            templates,
        };

        return http_server;
    }

    pub fn serve(&self) {
        let listener = TcpListener::bind(self.addr().as_str()).unwrap();

        for stream in listener.incoming() {
            if let Ok(stream) = stream {
                // spawn a new thread for each connection
                let config = self.config.clone();
                let templates = self.templates.clone();

                thread::spawn(move || {
                    // create a new server instance for the thread with the necessary data
                    let server = Server::new(config, templates);
                    server.handle_request(stream);
                });
            }
        }
    }

    pub fn handle_request(&self, mut stream: TcpStream) {
        if let Some(request) = Request::from_stream(&stream) {
            self.handle_response(request, &mut stream);
        } else {
            Logger::log(LogLevel::WARN, "Failed to read request.")
        }
    }

    pub fn handle_response(&self, request: Request, stream: &mut TcpStream) {
        if let Some(mut response) = Response::new(request, self.templates.to_owned()) {
            response.serve(&self.config.root_dir);
            self.method_handle(&mut response);
            self.server_transformation(&mut response);
            let _ = stream.write_all(response.to_bytes().as_slice()).unwrap();
            stream.flush().unwrap();
            Self::log_response(&response);
        } else {
            Logger::log(LogLevel::WARN, "Failed to send response.")
        }
    }

    pub fn addr(&self) -> String {
        format!("{}:{}", self.config.host, self.config.port)
    }

    pub fn addr_with_protocol(&self) -> String {
        format!("http://{}", self.addr())
    }

    pub fn version() -> String {
        format!("{} {}", Self::SERVER_NAME.to_string(), Self::SERVER_VERSION.to_string())
    }

    pub fn server_transformation(&self, response: &mut Response) {
        // add to headers server name
        response.headers.push(("Server".to_string(), Self::version()));
    }

    pub fn method_handle(&self, response: &mut Response) {
        if response.request.method == HttpMethod::GET {
            // nothing, process as usual
        }

        if response.request.method == HttpMethod::HEAD {
            // do not return body
            response.body = Vec::new();
        }

        if response.request.method == HttpMethod::OPTIONS {
            // do not return body
            response.body = Vec::new();

            // headers
            response.headers.push(("Date".to_string(), Utils::datetime_rfc_1123().to_string()));
            response.headers.push(("Allow".to_string(), "GET, HEAD, OPTIONS, TRACE".to_string()));
            // @see: https://developer.mozilla.org/en-US/docs/Web/HTTP/CORS
            response.headers.push(("Access-Control-Allow-Origin".to_string(), "*".to_string()));
            response.headers.push(("Access-Control-Allow-Methods".to_string(), "GET, HEAD, OPTIONS, TRACE".to_string()));
            // response.headers.push(("Access-Control-Allow-Headers".to_string(), "content-type, accept".to_string()));
        }

        if response.request.method == HttpMethod::TRACE {
            // do not return body
            response.body = Vec::new();

            // We supports TRACE universally (ignoring route existence), so it will always be 200 OK
            // @see: https://developer.mozilla.org/en-US/docs/Web/HTTP/Methods/TRACE#successful_trace_request
            response.status_code = HttpStatus::Ok;

            // flush headers
            response.headers.clear();

            // correct type
            response.headers.push(("Content-Type".to_string(), "message/http".to_string()));

            // new body
            let body = format!("{}\r\n", response.request.http_description());

            // new body length
            response.headers.push(("Content-Length".to_string(), body.len().to_string()));

            // set new body
            response.body = body.into_bytes();
        }

        if !Self::SUPPORTED_HTTP_METHODS.contains(&response.request.method) {
            // do not return body
            response.body = Vec::new();
            // headers
            response.headers.clear();
            response.headers.push(("Allow".to_string(), "GET, HEAD, OPTIONS, TRACE".to_string()));
            // status
            response.status_code = HttpStatus::MethodNotAllowed;
        }
    }

    pub fn log_response(response: &Response) {
        let status_line = response.request.to_string().lines().next().unwrap().to_string();
        let log_message = &format!(
            "\"{}\" {} {}",
            status_line,
            response.status_code.to_code(),
            response.body.len(),
        );
        Logger::log(LogLevel::INFO, log_message);
    }
}
