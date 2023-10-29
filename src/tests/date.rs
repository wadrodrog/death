#[test]
fn build_date() {
    use crate::date::{Date, ParseError};
    assert_eq!(Date::build(2015, 0, 0), Err(ParseError::InvalidMonth));
    assert_eq!(Date::build(2015, 13, 1), Err(ParseError::InvalidMonth));
    assert_eq!(Date::build(2015, 1, 32), Err(ParseError::InvalidDay));
    assert_eq!(Date::build(2015, 2, 29), Err(ParseError::InvalidDay));
    assert_eq!(Date::build(2016, 2, 30), Err(ParseError::InvalidDay));
}

#[test]
fn parse_date() {
    use crate::date::{Date, ParseError};
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
}

#[test]
fn max_day() {
    use crate::date::Date;
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
}
