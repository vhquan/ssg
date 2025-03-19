use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use pulldown_cmark::{Parser, html};
use tera::{Tera, Context};
use tiny_http::{Server, Response};
use std::io::Read;
use clap::{Command, Arg};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct PostMetadata {
    title: String,
    date: String,
    author: String,
    tags: Vec<String>,
    summary: String,
}

fn extract_metadata(markdown: &str) -> Option<PostMetadata> {
    if markdown.starts_with("---") {
        let parts: Vec<&str> = markdown.splitn(3, "---").collect();
        if parts.len() > 2 {
            let yaml_str = parts[1].trim();
            match serde_yaml::from_str(yaml_str) {
                Ok(metadata) => Some(metadata),
                Err(e) => {
                    eprintln!("Error parsing metadata: {}", e);
                    None
                }
            }
        } else {
            None
        }
    } else {
        None
    }
}

fn copy_directory(src: &Path, dst: &Path) -> std::io::Result<()> {
    if !dst.exists() {
        fs::create_dir_all(dst)?;
    }

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());

        if ty.is_dir() {
            copy_directory(&src_path, &dst_path)?;
        } else {
            fs::copy(src_path, dst_path)?;
        }
    }
    Ok(())
}

fn publish_site() -> std::io::Result<()> {
    // First build the site
    build_site();

    // Then copy the style directory
    let style_dir = Path::new("style");
    let static_style_dir = Path::new("static/style");

    println!("Copying style directory to static...");
    copy_directory(style_dir, static_style_dir)?;
    println!("Site published successfully!");
    Ok(())
}

fn main() {
    let matches = Command::new("Devlift SSG")
        .arg(Arg::new("command")
            .required(true)
            .index(1)
            .help("Command: 'build', 'serve', or 'publish'"))
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
        Some("publish") => {
            if let Err(e) = publish_site() {
                eprintln!("Error publishing site: {}", e);
            }
        }
        _ => println!("Invalid command. Use 'build', 'serve', or 'publish'."),
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
            let metadata = extract_metadata(&markdown);
            let html_output = markdown_to_html_with_template(&markdown, metadata.as_ref());

            let display_title = if let Some(meta) = &metadata {
                meta.title.clone()
            } else {
                path.file_stem().unwrap().to_str().unwrap().to_owned()
            };

            // Convert spaces to hyphens in the filename
            let safe_filename = display_title.replace(" ", "-");
            let output_filename = format!("{}.html", safe_filename);
            let output_path = output_dir.join(&output_filename);
            let mut file = File::create(output_path).unwrap();
            file.write_all(html_output.as_bytes()).unwrap();

            posts.push((display_title.clone(), safe_filename, output_filename));
        }
    }

    generate_index_page();
}

fn markdown_to_html(markdown: &str) -> String {
    // If the markdown starts with ---, extract only the content part
    let content = if markdown.starts_with("---") {
        let parts: Vec<&str> = markdown.splitn(3, "---").collect();
        if parts.len() > 2 { parts[2] } else { markdown }
    } else {
        markdown
    };

    let parser = Parser::new(content.trim());
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}

fn markdown_to_html_with_template(markdown: &str, metadata: Option<&PostMetadata>) -> String {
    let tera = Tera::new("templates/*.html").unwrap();
    let mut context = Context::new();
    
    if let Some(meta) = metadata {
        context.insert("title", &meta.title);
        context.insert("date", &meta.date);
        context.insert("author", &meta.author);
        context.insert("tags", &meta.tags);
        context.insert("summary", &meta.summary);
    }
    
    context.insert("content", &markdown_to_html(markdown));
    tera.render("base.html", &context).unwrap()
}

fn generate_index_page() {
    let tera = Tera::new("templates/*.html").unwrap();
    let mut context = Context::new();
    
    // Convert posts into a format that Tera can iterate over
    let mut posts_with_metadata: Vec<_> = Vec::new();
    let mut tags_map: std::collections::HashMap<String, Vec<std::collections::HashMap<String, String>>> = std::collections::HashMap::new();
    
    for entry in fs::read_dir("resources").unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        
        if path.extension().and_then(|ext| ext.to_str()) == Some("md") {
            let markdown = fs::read_to_string(&path).unwrap();
            if let Some(metadata) = extract_metadata(&markdown) {
                let title = metadata.title;
                let date = metadata.date;
                let safe_filename = title.replace(" ", "-");
                let link = format!("{}.html", safe_filename);
                
                let mut map = std::collections::HashMap::new();
                map.insert("title".to_string(), title.clone());
                map.insert("date".to_string(), date);
                map.insert("link".to_string(), link.clone());
                posts_with_metadata.push(map.clone());

                // Collect unique tags and map posts
                for tag in metadata.tags {
                    tags_map.entry(tag.clone()).or_insert_with(Vec::new).push(map.clone());
                }
            }
        }
    }
    
    // Sort posts by date in descending order
    posts_with_metadata.sort_by(|a, b| {
        b.get("date").unwrap().cmp(a.get("date").unwrap())
    });
    
    // Create tag links
    let tags: Vec<_> = tags_map.keys().map(|tag| {
        let mut map = std::collections::HashMap::new();
        map.insert("tag".to_string(), tag.clone());
        map.insert("link".to_string(), format!("tags/{}.html", tag.replace(" ", "-")));
        map
    }).collect();
    
    context.insert("posts", &posts_with_metadata);
    context.insert("tags", &tags);

    let index_html = tera.render("index.html", &context).unwrap();
    let mut file = File::create("static/index.html").unwrap();
    file.write_all(index_html.as_bytes()).unwrap();

    // Ensure tags directory exists
    fs::create_dir_all("static/tags").unwrap();

    // Generate tag pages
    for (tag, posts) in tags_map {
        let mut tag_context = Context::new();
        tag_context.insert("tag", &tag);
        tag_context.insert("posts", &posts);

        let tag_html = tera.render("tag.html", &tag_context).unwrap();
        let tag_filename = format!("static/tags/{}.html", tag.replace(" ", "-"));
        let mut file = File::create(tag_filename).unwrap();
        file.write_all(tag_html.as_bytes()).unwrap();
    }
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