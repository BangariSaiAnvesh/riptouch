use clap::Parser;
use chrono::*;
use std::{io::{Write, Read}, fs::File};

#[derive(Parser)]
#[clap(version, about)]
#[derive(Default)]
/// A simple touch replacement that can add custom text in file.
struct Cli {
    #[clap(parse(from_os_str))]
    /// Path to the file including file name.
    path: std::path::PathBuf,
    #[clap(short, long)]
    /// To add text from config, use flag
    add: bool,
}

fn make_file() -> File {
    let mut app = match std::fs::File::create("rtrc") {
        Ok(file) => file,
        Err(_) => panic!("Failed to make file"),
    };
    app.write_all(b"What ever is below CREATOR MESSAGE will be added to the created file.\n\n").expect("Write failed.");
    app.write_all(b"[Creator Message]\n").expect("Write failed.");
    return app;
}

fn make_file_with_text(args: Cli) {
    let mut text_file = match std::fs::File::open("rtrc") {
        Ok(file) => file,
        Err(_) => make_file()
    };
    let mut file = std::fs::File::create(args.path).expect("Error making file");
    let mut contents = String::new();
    text_file.read_to_string(&mut contents).expect("Failed to read config file.");
    let mut state = false;
    for line in contents.lines() {
        if line == "[Language Comment]" { break; }
        if line.ends_with("Date: ") {
            file.write_all(format!("{}", line).as_bytes()).expect("Failed to write");
            file.write_all(format!("{}\n", Local::now().date()).as_bytes()).expect("Failed to write");
            continue;
        }
        if state {
            file.write_all(format!("{}\n", line).as_bytes()).expect("Failed to write");
            continue;
        }
        if line == "[Creator Message]" {
            state = true;
        }
    }
}

fn make_file_without_text(args: Cli) {
    std::fs::File::create(args.path).expect("Error making file");
}

fn main() {
    let args = Cli::parse();
    if args.add {
        make_file_with_text(args);
    } else {
        make_file_without_text(args);
    }
}