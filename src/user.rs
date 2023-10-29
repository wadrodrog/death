use crate::date::{Date, self};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Debug)]
pub struct User {
    birthday: Date,
    name: String,
}

impl User {
    /// Return a new empty user.
    ///
    /// # Example
    /// ```
    /// use death::user::User;
    /// use death::date::Date;
    ///
    /// let user = User::new();
    ///
    /// assert_eq!(user.birthday(), Date::build(1970, 1, 1).unwrap());
    /// assert_eq!(user.name(), &String::from("None"));
    /// ```
    pub fn new() -> User {
        User {
            birthday: Date::build(1970, 1, 1).unwrap(),
            name: String::from("None")
        }
    }
    
    /// Set a birthday for user.
    ///
    /// # Example
    /// ```
    /// use death::user::User;
    /// use death::date::Date;
    ///
    /// let mut user = User::new();
    /// let birthday = Date::build(1990, 11, 12).unwrap();
    /// user.set_birthday(birthday);
    ///
    /// assert_eq!(user.birthday(), birthday);
    /// ```
    pub fn set_birthday(&mut self, birthday: Date) {
        self.birthday = birthday;
    }

    /// Set a name for user.
    ///
    /// # Example
    /// ```
    /// use death::user::User;
    ///
    /// let mut user = User::new();
    /// let name = String::from("John Doe");
    /// user.set_name(name);
    ///
    /// assert_eq!(user.name(), &String::from("John Doe"));
    /// ```
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    /// Returns user's years old
    ///
    /// # Example
    /// ```
    /// use death::user::User;
    /// use death::date::Date;
    ///
    /// let mut user = User::new();
    /// let birthday = Date::build(1990, 11, 12).unwrap();
    /// user.set_birthday(birthday);
    ///
    /// assert_eq!(user.years_old(), birthday.years_from(Date::today()));
    /// ```
    pub fn years_old(&self) -> u16 {
        self.birthday().years_from(Date::today())
    }

    /// Returns user's birthday
    ///
    /// # Example
    /// ```
    /// use death::user::User;
    /// use death::date::Date;
    ///
    /// let user = User::new();
    /// 
    /// assert_eq!(user.birthday(), Date::build(1970, 1, 1).unwrap());
    /// ```
    pub fn birthday(&self) -> Date {
        self.birthday
    }

    /// Returns user's name
    ///
    /// # Example
    /// ```
    /// use death::user::User;
    ///
    /// let user = User::new();
    /// 
    /// assert_eq!(user.name(), &String::from("None"));
    /// ```
    pub fn name(&self) -> &String {
        &self.name
    }

    fn calculate_hash(&self) -> u16 {
        let mut s = DefaultHasher::new();
        self.name.hash(&mut s);
        s.finish() as u16
    }

    fn get_years_left(&self) -> u16 {
        let max_left = date::MAX_YEARS_OLD - self.years_old();
        self.calculate_hash() % max_left + 1
    }

    /// Returns user's predicted danger
    pub fn get_danger(&self) -> &'static str {
        let dangers = vec![
            "cars",
            "illness",
            "height",
            "darkness",
            "fire",
            "water",
            "nature",
            "building",
            "electricity",
            "explosions",
            "food",
            "animals",
            "temperature",
            "weapons",
        ];
        dangers[(self.calculate_hash() % (dangers.len() as u16)) as usize]
    }

    /// Returns calculated death date of current user.
    pub fn get_death_date(&self) -> Date {
        let hash = self.calculate_hash();

        let year = Date::today().year() + self.get_years_left();
        let month = hash % 12 + 1;
        let day = 1;
        let date0 = Date::build(year, month, day).unwrap();
        let day = hash % date0.get_max_day() + 1;
        let date = Date::build(year, month, day).unwrap();

        date
    }
}

mod tests {
    #[test]
    fn calculate_hash() {
        use super::*;
        
        let user1 = User::new();
        let user2 = User::new();
        let mut user3 = User::new();

        user3.set_name(String::from("null"));

        assert_eq!(user1.calculate_hash(), user2.calculate_hash());
        assert_ne!(user1.calculate_hash(), user3.calculate_hash());
    }
}
