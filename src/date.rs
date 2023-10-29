use chrono::{Local, Datelike};
use std::cmp;

#[derive(Debug)]
#[derive(PartialEq)]
pub enum ParseError {
    SeparatorNotFound,
    InvalidPartsCount,
    NumberConversionError,
    InvalidYear,
    InvalidMonth,
    InvalidDay,
}

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Eq)]
#[derive(Ord)]
#[derive(Clone, Copy)]
pub struct Date {
    year: u16,
    month: u16,
    day: u16
}

pub const MAX_YEARS_OLD: u16 = 95;

impl Date {
    /// Makes a new `Date` from the today's date.
    pub fn today() -> Date {
        let dt = Local::now().date_naive();
        Date {
            year: dt.year() as u16,
            month: dt.month() as u16,
            day: dt.day() as u16,
        }
    }

    /// Makes `Date` from a year, month and day.
    ///
    /// # Errors
    ///
    /// Returns `ParseError` if the string contains invalid date or date in the
    /// future.
    ///
    /// # Example
    ///
    /// ```
    /// use death::date::Date;
    ///
    /// let date = Date::build(2023, 10, 27).unwrap();
    ///
    /// assert_eq!(date.year(), 2023);
    /// assert_eq!(date.month(), 10);
    /// assert_eq!(date.day(), 27);
    /// ```
    pub fn build(year: u16, month: u16, day: u16) -> Result<Date, ParseError> {
        if month < 1 || month > 12 {
            return Err(ParseError::InvalidMonth);
        }

        let mut date = Date { year, month, day: 1 };
        
        if day < 1 || day > date.get_max_day() {
            return Err(ParseError::InvalidDay);
        }

        date.day = day;
        
        Ok(date)
    }

    /// Parses `Date` from a string.
    ///
    /// # Errors
    ///
    /// Returns `ParseError` if the string contains invalid date or date in the
    /// future.
    ///
    /// # Example
    ///
    /// ```
    /// use death::date::Date;
    ///
    /// let s = String::from("27/10/2023");
    /// let date = Date::parse(&s);
    ///
    /// assert_eq!(Date::build(2023, 10, 27), date);
    /// ```
    pub fn parse(s: &String) -> Result<Date, ParseError> {
        // Find a separator
        let separators = vec!['.', '/', '-', ' '];
        let mut sep: Option<char> = None;
        for &separator in separators.iter() {
            if s.contains(separator) {
                sep = Some(separator);
                break;
            }
        }
        
        // Split into the parts
        let parts;

        if let Some(value) = sep {
            parts = s.split(value);
        } else {
            return Err(ParseError::SeparatorNotFound);
        }

        // Iterate parts
        let mut numbers = vec![];
        
        for part in parts {
            let n: u16 = match part.parse() {
                Ok(n) => n,
                Err(_) => return Err(ParseError::NumberConversionError),
            };
            numbers.push(n);
        }

        if numbers.len() != 3 {
            return Err(ParseError::InvalidPartsCount);
        }

        Date::build(numbers[2], numbers[1], numbers[0])
    }

    /// Returns `true` if the current year is the leap year.
    ///
    /// # Example
    /// ```
    /// use death::date::Date;
    ///
    /// assert!(Date::build(2016, 3, 7).unwrap().is_leap_year());
    /// assert!(!Date::build(2015, 3, 7).unwrap().is_leap_year());
    /// assert!(Date::build(2000, 3, 7).unwrap().is_leap_year());
    /// ````
    pub fn is_leap_year(&self) -> bool {
        self.year % 4 == 0 && self.year % 100 != 0 || self.year % 400 == 0
    }

    /// Returns the max day of current month.
    ///
    /// # Example
    /// ```
    /// use death::date::Date;
    ///
    /// assert_eq!(Date::build(2015, 3, 7).unwrap().get_max_day(), 31);
    /// assert_eq!(Date::build(2015, 4, 7).unwrap().get_max_day(), 30);
    /// assert_eq!(Date::build(2015, 2, 7).unwrap().get_max_day(), 28);
    /// assert_eq!(Date::build(2016, 2, 7).unwrap().get_max_day(), 29);
    /// ````
    pub fn get_max_day(&self) -> u16 {
        let a = vec![1, 3, 5, 7, 8, 10, 12];
        
        if self.month == 2 {
            if self.is_leap_year() {
                return 29;
            }
            return 28;
        }

        if a.contains(&self.month) {
            return 31;
        }
        30
    }

