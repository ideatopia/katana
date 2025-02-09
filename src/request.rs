use std::io;
use std::io::BufRead;
use std::net::TcpStream;
use crate::http::HttpVersion;

#[derive(Debug)]
#[derive(Clone)]
pub struct Request {
    pub version: HttpVersion,
    pub domain: String,
    pub path: String,
    pub method: String,
    pub queries: Vec<(String, String)>,
    pub headers: Vec<(String, String)>,
    pub cookies: Vec<(String, String)>,
}

impl Request {
    pub fn from_stream(mut stream: &TcpStream) -> Option<Self> {
        let reader = io::BufReader::new(&mut stream);
        let mut lines = reader.lines();

        // read the first line "GET /path?foo=bar HTTP/1.1"
        let request_line = lines.next()?.ok()?;

        let parts: Vec<&str> = request_line.split_whitespace().collect();
        if parts.len() < 3 {
            return None; // invalid request
        }

        let method = parts[0].to_string();
        let raw_path = parts[1];
        let mut path = Self::decode_url(raw_path);
        let version = HttpVersion::from_str(&parts[2].replace("HTTP/", "")).unwrap();

        let mut domain = String::new();
        let mut queries = Vec::new();
        let mut headers = Vec::new();
        let mut cookies = Vec::new();

        // extract queries from the path
        if let Some((path_part, query_part)) = path.clone().split_once('?') {
            path = path_part.to_string();
            queries = query_part
                .split('&')
                .filter_map(|pair| pair.split_once('='))
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect();
        }

        // read headers
        for line in lines.by_ref() {
            let line = line.ok()?;
            if line.is_empty() {
                break; // End of headers
            }

            if let Some((key, value)) = line.split_once(": ") {
                let key = key.to_string();
                let value = value.to_string();
                headers.push((key.clone(), value.clone()));

                if key.to_lowercase() == "host" {
                    domain = value;
                } else if key.to_lowercase() == "cookie" {
                    cookies = value
                        .split("; ")
                        .filter_map(|cookie| cookie.split_once('='))
                        .map(|(k, v)| (k.to_string(), v.to_string()))
                        .collect();
                }
            }
        }

        // body and HTTP verbs are ignored for now

        Some(Self {
            method,
            path,
            version,
            domain,
            queries,
            headers,
            cookies,
        })
    }

    pub fn decode_url(url: &str) -> String {
        let mut result = String::with_capacity(url.len());
        let mut chars = url.chars().peekable();

        while let Some(c) = chars.next() {
            if c == '%' {
                let mut hex = String::with_capacity(2);
                if let Some(h1) = chars.next() {
                    hex.push(h1);
                    if let Some(h2) = chars.next() {
                        hex.push(h2);
                        if let Ok(byte) = u8::from_str_radix(&hex, 16) {
                            result.push(byte as char);
                            continue;
                        }
                    }
                }
                result.push('%');
                for h in hex.chars() {
                    result.push(h);
                }
            } else if c == '+' {
                result.push(' ');
            } else {
                result.push(c);
            }
        }
        result
    }

    pub fn to_string(&self) -> String {
        let mut result = String::new();

        // format the status line
        result.push_str(&format!(
            "{} {} {}\r\n",
            self.method,
            self.path,
            self.version.as_str()
        ));

        // add query parameters as part of the URL
        if !self.queries.is_empty() {
            let query_str: Vec<String> = self.queries.iter()
                .map(|(k, v)| format!("{}={}", k.trim(), v.trim()))
                .collect();
            let query_string = query_str.join("&");
            result.push_str(&format!("{}?{}\r\n", self.path, query_string));
        }

        // add headers (each header should be "Key: Value")
        for (key, value) in &self.headers {
            result.push_str(&format!("{}: {}\r\n", key.trim(), value.trim()));
        }

        return result;
    }
}
