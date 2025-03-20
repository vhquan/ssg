# Devlift Static Site Generator

A modern, lightweight static site generator written in Rust. This tool helps you create beautiful, fast-loading websites with minimal configuration.

## Project Structure

- **src/**: Rust source code for the static site generator
- **style/**: CSS files for styling the website
- **templates/**: HTML templates using the Tera templating engine
- **static/**: Generated website output (HTML files)
- **resources/**: Source content for your website (markdown files, etc.)

## Features

- Fast generation using Rust
- Responsive design with clean typography
- Tag-based content organization
- Code syntax highlighting for multiple languages
- Markdown support for content creation

## Code Syntax Highlighting

This site generator includes built-in syntax highlighting for code blocks using Highlight.js. To use this feature in your content, wrap your code in markdown code blocks with language specification:

````markdown
```rust
fn main() {
    println!("Hello, world!");
}
```
````

Supported languages include:
- Rust
- C++
- Python
- JavaScript
- JSON
- And many more via Highlight.js

## Usage

To build your site:

```bash
cargo run -- build
```

To serve your site locally for development:

```bash
cargo run -- serve
```

## Customization

You can customize the appearance of your site by modifying the CSS files in the `style/` directory. The code highlighting theme can be changed by updating the Highlight.js theme in the base template.