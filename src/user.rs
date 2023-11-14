use crate::date::{Date, self};
use crate::cli;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Debug)]
pub struct User {
    id: u64,
    age: u8,
    death_reasons: Vec<String>,
}

impl User {
    /// Returns a new user.
    pub fn new(id: u64, age: u8, death_reasons: Vec<String>) -> User {
        User { id, age, death_reasons }
    }

    /// Returns a new user from command-line arguments.
    ///
    /// If some argument was empty, default value will be used.
    ///
    /// # Errors
    ///
    /// If some argument is invalid, program will close immediately.
    pub fn from(args: &cli::Cli) -> User {
        let empty = String::from("01/01/1970");
        let birthday_string = match &args.birthday {
            Some(v) => v,
            None => &empty,
        };
        let birthday = cli::parse_birthday(&birthday_string);
        let age = match birthday {
            Ok(v) => v.years_from(Date::today()) as u8,
            Err(e) => {
                cli::print_error(e, 1);
                0
            }
        };

        let death_reasons = match crate::read_death_reasons(
            &args.death_reasons
        ) {
            Ok(v) => v,
            Err(e) => {
                cli::print_error(e, 1);
                vec![]
            },
        };

        let id = User::get_id_from_string(
            &args.name.as_deref().unwrap_or("").to_string()
        );

        User { id, age, death_reasons }
    }

    /// Set an id for user.
    pub fn set_id(&mut self, id: u64) {
        self.id = id;
    }

    /// Set an age for user.
    pub fn set_age(&mut self, age: u8) {
        self.age = age;
    }
    
    /// Get an id from string's hash.
    pub fn get_id_from_string(string: &String) -> u64 {
        let mut s = DefaultHasher::new();
        string.hash(&mut s);
        s.finish()
    }

    fn get_years_left(&self, linear: bool) -> u8 {
        if linear {
            let max_age: u64 = (date::MAX_AGE - self.age as u16) as u64;
            return (self.id % max_age + 1) as u8;
        }

        // Returns smaller values more often than larger values

        // Max y
        let max_age: f64 = (date::MAX_AGE - self.age as u16) as f64;

        // Stretch the graph horizontally to make the result more accurate
        let k: f64 = 100.0;

        // base^0=1, base^x_max=max_age
        let base: f64 = max_age.powf(1.0 / (max_age * k));

        // From 0 to max_age * k - 1
        let x: f64 = (self.id % (max_age * k) as u64) as f64;

        // f(x)=a^x
        base.powf(x) as u8
    }

    /// Returns user's predicted death reason.
    pub fn get_death_reason(&self) -> &String {
        &self.death_reasons[
            (self.id % (self.death_reasons.len() as u64)) as usize
        ]
    }

    /// Returns calculated death date of current user.
    pub fn get_death_date(&self, linear: bool) -> Date {
        let year = Date::today().year() + self.get_years_left(linear) as u16;
        let month = (self.id % 12 + 1) as u8;
        let day = 1;
        let date0 = Date::build(year, month, day).unwrap();
        let day = (self.id % date0.get_max_day() as u64 + 1) as u8;
        let date = Date::build(year, month, day).unwrap();

        date
    }
}
