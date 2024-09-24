use std::{
    collections::HashMap,
    fmt::{Display, Formatter},
};

pub struct HttpResponse {
    pub status_line: StatusLine,
    pub headers: Option<HashMap<String, String>>,
    pub body: Option<String>,
}

impl Display for HttpResponse {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let mut response = String::new();
        response.push_str(&format!(
            "{} {}\r\n",
            self.status_line.version, self.status_line.status_code
        ));
        if let Some(headers) = &self.headers {
            for (key, value) in headers {
                response.push_str(&format!("{}: {}\r\n", key, value));
            }
        }
        response.push_str("\r\n");
        if let Some(body) = &self.body {
            response.push_str(body);
        }
        write!(f, "{}", response)
    }
}

pub struct StatusLine {
    pub version: HttpVersion,
    pub status_code: StatusCode,
    pub reason_phrase: Option<String>,
}

pub enum HttpVersion {
    HTTP_1_1,
}

impl Display for HttpVersion {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            HttpVersion::HTTP_1_1 => write!(f, "HTTP/1.1"),
        }
    }
}

pub enum StatusCode {
    OK,
}

impl Display for StatusCode {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            StatusCode::OK => write!(f, "200"),
        }
    }
}
