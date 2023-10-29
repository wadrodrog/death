use std::io::{self, Write};

use death;
use death::date::{Date, ParseError};
use death::user::User;

fn input() -> String {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    s.trim().parse().unwrap()
}

fn main() {
    let mut user = User::new();

    enter_birtday(&mut user);
    enter_name(&mut user);

    println!("");

    predict(&mut user);
}

fn enter_birtday(user: &mut User) {
    loop {
        print!("Enter your birthday (DD/MM/YYYY): ");
        let _ = io::stdout().flush();
        let today = Date::today();
        let birthday = match Date::parse(&input()) {
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
                println!("Error: {}", msg);
                continue;
            }
        };
        if today.year() < birthday.year() {
            println!("Your birthday can not be in the future.");
            continue;
        } else if today.year() == birthday.year() &&
            today.month() < birthday.month() {
            println!("Your birthday can not be in the future.");
            continue;
        } else if today.month() == birthday.month() &&
            today.day() < birthday.day() {
                println!("Your birthday can not be in the future.");
                continue;
            }
        user.set_birthday(birthday);
        break;
    }
}

fn enter_name(user: &mut User) {
    print!("Your name: ");
    let _ = io::stdout().flush();
    user.set_name(input());
}

fn predict(user: &mut User) {
    println!("DATE OF DEATH");
    println!("{}", user.get_death_date().get_string());
    println!("Be aware of: {}", user.get_danger());
}
