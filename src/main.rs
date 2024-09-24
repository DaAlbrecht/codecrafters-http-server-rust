use std::io::Write;
#[allow(unused_imports)]
use std::net::TcpListener;

use codecrafters_http_server::{HttpResponse, HttpVersion, StatusCode, StatusLine};

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    //
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    //
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let response = HttpResponse {
                    status_line: StatusLine {
                        version: HttpVersion::HTTP_1_1,
                        status_code: StatusCode::OK,
                        reason_phrase: None,
                    },
                    headers: None,
                    body: None,
                };
                write!(stream, "{}", response).unwrap();
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
