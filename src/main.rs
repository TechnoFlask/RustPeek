use rust_peek;
use std::{env, process};

fn main() {
    let cmd_args = env::args().collect::<Vec<String>>();
    let piped_data = rust_peek::piped_input().unwrap_or_else(|err| {
        eprintln!("Error while piping. {}", err);
        process::exit(1);
    });

    let cmd_line_data = rust_peek::parse_args(cmd_args, piped_data).unwrap_or_else(|e| {
        eprintln!("Error occurred: {}", e);
        // eprintln!("Usage: io_project <search string> <file path>");
        process::exit(1);
    });

    if let Err(e) = rust_peek::run(cmd_line_data) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
