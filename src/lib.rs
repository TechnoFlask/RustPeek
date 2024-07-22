use std::{
    env,
    error::Error,
    fs,
    io::{self, Read},
};

use atty::Stream;

#[derive(Debug)]
pub enum Case {
    Sensitive,
    Insensitive,
}

#[derive(Debug)]
pub struct Args {
    search_string: String,
    data: Data,
    case: Case,
}

impl Args {
    fn new(args: (String, Data, Case)) -> Self {
        Self {
            search_string: args.0,
            data: args.1,
            case: args.2,
        }
    }
}

#[derive(Debug)]
pub enum Data {
    File(String),
    Pipe(String),
}

pub fn piped_input() -> Result<Option<String>, Box<dyn Error>> {
    if !atty::is(Stream::Stdin) {
        let stdin = io::stdin();
        let mut reader = stdin.lock();

        let mut buffer = Vec::new();
        reader.read_to_end(&mut buffer)?;
        return Ok(Some(String::from_utf8_lossy(&buffer[..]).to_string()));
    }
    Ok(None)
}

fn validate_args(piped_data: &Option<String>, cmd_args: &Vec<String>) -> Result<(), &'static str> {
    if piped_data.is_some() {
        if cmd_args.len() != 1 {
            return Err(
                "Incorrect number of arguments. 1 needed.\nUsage: <binary> <search string>",
            );
        }
    } else if cmd_args.len() != 2 {
        return Err(
            "Incorrect number of arguments. 2 needed.\nUsage: <binary> <search string> <file path>",
        );
    }

    Ok(())
}

pub fn parse_args(
    mut cmd_args: Vec<String>,
    piped_data: Option<String>,
) -> Result<Args, &'static str> {
    cmd_args.remove(0);

    validate_args(&piped_data, &cmd_args)?;

    let mut it = cmd_args.into_iter();
    let search_string = it.next().unwrap();
    let case = if env::var("IGNORE_CASE").is_ok() {
        Case::Insensitive
    } else {
        Case::Sensitive
    };

    if piped_data.is_some() {
        return Ok(Args::new((
            search_string,
            Data::Pipe(piped_data.unwrap()),
            case,
        )));
    } else {
        Ok(Args::new((
            search_string,
            Data::File(it.next().unwrap()),
            case,
        )))
    }
}

pub fn search<'a>(search_str: &str, data: &'a str, mode: Case) -> Vec<&'a str> {
    let mut result = Vec::<&'a str>::new();

    data.lines().for_each(|line| {
        if let Case::Insensitive = mode {
            if line.to_lowercase().contains(&search_str.to_lowercase()) {
                result.push(line);
            }
        } else {
            if line.contains(search_str) {
                result.push(line);
            }
        }
    });

    result
}

pub fn run(cmd_args: Args) -> Result<(), Box<dyn Error>> {
    let contents = match cmd_args.data {
        Data::Pipe(piped_data) => piped_data,
        Data::File(file_path) => fs::read_to_string(&file_path)?,
    };

    let mode = cmd_args.case;

    let search_results = search(&cmd_args.search_string, &contents, mode);

    if search_results.len() == 0 {
        println!("No matches found.");
        return Ok(());
    }

    for (i, res) in search_results.into_iter().enumerate() {
        println!("Match {}: {}", i + 1, res);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_case_sensitive() {
        let s_query = "great";
        let s_space = "Hi\nDo you know?\nRust is great\nIt is efficient\nFast and really safe";

        assert_eq!(
            search(s_query, s_space, Case::Sensitive),
            vec!["Rust is great"]
        );
    }

    #[test]
    fn search_case_insensitive() {
        let s_query = "iS";
        let s_space = "Hi\nDo you know?\nRust is great\nIt is efficient\nFast and really safe";

        assert_eq!(
            search(s_query, s_space, Case::Insensitive),
            vec!["Rust is great", "It is efficient"]
        );
    }
}