    /// Returns month name
    ///
    /// # Example
    /// ```
    /// use death::date::Date;
    ///
    /// let date = Date::build(2012, 12, 12).unwrap();
    ///
    /// assert_eq!(date.get_month_name(), String::from("December"));
    /// ```
    pub fn get_month_name(&self) -> &str {
        let months = vec![
            "January", "February", "March", "April", "May", "June", "July",
            "August", "September", "October", "November", "December"
        ];
        months[(self.month - 1) as usize]
    }
    
    /// Returns date string
    ///
    /// # Example
    /// ```
    /// use death::date::Date;
    ///
    /// let date = Date::build(2012, 2, 1).unwrap();
    /// 
    /// assert_eq!(date.get_string(), String::from("1 February 2012"));
    /// ```
    pub fn get_string(&self) -> String {
        format!(
            "{} {} {}", self.day(), self.get_month_name(), self.year()
        )
    }

    /// Returns the copy of `Date` with month number increased.
    ///
    /// If the day was greater than next month's max day, it will be decreased
    /// to max day.
    ///
    /// If month was `12`, the year will be increased and month set to `1`.
    ///
    /// # Example
    /// ```
    /// use death::date::Date;
    ///
    /// assert_eq!(
    ///     Date::build(2015, 3, 7).unwrap().next_month(),
    ///     Date::build(2015, 4, 7).unwrap()
    /// );
    /// assert_eq!(
    ///     Date::build(2015, 1, 31).unwrap().next_month(),
    ///     Date::build(2015, 2, 28).unwrap()
    /// );
    /// assert_eq!(
    ///     Date::build(2015, 12, 7).unwrap().next_month(),
    ///     Date::build(2016, 1, 7).unwrap()
    /// );
    /// ```
    pub fn next_month(&self) -> Date {
        let mut date = Date {
            year: self.year(), month: self.month(), day: self.day()
        };

        if date.month() == 12 {
            date.year += 1;
            date.month = 1;
        } else {
            date.month += 1;
        }

        date.day = date.day.clamp(1, date.get_max_day());

        date
    }

    /// Returns the copy of `Date` with day number increased.
    ///
    /// If the day was last in current month, month will be increased and day
    /// set to `1`. If month was `12`, the year will be increased with month
    /// and day set to `1`.
    ///
    /// # Example
    /// ```
    /// use death::date::Date;
    ///
    /// assert_eq!(
    ///     Date::build(2015, 3, 7).unwrap().next_day(),
    ///     Date::build(2015, 3, 8).unwrap()
    /// );
    /// assert_eq!(
    ///     Date::build(2015, 2, 28).unwrap().next_day(),
    ///     Date::build(2015, 3, 1).unwrap()
    /// );
    /// assert_eq!(
    ///     Date::build(2015, 12, 31).unwrap().next_day(),
    ///     Date::build(2016, 1, 1).unwrap()
    /// );
    /// ```
    pub fn next_day(&self) -> Date {
        let mut date = Date {
            year: self.year(), month: self.month(), day: self.day()
        };

        if date.day() == date.get_max_day() {
            if date.month() == 12 {
                date.year += 1;
                date.month = 1;
            } else {
                date.month += 1;
            }
            date.day = 1;
        } else {
            date.day += 1;
        }

        date
    }

    /// Returns a number of full years from the other date.
    ///
    /// # Example
    /// ```
    /// use death::date::Date;
    ///
    /// let a = Date::build(1998, 5, 12).unwrap();
    /// let b = Date::build(2015, 4, 2).unwrap();
    /// let c = Date::build(2015, 5, 13).unwrap();
    ///
    /// assert_eq!(a.years_from(b), 16);
    /// assert_eq!(a.years_from(c), 17);
    /// ```
    pub fn years_from(&self, other: Date) -> u16 {
        let left = cmp::min(self.clone(), other.clone());
        let right = cmp::max(self.clone(), other.clone());
        let mut diff = right.year() - left.year();

        if right.month() < left.month() ||
        right.month() == left.month() && right.day() < left.day() {
            diff -= 1;
        }

        diff
    }

    /// Returns the year number.
    ///
    /// # Example
    /// ```
    /// use death::date::Date;
    ///
    /// assert_eq!(Date::build(2015, 3, 7).unwrap().year(), 2015);
    /// ```
    pub fn year(&self) -> u16 {
        self.year
    }

    /// Returns the month number from 1 to 12.
    ///
    /// # Example
    /// ```
    /// use death::date::Date;
    ///
    /// assert_eq!(Date::build(2015, 3, 7).unwrap().month(), 3);
    /// ```
    pub fn month(&self) -> u16 {
        self.month
    }

    /// Returns the day number.
    ///
    /// # Example
    /// ```
    /// use death::date::Date;
    ///
    /// assert_eq!(Date::build(2015, 3, 7).unwrap().day(), 7);
    /// ```
    pub fn day(&self) -> u16 {
        self.day
    }
}
