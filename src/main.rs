use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use pulldown_cmark::{Parser, html};
use tera::{Tera, Context};
use tiny_http::{Server, Response};
use std::io::Read;
use clap::{Command, Arg};

fn main() {
    let matches = Command::new("Devlift SSG")
        .arg(Arg::new("command")
            .required(true)
            .index(1)
            .help("Command: 'build' or 'serve'"))
        .get_matches();

    match matches.get_one::<String>("command").map(|s| s.as_str()) {
        Some("build") => {
            println!("Building site...");
            build_site();
        }
        Some("serve") => {
            println!("Starting server...");
            start_server();
        }
        _ => println!("Invalid command. Use 'build' or 'serve'."),
    }
}

fn build_site() {
    let resources_dir = Path::new("resources");
    let output_dir = Path::new("static");
    let mut posts = Vec::new();

    fs::create_dir_all(output_dir).unwrap();

    for entry in fs::read_dir(resources_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.extension().and_then(|ext| ext.to_str()) == Some("md") {
            let markdown = fs::read_to_string(&path).unwrap();
            let title = path.file_stem().unwrap().to_str().unwrap().to_owned();
            let html_content = markdown_to_html(&markdown);
            let html_output = markdown_to_html_with_template(&title, &html_content);

            let output_filename = format!("{}.html", title);
            let output_path = output_dir.join(&output_filename);
            let mut file = File::create(output_path).unwrap();
            file.write_all(html_output.as_bytes()).unwrap();

            posts.push((title.clone(), output_filename));
        }
    }

    generate_index_page(&posts);
}

fn markdown_to_html(markdown: &str) -> String {
    let parser = Parser::new(markdown);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}

fn markdown_to_html_with_template(title: &str, markdown: &str) -> String {
    let tera = Tera::new("templates/*.html").unwrap();
    let mut context = Context::new();
    context.insert("title", title);
    context.insert("content", &markdown_to_html(markdown));

    tera.render("base.html", &context).unwrap()
}

fn generate_index_page(posts: &Vec<(String, String)>) {
    let tera = Tera::new("templates/*.html").unwrap();
    let mut context = Context::new();
    
    // Convert posts into a format that Tera can iterate over
    let posts_data: Vec<_> = posts.iter()
        .map(|(title, link)| {
            let mut map = std::collections::HashMap::new();
            map.insert("title".to_string(), title.clone());
            map.insert("link".to_string(), link.clone());
            map
        })
        .collect();
    
    context.insert("posts", &posts_data);

    let index_html = tera.render("index.html", &context).unwrap();
    let mut file = File::create("static/index.html").unwrap();
    file.write_all(index_html.as_bytes()).unwrap();
}

fn get_content_type(path: &str) -> &'static str {
    match Path::new(path).extension().and_then(|ext| ext.to_str()) {
        Some("html") => "text/html",
        Some("css") => "text/css",
        Some("js") => "application/javascript",
        _ => "text/plain",
    }
}

fn start_server() {
    let server = Server::http("127.0.0.1:8080").unwrap();
    println!("Serving on http://127.0.0.1:8080");

    for request in server.incoming_requests() {
        let url = request.url();
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