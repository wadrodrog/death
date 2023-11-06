use std::io::{self, Write};
use std::process;

use death::cli;
use death::date::{Date, ParseError};
use death::user::User;

fn input() -> String {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    s.trim().parse().unwrap()
}

fn main() {
    let args = cli::parse();
    ask_birthday(&args, true);

    let name = ask_name(&args);
    let age = ask_birthday(&args, false);

    let death_reasons = match death::read_death_reasons(&args.death_reasons) {
        Ok(v) => v,
        Err(e) => {
            println!("error: {}", e);
            process::exit(1);
        },
    };

    let user = User::new(User::get_id_from_string(&name), age, death_reasons);

    println!("");

    predict(&user);
}

fn ask_name(args: &cli::Cli) -> String {
    if let Some(name) = args.name.as_deref() {
        name.to_string()
    } else {
        print!("Your name: ");
        let _ = io::stdout().flush();
        input()
    }
}

fn parse_birthday(string: &String) -> Result<Date, String> {
    let today = Date::today();
    let birthday = match Date::parse(string) {
        Ok(d) => d,
        Err(e) => {
            let msg = match e {
                ParseError::SeparatorNotFound =>
                    "Use '/', or '.', or '-', or whitespace \
                    as separator between day, month and year.",
                ParseError::InvalidPartsCount =>
                    "Only a day, month and year should be entered.",
                ParseError::NumberConversionError => "Invalid number.",
                ParseError::InvalidYear => "Invalid year.",
                ParseError::InvalidMonth => "Invalid month.",
                ParseError::InvalidDay => "Invalid day.",
            };
            return Err(String::from(msg));
        }
    };
    if today < birthday {
        return Err(String::from("Your birthday can not be in the future."));
    }
    Ok(birthday)
}

fn ask_birthday(args: &cli::Cli, test: bool) -> u8 {
    let birthday;
    if let Some(birthday_string) = args.birthday.as_deref() {
        birthday = match parse_birthday(&birthday_string.to_string()) {
            Ok(bday) => bday,
            Err(msg) => {
                println!("error: {}", msg);
                process::exit(1);
            }
        };
        if test {
            return birthday.years_from(Date::today()) as u8;
        }
    } else {
        loop {
            print!("Enter your birthday (DD/MM/YYYY): ");
            let _ = io::stdout().flush();
            birthday = match parse_birthday(&input()) {
                Ok(bday) => bday,
                Err(msg) => {
                    println!("error: {}", msg);
                    continue;
                }
            };
            break;
        }
    }
    birthday.years_from(Date::today()) as u8
}

fn predict(user: &User) {
    println!("DATE OF DEATH");
    println!("{}", user.get_death_date());
    println!("Be aware of: {}", user.get_death_reason());
}
