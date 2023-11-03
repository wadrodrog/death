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
    pub fn new(
        id: u64, age: u8, death_reasons: Option<Vec<&'static str>>
    ) -> User {
        User {
            id,
            age,
            death_reasons: match death_reasons {
                Some(v) => v,
                None => crate::DEFAULT_DEATH_REASONS.to_vec(),
            },
        }
    }
    
    /// Get an id from string's hash.
    pub fn get_id_from_string(string: &String) -> u64 {
        let mut s = DefaultHasher::new();
        string.hash(&mut s);
        s.finish()
    }

    fn get_years_left(&self) -> u8 {
        let max_age: u64 = (date::MAX_YEARS_OLD - self.age as u16) as u64;
        (self.id % max_age + 1) as u8
    }

    /// Returns user's predicted death reason.
    pub fn get_death_reason(&self) -> &'static str {
        self.death_reasons[
            (self.id % (self.death_reasons.len() as u64)) as usize
        ]
    }

    /// Returns calculated death date of current user.
    pub fn get_death_date(&self) -> Date {
        let year = Date::today().year() + self.get_years_left() as u16;
        let month = (self.id % 12 + 1) as u8;
        let day = 1;
        let date0 = Date::build(year, month, day).unwrap();
        let day = (self.id % date0.get_max_day() as u64 + 1) as u8;
        let date = Date::build(year, month, day).unwrap();

        date
    }
}
