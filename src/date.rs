use chrono::{Local, Datelike};
use std::{cmp, fmt};

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
    month: u8,
    day: u8,
}

pub const MAX_YEARS_OLD: u16 = 100;

impl Date {
    /// Makes a new `Date` from the today's date.
    pub fn today() -> Date {
        let dt = Local::now().date_naive();
        Date {
            year: dt.year() as u16,
            month: dt.month() as u8,
            day: dt.day() as u8,
        }
    }

    /// Makes `Date` from a year, a month and a day.
    ///
    /// # Errors
    ///
    /// Returns date::ParseError if values are invalid
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
    pub fn build(year: u16, month: u8, day: u8) -> Result<Date, ParseError> {
        if year == 0 {
            return Err(ParseError::InvalidYear);
        }

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
    /// Returns date::ParseError if the string contains invalid date or date in the
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

        Date::build(numbers[2], numbers[1] as u8, numbers[0] as u8)
    }

    /// Returns `true` if the year is the leap year.
    ///
    /// # Example
    /// ```
    /// use death::date::Date;
    ///
    /// assert!(Date::is_leap_year(2016));
    /// assert!(!Date::is_leap_year(2015));
    /// assert!(Date::is_leap_year(2000));
    /// assert!(!Date::is_leap_year(1900));
    /// ```
    pub fn is_leap_year(year: u16) -> bool {
        year % 4 == 0 && year % 100 != 0 || year % 400 == 0
    }

    /// Returns `true` if the current year is the leap year.
    ///
    /// # Example
    /// ```
    /// use death::date::Date;
    ///
    /// assert!(Date::build(2016, 3, 7).unwrap().leap_year());
    /// assert!(!Date::build(2015, 3, 7).unwrap().leap_year());
    /// assert!(Date::build(2000, 3, 7).unwrap().leap_year());
    /// assert!(!Date::build(1900, 3, 7).unwrap().leap_year());
    /// ````
    pub fn leap_year(&self) -> bool {
        Date::is_leap_year(self.year)
    }

