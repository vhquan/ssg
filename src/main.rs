use clap::{Command, Arg};

// Import from our library crate
use ssg::{build_site, publish_site, start_server};

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