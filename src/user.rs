use crate::date::{Date, self};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Debug)]
pub struct User {
    id: u64,
    age: u8,
    death_reasons: Vec<&'static str>,
}

impl User {
    /// Returns a new user.
    pub fn new(id: u64, age: u8, death_reasons: Vec<&'static str>) -> User {
        User {
            id,
            age,
            death_reasons,
        }
    }
    
    /// Set an id from string's hash.
    pub fn get_id_from_string(string: &String) -> u64 {
        let mut s = DefaultHasher::new();
        string.hash(&mut s);
        s.finish()
    }

    /// Set an age from birthday.
    pub fn set_age_from_birthday(&mut self, birthday: Date) {
        self.age = birthday.years_from(Date::today()) as u8;
    }

    fn get_years_left(&self) -> u8 {
        let max_age: u64 = (date::MAX_YEARS_OLD - self.age as u16) as u64;
        (self.id % max_age + 1) as u8
    }

    /// Returns user's predicted danger
    pub fn get_death_reason(&self) -> &'static str {
        self.death_reasons[
            (self.id % (self.death_reasons.len() as u64)) as usize
        ]
    }

    /// Returns calculated death date of current user.
    pub fn get_death_date(&self) -> Date {
        let year = Date::today().year() + self.get_years_left() as u16;
        let month = (self.id % 12 + 1) as u16;
        let day = 1;
        let date0 = Date::build(year, month, day).unwrap();
        let day = (self.id % date0.get_max_day() as u64 + 1) as u16;
        let date = Date::build(year, month, day).unwrap();

        date
    }
}

mod tests { }


/// Returns death reasons from file. If `file_path` is None, a default death
/// reasons returned.
///
/// Errors
///
/// This function will return an error if path does not exist or user does not
/// have permission to read the file
pub fn load_custom_death_reasons(file_path: Option<String>) -> Vec<&'static str> {
    match file_path {
        Some(_file_path) => {
            // TODO: read the file

            /* let contents = fs::read_to_string(file_path)
                .expect("Should have been able to read the file");

            for line in contents.split("\n") {

            }
            println!("With text:\n{contents}"); */

            vec![]
        },
        // TODO: embed in executable https://doc.rust-lang.org/std/macro.include_bytes.html
        None => vec![
            "cars", "illness", "height", "darkness", "fire", "water", "nature",
            "building", "electricity", "explosions", "food", "animals",
            "temperature", "weapons"
        ]
    }
}
