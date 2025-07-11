use crate::core::config::config::Config;
use crate::core::server::http::{HttpMethod, HttpStatus};
use crate::core::utils::logger::Logger;
use crate::core::server::request::Request;
use crate::core::server::response::Response;
use crate::core::resources::templates::Templates;
use crate::core::utils::utils::Utils;
use std::net::{TcpListener, TcpStream};
use std::ops::DerefMut;
use std::thread;

pub struct Server {
    config: Config,
    templates: Templates,
}

impl Server {
    const SERVER_NAME: &'static str = "Katana";
    const SERVER_VERSION: &'static str = "0.1.0";
    pub const SUPPORTED_HTTP_METHODS: &'static [HttpMethod] = &[
        HttpMethod::GET,
        HttpMethod::HEAD,
        HttpMethod::OPTIONS,
        HttpMethod::TRACE,
    ];

    pub fn new(config: Config, templates: Templates) -> Self {
        Self { config, templates }
    }

    pub fn serve(&self) {
        Logger::debug(format!("[Server] Starting {} on {}", Self::version(), self.addr()).as_str());
        let listener = TcpListener::bind(self.addr().as_str()).unwrap();
        Logger::debug("[Server] Server is ready to accept connections");

        for stream in listener.incoming().flatten() {
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

    pub fn log_source_ip(&self, stream: &TcpStream) {
        Logger::debug(
            format!(
                "New connection from {}",
                Utils::get_peer_ip(stream)
            )
            .as_str(),
        );
    }

    pub fn handle_request(&self, mut stream: TcpStream) {
        self.log_source_ip(&stream);
        
        if let Some(request) = Request::from_stream(&stream) {
            Logger::debug(
                format!(
                    "[Server] Request received: {} {}",
                    request.method.as_str(),
                    request.path
                )
                .as_str(),
            );
            self.handle_response(request, &mut stream);
        } else {
            Logger::warn("[Server] Failed to read request");
        }
    }

    pub fn handle_response(&self, request: Request, mut stream: &mut TcpStream) {
        if let Some(mut response) = Response::new(request, self.templates.to_owned()) {
            response.serve(&self.config.document_root);
            self.method_handle(&mut response);
            self.server_transformation(&mut response);

            let result = response.stream(stream.deref_mut());
            match result {
                Ok(_response) => {
                    Logger::debug(
                        format!(
                            "[Server] Response sent successfully: {}",
                            response.status_code.to_code()
                        )
                        .as_str(),
                    );
                    Self::log_response(&response)
                }
                Err(e) => {
                    Logger::error(format!("[Server] Stream error: {}", e).as_str());
                }
            }
        } else {
            Logger::warn("[Server] Failed to create response");
        }
    }

    pub fn addr(&self) -> String {
        format!("{}:{}", self.config.host, self.config.port)
    }

    pub fn addr_with_protocol(&self) -> String {
        format!("http://{}", self.addr())
    }

    pub fn version() -> String {
        format!("{} {}", Self::SERVER_NAME, Self::SERVER_VERSION)
    }

    pub fn server_transformation(&self, response: &mut Response) {
        response.headers.add("Server".to_string(), Self::version());
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

            response
                .headers
                .add("Date".to_string(), Utils::datetime_rfc_1123().to_string());
            response.headers.add(
                "Allow".to_string(),
                HttpMethod::comma_separated(Self::SUPPORTED_HTTP_METHODS),
            );
            // @see: https://developer.mozilla.org/en-US/docs/Web/HTTP/CORS
            response
                .headers
                .add("Access-Control-Allow-Origin".to_string(), "*".to_string());
            response.headers.add(
                "Access-Control-Allow-Methods".to_string(),
                HttpMethod::comma_separated(Self::SUPPORTED_HTTP_METHODS),
            );
            // response.headers.add(
            //     "Access-Control-Allow-Headers".to_string(),
            //     "content-type, accept".to_string()
            // );
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
            response
                .headers
                .add("Content-Type".to_string(), "message/http".to_string());

            // new body
            let body = format!("\r\n{}", response.request.http_description());

            // new body length
            response
                .headers
                .add("Content-Length".to_string(), body.len().to_string());

            // set new body
            response.body = body.into_bytes();
        }

        if !Self::SUPPORTED_HTTP_METHODS.contains(&response.request.method) {
            // do not return body
            response.body = Vec::new();
            // headers
            response.headers.clear();
            response.headers.add(
                "Allow".to_string(),
                HttpMethod::comma_separated(Self::SUPPORTED_HTTP_METHODS),
            );
            // status
            response.status_code = HttpStatus::MethodNotAllowed;
        }
    }

    pub fn log_response(response: &Response) {
        let status_line = response
            .request
            .to_string()
            .lines()
            .next()
            .unwrap()
            .to_string();
        let log_message = &format!(
            "\"{}\" {} {}",
            status_line,
            response.status_code.to_code(),
            response.size,
        );
        Logger::info(log_message);
    }
}
