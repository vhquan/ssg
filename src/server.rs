use std::fs::File;
use std::io::Read;
// use std::path::Path;
use tiny_http::{Server, Response};
use chrono::Local;

use crate::utils::get_content_type;

pub fn start_server() {
    let server = Server::http("0.0.0.0:8080").unwrap();
    println!("Server running on port 8080 (accessible from any IP)");
    println!("Server started at {}", Local::now().format("%Y-%m-%d %H:%M:%S"));
    println!("Press Ctrl+C to stop the server");

    for request in server.incoming_requests() {
        let url = request.url();
        let method = request.method();
        let remote_addr = request.remote_addr();
        println!("[{}] {} {} from {}", 
            Local::now().format("%Y-%m-%d %H:%M:%S"),
            method,
            url,
            remote_addr.map_or("unknown".to_string(), |addr| addr.to_string())
        );

        let path = if url == "/" { "static/index.html" } else { &format!("static{}", url) };

        if let Ok(mut file) = File::open(path) {
            let mut content = String::new();
            file.read_to_string(&mut content).unwrap();
            let response = Response::from_string(content)
                .with_header(tiny_http::Header {
                    field: "Content-Type".parse().unwrap(),
                    value: get_content_type(path).parse().unwrap(),
                });
            request.respond(response).unwrap();
        } else {
            let response = Response::from_string("404 Not Found").with_status_code(404);
            request.respond(response).unwrap();
        }
    }
}
