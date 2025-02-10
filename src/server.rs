use std::io::Write;
use std::net::{TcpListener, TcpStream};
use std::path::PathBuf;
use std::thread;
use crate::logger::{Logger, LogLevel};
use crate::request::Request;
use crate::response::Response;
use crate::templates::Templates;

pub struct Server {
    host: String,
    port: u16,
    root_dir: PathBuf,
    templates: Templates,
}

impl Server {
    const SERVER_NAME: &'static str = "Katana";
    const SERVER_VERSION: &'static str = "0.1.0";

    pub fn new(host: String, port: u16, root_dir: PathBuf) -> Self {
        let http_server = Self {
            host,
            port,
            root_dir,
            templates: Templates::load(),
        };

        return http_server;
    }

    pub fn serve(&self) {
        let listener = TcpListener::bind(self.addr().as_str()).unwrap();

        for stream in listener.incoming() {
            if let Ok(stream) = stream {
                // spawn a new thread for each connection
                let host = self.host.clone();
                let port = self.port;
                let root_dir = self.root_dir.clone();

                thread::spawn(move || {
                    // create a new server instance for the thread with the necessary data
                    let server = Server::new(host, port, root_dir);
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
        if let Some(mut response) = Response::new(request) {
            response.serve(&self.root_dir);
            self.server_transformation(&mut response);
            let _ = stream.write_all(response.to_string().as_bytes());
            Self::log_response(&response);
        } else {
            Logger::log(LogLevel::WARN, "Failed to send response.")
        }
    }

    pub fn addr(&self) -> String {
        format!("{}:{}", self.host, self.port)
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
