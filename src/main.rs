mod content_formatter;
mod file_walker;

use clap::{App, Arg};
use std::path::Path;
use tokio::fs;

#[tokio::main]
async fn main() {
    let matches = App::new("Code Parser")
        .version("0.1.0")
        .author("charslee013 <charslee013@gmail.com>")
        .about("Parses code files based on specified languages and formats output.")
        .arg(
            Arg::new("language")
                .short('l')
                .long("language")
                .takes_value(true)
                .help("Specifies the programming language file extension to parse"),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .takes_value(true)
                .help("Optional output file to store the results"),
        )
        .arg(
            Arg::new("with-test")
                .long("with-test")
                .takes_value(false)
                .help("Includes files and folders containing 'test' in their names"),
        )
        .arg(
            Arg::new("path")
                .short('p')
                .long("path")
                .takes_value(true)
                .help("The root path to start scanning for code files"),
        )
        .get_matches();

    let language = matches.value_of("language").unwrap_or("py");
    let output_file = matches.value_of("output");
    let include_tests = matches.is_present("with-test");
    let root_path = matches.value_of("path").unwrap_or("."); // This captures the path from the CLI

    println!("Here is the program code in {}. Please understand the program and provide a brief explanation of the role and relationship between each component.", language);

    // Use the path provided by the user
    let paths = file_walker::walk_directory(Path::new(root_path), include_tests, language);
    for entry in paths {
        let content = content_formatter::read_file_content(entry.path()).await;
        match content {
            Ok(content) => {
                let formatted_content = content_formatter::format_content(&content, language);
                println!("\n{}\n{}", entry.path().display(), formatted_content);
                if let Some(output) = output_file {
                    fs::write(output, formatted_content.as_bytes())
                        .await
                        .unwrap();
                }
            }
            Err(e) => println!("Error reading file {:?}: {}", entry.path(), e),
        }
    }
}
