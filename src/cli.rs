use std::path::PathBuf;
use std::{fmt, process};
use std::io::{self, Write};

use crate::date::{Date, ParseError};

use clap::Parser;
use colored::*;

/// A program that predicts your death date
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Your name
    #[arg(short, long)]
    pub name: Option<String>,

    /// Your birthday
    #[arg(short, long)]
    pub birthday: Option<String>,

    /// Custom death reasons file
    #[arg(short, long, value_name = "FILE")]
    pub death_reasons: Option<PathBuf>,
}

/// Parse command-line arguments.
pub fn parse() -> Cli {
    Cli::parse()
}

/// Print error to stderr.
///
/// If `code` is not `0`, program will close with this code.
pub fn print_error<T: fmt::Display>(error: T, code: i32) {
    eprintln!("{} {}", "error:".red(), error);
    if code != 0 {
        process::exit(code);
    }
}

/// Read string from console input.
pub fn input() -> String {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    s.trim().parse().unwrap()
}

/// Ask user something in the same line as input.
pub fn prompt(msg: &str) -> String {
    print!("{}: ", msg);
    let _ = io::stdout().flush();
    input()
}

/// Ask user's name.
pub fn ask_name() -> String {
    prompt("Your name")
}

/// Parse birthday from string.
///
/// # Errors
///
/// Returns a string containing the reason why parsing was failed.
pub fn parse_birthday(string: &String) -> Result<Date, String> {
    let today = Date::today();
    let birthday = match Date::parse(string) {
        Ok(d) => d,
        Err(e) => {
            let msg = match e {
                ParseError::SeparatorNotFound =>
                    "Use '/', or '.', or '-', or whitespace \
                    as separator between day, month and year.",
                ParseError::InvalidPartsCount =>
                    "Invalid should be DD/MM/YYYY - day, month and year.",
                ParseError::NumberConversionError => "Invalid number.",
                ParseError::InvalidYear => "Invalid year.",
                ParseError::InvalidMonth => "Invalid month.",
                ParseError::InvalidDay => "Invalid day.",
            };
            return Err(String::from(msg));
        }
    };
    if today < birthday {
        return Err(String::from("Your birthday cannot be in the future."));
    }
    Ok(birthday)
}

/// Ask user's birthday
pub fn ask_birthday() -> u8 {
    let birthday;
    loop {
        let inp = prompt("Enter your birthday (DD/MM/YYYY)");
        let _ = io::stdout().flush();
        birthday = match parse_birthday(&inp) {
            Ok(bday) => bday,
            Err(e) => {
                print_error(e, 0);
                continue;
            }
        };
        break;
    }
    birthday.years_from(Date::today()) as u8
}
