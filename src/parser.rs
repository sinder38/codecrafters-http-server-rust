use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct HttpRequest {
    pub method: String,
    pub target: String,
    pub version: String, //TODO maybe it should be an integer
    pub headers: HashMap<String, String>,
    pub body: String,
}

#[derive(Debug)]
pub enum HttpRequestError {
    MissingRequestLine,
    MissingMethod,
    MissingPath,
    MissingVersion,
    InvalidHeader,
}

impl HttpRequest {
    pub fn parse<T>(request: T) -> Result<HttpRequest, HttpRequestError>
    where
        T: AsRef<str>,
    {
        let mut lines = request.as_ref().lines();

        // Parse request line
        let request_line = lines.next().ok_or(HttpRequestError::MissingRequestLine)?;
        let mut parts = request_line.split(" ");
        let method = parts.next().ok_or(HttpRequestError::MissingMethod)?;
        let target = parts.next().ok_or(HttpRequestError::MissingPath)?;
        let version = parts.next().ok_or(HttpRequestError::MissingVersion)?;

        // Parse headers
        let mut headers = HashMap::new();
        for line in &mut lines {
            let line = line.trim();
            if line.is_empty() {
                break;
            }
            let mut header_parts = line.splitn(2, ':');
            headers.insert(
                header_parts
                    .next()
                    .ok_or(HttpRequestError::InvalidHeader)?
                    .trim()
                    .to_string(),
                header_parts
                    .next()
                    .ok_or(HttpRequestError::InvalidHeader)?
                    .trim()
                    .to_string(),
            );
        }

        // Parse body
        let body: String = lines.collect();

        Ok(HttpRequest {
            method: method.to_string(),
            target: target.to_string(),
            version: version.to_string(),
            headers,
            body,
        })
    }
}

#[test]
fn test1() {
    let mut headers = HashMap::new();
    headers.insert("User-Agent".to_string(), "curl/7.64.1".to_string());
    headers.insert("Host".to_string(), "localhost:4221".to_string());
    headers.insert("Accept".to_string(), "*/*".to_string());
    let desired_request = HttpRequest {
        method: "GET".to_string(),
        target: "/index.html".to_string(),
        version: "HTTP/1.1".to_string(),
        headers,
        body: "body".to_string(),
    };

    let request = "GET /index.html HTTP/1.1\r\nHost: localhost:4221\r\nUser-Agent: curl/7.64.1\r\nAccept: */*\r\n\r\nbody";
    let parsed_request = HttpRequest::parse(request);
    match parsed_request {
        Ok(request) => {
            println!("{:#?}", request);
            assert_eq!(request, desired_request);
        }
        Err(e) => eprintln!("Failed to parse HTTP request: {:?}", e),
    }
}
