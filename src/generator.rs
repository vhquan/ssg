use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use std::collections::HashMap;
use tera::{Tera, Context};

use crate::markdown::{extract_metadata, markdown_to_html_with_template};
use crate::utils::copy_directory;

pub fn build_site() {
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

            // Convert title to lowercase and spaces to underscores in the filename
            let safe_filename = display_title.to_lowercase().replace(" ", "_");
            let output_filename = format!("{}.html", safe_filename);
            let output_path = output_dir.join(&output_filename);
            let mut file = File::create(output_path).unwrap();
            file.write_all(html_output.as_bytes()).unwrap();

            posts.push((display_title.clone(), safe_filename, output_filename));
        }
    }

    generate_index_page();
}

pub fn generate_index_page() {
    let tera = Tera::new("templates/*.html").unwrap();
    let mut context = Context::new();
    
    // Convert posts into a format that Tera can iterate over
    let mut posts_with_metadata: Vec<_> = Vec::new();
    let mut tags_map: HashMap<String, Vec<HashMap<String, String>>> = HashMap::new();
    
    for entry in fs::read_dir("resources").unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        
        if path.extension().and_then(|ext| ext.to_str()) == Some("md") {
            let markdown = fs::read_to_string(&path).unwrap();
            if let Some(metadata) = extract_metadata(&markdown) {
                let title = metadata.title;
                let date = metadata.date;
                let safe_filename = title.to_lowercase().replace(" ", "_");
                let link = format!("{}.html", safe_filename);
                
                let mut map = HashMap::new();
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
        let mut map = HashMap::new();
        map.insert("tag".to_string(), tag.clone());
        map.insert("link".to_string(), format!("tags/{}.html", tag.to_lowercase().replace(" ", "_")));
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

pub fn publish_site() -> std::io::Result<()> {
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