    /// Returns the max day of month in year.
    ///
    /// # Example
    /// ```
    /// use death::date::Date;
    ///
    /// assert_eq!(Date::max_day_of(2015, 3), 31);
    /// assert_eq!(Date::max_day_of(2015, 4), 30);
    /// assert_eq!(Date::max_day_of(2015, 2), 28);
    /// assert_eq!(Date::max_day_of(2016, 2), 29);
    /// ```
    pub fn max_day_of(year: u16, month: u8) -> u8 {
        let a = vec![1, 3, 5, 7, 8, 10, 12];
        
        if month == 2 {
            if Date::is_leap_year(year) {
                return 29;
            }
            return 28;
        }

        if a.contains(&month) {
            return 31;
        }
        30
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
    /// ```
    pub fn get_max_day(&self) -> u8 {
        Date::max_day_of(self.year, self.month)
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
    ///     Date::build(2016, 2, 28).unwrap().next_day(),
    ///     Date::build(2016, 2, 29).unwrap()
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
    pub fn month(&self) -> u8 {
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
    pub fn day(&self) -> u8 {
        self.day
    }
}

impl fmt::Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.day(), self.get_month_name(), self.year())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_date() {
        assert_eq!(Date::build(0, 1, 1), Err(ParseError::InvalidYear));
        assert_eq!(Date::build(2015, 0, 0), Err(ParseError::InvalidMonth));
        assert_eq!(Date::build(2015, 13, 1), Err(ParseError::InvalidMonth));
        assert_eq!(Date::build(2015, 1, 32), Err(ParseError::InvalidDay));
        assert_eq!(Date::build(2015, 1, 0), Err(ParseError::InvalidDay));
        assert_eq!(Date::build(2015, 2, 29), Err(ParseError::InvalidDay));
        assert_eq!(Date::build(2016, 2, 30), Err(ParseError::InvalidDay));
    }

    #[test]
    fn parse_date() {
        // Success
        assert_eq!(
            Date::parse(&String::from("23.10.2015")), Date::build(2015, 10, 23)
        );
        assert_eq!(
            Date::parse(&String::from("23/10/2015")), Date::build(2015, 10, 23)
        );
        assert_eq!(
            Date::parse(&String::from("23-10-2015")), Date::build(2015, 10, 23)
        );
        assert_eq!(
            Date::parse(&String::from("23 10 2015")), Date::build(2015, 10, 23)
        );

        // Fail
        assert_eq!(
            Date::parse(&String::from("23\\09\\2015")),
            Err(ParseError::SeparatorNotFound)
        );
        assert_eq!(
            Date::parse(&String::from("23_09_2015")),
            Err(ParseError::SeparatorNotFound)
        );
        assert_eq!(
            Date::parse(&String::from("23092015")),
            Err(ParseError::SeparatorNotFound)
        );
        assert_eq!(
            Date::parse(&String::from("23.09.20.15")),
            Err(ParseError::InvalidPartsCount)
        );
        assert_eq!(
            Date::parse(&String::from("qwerty/asdfg.zxcvb")),
            Err(ParseError::NumberConversionError)
        );
        assert_eq!(
            Date::parse(&String::from("23.092015")),
            Err(ParseError::NumberConversionError)
        );
        assert_eq!(
            Date::parse(&String::from("20/10/-2015")),
            Err(ParseError::NumberConversionError)
        );
        assert_eq!(
            Date::parse(&String::from("32/10/2015")),
            Err(ParseError::InvalidDay)
        );
        assert_eq!(
            Date::parse(&String::from("20/13/2015")),
            Err(ParseError::InvalidMonth)
        );
        assert_eq!(
            Date::parse(&String::from("29/2/2015")),
            Err(ParseError::InvalidDay)
        );
        assert_eq!(
            Date::parse(&String::from("1/2/0")),
            Err(ParseError::InvalidYear)
        );
    }

    #[test]
    fn max_day() {
        assert_eq!(Date::build(2015, 1, 1).unwrap().get_max_day(), 31);
        assert_eq!(Date::build(2015, 2, 1).unwrap().get_max_day(), 28);
        assert_eq!(Date::build(2016, 2, 1).unwrap().get_max_day(), 29);
        assert_eq!(Date::build(2015, 3, 1).unwrap().get_max_day(), 31);
        assert_eq!(Date::build(2015, 4, 1).unwrap().get_max_day(), 30);
        assert_eq!(Date::build(2015, 5, 1).unwrap().get_max_day(), 31);
        assert_eq!(Date::build(2015, 6, 1).unwrap().get_max_day(), 30);
        assert_eq!(Date::build(2015, 7, 1).unwrap().get_max_day(), 31);
        assert_eq!(Date::build(2015, 8, 1).unwrap().get_max_day(), 31);
        assert_eq!(Date::build(2015, 9, 1).unwrap().get_max_day(), 30);
        assert_eq!(Date::build(2015, 10, 1).unwrap().get_max_day(), 31);
        assert_eq!(Date::build(2015, 11, 1).unwrap().get_max_day(), 30);
        assert_eq!(Date::build(2015, 12, 1).unwrap().get_max_day(), 31);

        assert_eq!(Date::max_day_of(2015, 1), 31);
        assert_eq!(Date::max_day_of(2015, 2), 28);
        assert_eq!(Date::max_day_of(2016, 2), 29);
        assert_eq!(Date::max_day_of(2015, 3), 31);
        assert_eq!(Date::max_day_of(2015, 4), 30);
        assert_eq!(Date::max_day_of(2015, 5), 31);
        assert_eq!(Date::max_day_of(2015, 6), 30);
        assert_eq!(Date::max_day_of(2015, 7), 31);
        assert_eq!(Date::max_day_of(2015, 8), 31);
        assert_eq!(Date::max_day_of(2015, 9), 30);
        assert_eq!(Date::max_day_of(2015, 10), 31);
        assert_eq!(Date::max_day_of(2015, 11), 30);
        assert_eq!(Date::max_day_of(2015, 12), 31);
    }
}
