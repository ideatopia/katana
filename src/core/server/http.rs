#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum HttpVersion {
    Http10,
    Http11,
    Http20,
    Http30,
}

impl HttpVersion {
    pub fn as_str(&self) -> &str {
        match self {
            HttpVersion::Http10 => "HTTP/1.0",
            HttpVersion::Http11 => "HTTP/1.1",
            HttpVersion::Http20 => "HTTP/2.0",
            HttpVersion::Http30 => "HTTP/3.0",
        }
    }

    pub fn from_str(version_str: &str) -> Option<HttpVersion> {
        match version_str {
            "1.0" => Some(HttpVersion::Http10),
            "1.1" => Some(HttpVersion::Http11),
            "2.0" => Some(HttpVersion::Http20),
            "3.0" => Some(HttpVersion::Http30),
            _ => None, // none for unsupported version
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum HttpStatus {
    // Informational responses (100–199)
    Continue = 100,
    SwitchingProtocols = 101,
    Processing = 102,
    EarlyHints = 103,

    // Successful responses (200–299)
    Ok = 200,
    Created = 201,
    Accepted = 202,
    NonAuthoritativeInformation = 203,
    NoContent = 204,
    ResetContent = 205,
    PartialContent = 206,
    MultiStatus = 207,
    AlreadyReported = 208,
    IMUsed = 226,

    // Redirection messages (300–399)
    MovedPermanently = 301,
    Found = 302,
    SeeOther = 303,
    NotModified = 304,
    UseProxy = 305,
    TemporaryRedirect = 307,
    PermanentRedirect = 308,

    // Client error responses (400–499)
    BadRequest = 400,
    Unauthorized = 401,
    PaymentRequired = 402,
    Forbidden = 403,
    NotFound = 404,
    MethodNotAllowed = 405,
    NotAcceptable = 406,
    ProxyAuthenticationRequired = 407,
    RequestTimeout = 408,
    Conflict = 409,
    Gone = 410,
    LengthRequired = 411,
    PreconditionFailed = 412,
    PayloadTooLarge = 413,
    URITooLong = 414,
    UnsupportedMediaType = 415,
    RangeNotSatisfiable = 416,
    ExpectationFailed = 417,
    ImATeapot = 418,
    MisdirectedRequest = 421,
    UnprocessableEntity = 422,
    Locked = 423,
    FailedDependency = 424,
    TooEarly = 425,
    UpgradeRequired = 426,
    PreconditionRequired = 428,
    TooManyRequests = 429,
    RequestHeaderFieldsTooLarge = 431,
    UnavailableForLegalReasons = 451,

    // Server error responses (500–599)
    InternalServerError = 500,
    NotImplemented = 501,
    BadGateway = 502,
    ServiceUnavailable = 503,
    GatewayTimeout = 504,
    HTTPVersionNotSupported = 505,
    VariantAlsoNegotiates = 506,
    InsufficientStorage = 507,
    LoopDetected = 508,
    NotExtended = 510,
    NetworkAuthenticationRequired = 511,
}

impl HttpStatus {
    pub fn to_code(&self) -> u16 {
        *self as u16
    }

    pub fn to_message(&self) -> &str {
        match self {
            // Informational responses (100–199)
            HttpStatus::Continue => "Continue",
            HttpStatus::SwitchingProtocols => "Switching Protocols",
            HttpStatus::Processing => "Processing",
            HttpStatus::EarlyHints => "Early Hints",

            // Successful responses (200–299)
            HttpStatus::Ok => "OK",
            HttpStatus::Created => "Created",
            HttpStatus::Accepted => "Accepted",
            HttpStatus::NonAuthoritativeInformation => "Non-Authoritative Information",
            HttpStatus::NoContent => "No Content",
            HttpStatus::ResetContent => "Reset Content",
            HttpStatus::PartialContent => "Partial Content",
            HttpStatus::MultiStatus => "Multi-Status",
            HttpStatus::AlreadyReported => "Already Reported",
            HttpStatus::IMUsed => "IM Used",

            // Redirection messages (300–399)
            HttpStatus::MovedPermanently => "Moved Permanently",
            HttpStatus::Found => "Found",
            HttpStatus::SeeOther => "See Other",
            HttpStatus::NotModified => "Not Modified",
            HttpStatus::UseProxy => "Use Proxy",
            HttpStatus::TemporaryRedirect => "Temporary Redirect",
            HttpStatus::PermanentRedirect => "Permanent Redirect",

            // Client error responses (400–499)
            HttpStatus::BadRequest => "Bad Request",
            HttpStatus::Unauthorized => "Unauthorized",
            HttpStatus::PaymentRequired => "Payment Required",
            HttpStatus::Forbidden => "Forbidden",
            HttpStatus::NotFound => "Not Found",
            HttpStatus::MethodNotAllowed => "Method Not Allowed",
            HttpStatus::NotAcceptable => "Not Acceptable",
            HttpStatus::ProxyAuthenticationRequired => "Proxy Authentication Required",
            HttpStatus::RequestTimeout => "Request Timeout",
            HttpStatus::Conflict => "Conflict",
            HttpStatus::Gone => "Gone",
            HttpStatus::LengthRequired => "Length Required",
            HttpStatus::PreconditionFailed => "Precondition Failed",
            HttpStatus::PayloadTooLarge => "Payload Too Large",
            HttpStatus::URITooLong => "URI Too Long",
            HttpStatus::UnsupportedMediaType => "Unsupported Media Type",
            HttpStatus::RangeNotSatisfiable => "Range Not Satisfiable",
            HttpStatus::ExpectationFailed => "Expectation Failed",
            HttpStatus::ImATeapot => "I'm a teapot",
            HttpStatus::MisdirectedRequest => "Misdirected Request",
            HttpStatus::UnprocessableEntity => "Unprocessable Entity",
            HttpStatus::Locked => "Locked",
            HttpStatus::FailedDependency => "Failed Dependency",
            HttpStatus::TooEarly => "Too Early",
            HttpStatus::UpgradeRequired => "Upgrade Required",
            HttpStatus::PreconditionRequired => "Precondition Required",
            HttpStatus::TooManyRequests => "Too Many Requests",
            HttpStatus::RequestHeaderFieldsTooLarge => "Request Header Fields Too Large",
            HttpStatus::UnavailableForLegalReasons => "Unavailable For Legal Reasons",

            // Server error responses (500–599)
            HttpStatus::InternalServerError => "Internal Server Error",
            HttpStatus::NotImplemented => "Not Implemented",
            HttpStatus::BadGateway => "Bad Gateway",
            HttpStatus::ServiceUnavailable => "Service Unavailable",
            HttpStatus::GatewayTimeout => "Gateway Timeout",
            HttpStatus::HTTPVersionNotSupported => "HTTP Version Not Supported",
            HttpStatus::VariantAlsoNegotiates => "Variant Also Negotiates",
            HttpStatus::InsufficientStorage => "Insufficient Storage",
            HttpStatus::LoopDetected => "Loop Detected",
            HttpStatus::NotExtended => "Not Extended",
            HttpStatus::NetworkAuthenticationRequired => "Network Authentication Required",
        }
    }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub enum HttpMethod {
    GET,
    HEAD,
    POST,
    PUT,
    DELETE,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH,
}

impl HttpMethod {
    pub fn as_str(&self) -> &str {
        match self {
            Self::GET => "GET",
            Self::HEAD => "HEAD",
            Self::POST => "POST",
            Self::PUT => "PUT",
            Self::DELETE => "DELETE",
            Self::CONNECT => "CONNECT",
            Self::OPTIONS => "OPTIONS",
            Self::TRACE => "TRACE",
            Self::PATCH => "PATCH",
        }
    }

    pub fn from_str(method_str: &str) -> Option<Self> {
        match method_str {
            "GET" => Some(Self::GET),
            "HEAD" => Some(Self::HEAD),
            "POST" => Some(Self::POST),
            "PUT" => Some(Self::PUT),
            "DELETE" => Some(Self::DELETE),
            "CONNECT" => Some(Self::CONNECT),
            "OPTIONS" => Some(Self::OPTIONS),
            "TRACE" => Some(Self::TRACE),
            "PATCH" => Some(Self::PATCH),
            _ => None, // none for unsupported version
        }
    }

    pub fn to_vec(&self) -> Vec<Self> {
        vec![
            Self::GET,
            Self::HEAD,
            Self::POST,
            Self::PUT,
            Self::DELETE,
            Self::CONNECT,
            Self::OPTIONS,
            Self::TRACE,
            Self::PATCH,
        ]
    }

    pub fn immutable() -> Vec<Self> {
        vec![Self::GET, Self::HEAD, Self::OPTIONS, Self::TRACE]
    }

    pub fn mutable() -> Vec<Self> {
        vec![
            Self::POST,
            Self::PUT,
            Self::DELETE,
            Self::PATCH,
            Self::CONNECT,
        ]
    }

    pub fn comma_separated(methods: &[HttpMethod]) -> String {
        methods
            .iter()
            .map(|m| m.as_str())
            .collect::<Vec<_>>()
            .join(", ")
    }
}
