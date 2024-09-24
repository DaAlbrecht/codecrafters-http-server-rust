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
        write!(f, "{}", self.status_line)?;
        if let Some(headers) = &self.headers {
            for (key, value) in headers {
                write!(f, "{}: {}\r\n", key, value)?;
            }
        }
        write!(f, "\r\n")?;
        if let Some(body) = &self.body {
            write!(f, "{}", body)?;
        }
        Ok(())
    }
}

pub struct StatusLine {
    pub version: HttpVersion,
    pub status_code: StatusCode,
    pub reason_phrase: Option<String>,
}

impl Display for StatusLine {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let reason_phrase = match &self.reason_phrase {
            Some(reason_phrase) => reason_phrase,
            None => "",
        };
        write!(
            f,
            "{} {} {}\r\n",
            self.version, self.status_code, reason_phrase
        )
    }
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
