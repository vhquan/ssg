use pulldown_cmark::{Parser, html};
use tera::{Tera, Context};
// use std::fs;

use crate::models::PostMetadata;

pub fn extract_metadata(markdown: &str) -> Option<PostMetadata> {
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

pub fn markdown_to_html(markdown: &str) -> String {
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

pub fn markdown_to_html_with_template(markdown: &str, metadata: Option<&PostMetadata>) -> String {
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
