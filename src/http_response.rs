use std::collections::HashMap;
use std::string::ToString;

pub const CONTENT_LENGTH_KEY: &str = "Content-Length";
pub const CONTENT_TYPE_KEY: &str = "Content-Type";
pub const USER_AGENT_KEY: &str = "User-Agent";

pub const PLAIN_TEXT_CONTENT_TYPE: &str = "text/plain";
pub const FILE_CONTENT_TYPE: &str = "application/octet-stream";

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum HttpResponseStatus {
    Ok,
    BadRequest,
    Unauthorized,
    Forbidden,
    NotFound,
    InternalServerError,
    NotImplemented,
    BadGateway,
    ServiceUnavailable,
}

impl HttpResponseStatus {
    pub fn code(&self) -> u16 {
        match self {
            Self::Ok => 200,
            Self::BadRequest => 400,
            Self::Unauthorized => 401,
            Self::Forbidden => 403,
            Self::NotFound => 404,
            Self::InternalServerError => 500,
            Self::NotImplemented => 501,
            Self::BadGateway => 502,
            Self::ServiceUnavailable => 503,
        }
    }

    pub fn message(&self) -> &'static str {
        match self {
            Self::Ok => "OK",
            Self::BadRequest => "Bad Request",
            Self::Unauthorized => "Unauthorized",
            Self::Forbidden => "Forbidden",
            Self::NotFound => "Not Found",
            Self::InternalServerError => "Internal Server Error",
            Self::NotImplemented => "Not Implemented",
            Self::BadGateway => "Bad Gateway",
            Self::ServiceUnavailable => "Service Unavailable",
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct HttpResponse {
    status_code: HttpResponseStatus,
    headers: HashMap<String, String>,
    body: String, // maybe this all should be str
}

impl HttpResponse {
    pub fn new(
        http_response_status: HttpResponseStatus,
        headers: Option<HashMap<String, String>>,
        body: Option<String>,
        content_type: Option<String>,
    ) -> HttpResponse {
        match body {
            None => HttpResponse {
                status_code: http_response_status,
                headers: headers.unwrap_or_default(),
                body: "".to_string(),
            },
            Some(body) => {
                let mut resp = HttpResponse {
                    status_code: http_response_status,
                    headers: headers.unwrap_or_default(),
                    body,
                };
                resp.update_content_length();
                resp.update_content_type(
                    content_type.unwrap_or(PLAIN_TEXT_CONTENT_TYPE.to_string()),
                );
                resp
            }
        }
    }
    pub fn _status_code(&self) -> &HttpResponseStatus {
        &self.status_code
    }

    pub fn _headers(&self) -> &HashMap<String, String> {
        &self.headers
    }

    pub fn _body(&self) -> &str {
        &self.body
    }
    pub fn _set_status_code(&mut self, status_code: HttpResponseStatus) {
        self.status_code = status_code;
    }
    pub fn _add_header(&mut self, key: String, value: String) {
        // TODO prevent content length from being added
        self.headers.insert(key, value);
    }
    pub fn _set_body(&mut self, body: String, content_type: String) {
        // content type should be enum
        self.body = body;
        self.update_content_length();
        self.headers
            .insert(CONTENT_TYPE_KEY.to_string(), content_type);
    }
    fn update_content_length(&mut self) {
        self.headers
            .insert(CONTENT_LENGTH_KEY.to_string(), self.body.len().to_string());
    }
    fn update_content_type(&mut self, content_type: String) {
        //TODO switch to enum later
        self.headers
            .insert(CONTENT_TYPE_KEY.to_string(), content_type);
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let headers: String = self
            .headers
            .iter()
            .map(|(key, value)| format!("{}: {}\r\n", key, value))
            .collect();
        let response = format!(
            "HTTP/1.1 {} {}\r\n{}\r\n{}",
            self.status_code.code(),
            self.status_code.message(),
            headers,
            self.body,
        )
        .into_bytes();
        response
    }
}
