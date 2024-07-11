use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct HttpRequest<'a> {
    // I wanted this struct to provide reference to already existing request stored in string
    // So it has its lifetime
    pub method: &'a str,
    pub uri: Vec<&'a str>,
    pub version: &'a str,
    pub headers: HashMap<&'a str, &'a str>,
    pub body: &'a str,
}

#[derive(Debug)]
pub enum HttpRequestError {
    MissingRequestLine,
    MissingMethod,
    MissingPath,
    MissingVersion,
    InvalidHeader,
}

impl<'a> HttpRequest<'a> {
    pub fn from(request: &'a str) -> Result<HttpRequest<'a>, HttpRequestError> {
        let mut lines = request.lines();

        // Parse request line
        let request_line = lines.next().ok_or(HttpRequestError::MissingRequestLine)?;
        let mut parts = request_line.split(" ");
        let method = parts.next().ok_or(HttpRequestError::MissingMethod)?;
        // I kinda gave up on uri :C
        let uri: Vec<&str> = parts
            .next()
            .ok_or(HttpRequestError::MissingPath)?
            .split_inclusive("/")
            .collect();
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
                    .trim(),
                header_parts
                    .next()
                    .ok_or(HttpRequestError::InvalidHeader)?
                    .trim(),
            );
        }
        // Parse body
        let body: &str = lines.next().unwrap_or_else(|| ""); // should be safe but assert will catch
        assert_eq!(lines.next(), None);

        Ok(HttpRequest {
            method,
            uri,
            version,
            headers,
            body,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut headers = HashMap::new();
        headers.insert("User-Agent", "curl/7.64.1");
        headers.insert("Host", "localhost:4221");
        headers.insert("Accept", "*/*");
        let desired_request = HttpRequest {
            method: "GET",
            uri: vec!["/", "index.html"],
            version: "HTTP/1.1",
            headers,
            body: "body",
        };

        let request = "GET /index.html HTTP/1.1\r\nHost: localhost:4221\r\nUser-Agent: curl/7.64.1\r\nAccept: */*\r\n\r\nbody";
        let parsed_request = HttpRequest::from(request);
        match parsed_request {
            Ok(request) => {
                println!("{:#?}", request);
                assert_eq!(request, desired_request);
            }
            Err(e) => eprintln!("Failed to parse HTTP request: {:?}", e),
        }
    }

    #[test]
    fn test2() {
        let mut headers = HashMap::new();
        headers.insert("User-Agent", "curl/7.64.1");
        headers.insert("Host", "localhost:4221");
        headers.insert("Accept", "*/*");
        let desired_request = HttpRequest {
            method: "GET",
            uri: vec!["/", "books/", "abab"],
            version: "HTTP/1.1",
            headers,
            body: "body",
        };

        let request = "GET /books/abab HTTP/1.1\r\nHost: localhost:4221\r\nUser-Agent: curl/7.64.1\r\nAccept: */*\r\n\r\nbody";
        let parsed_request = HttpRequest::from(request);
        match parsed_request {
            Ok(request) => {
                println!("{:#?}", request);
                assert_eq!(request, desired_request);
            }
            Err(e) => eprintln!("Failed to parse HTTP request: {:?}", e),
        }
    }
}
