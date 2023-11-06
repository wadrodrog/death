use std::fs;
use std::path::PathBuf;
use std::io::{Error, ErrorKind};

pub mod date;
pub mod user;
pub mod cli;

/// Returns default death reasons.
pub fn default_death_reasons() -> Vec<String> {
    vec![
        "cars".to_string(), "illness".to_string(), "height".to_string(),
        "darkness".to_string(), "fire".to_string(), "water".to_string(),
        "nature".to_string(), "construction".to_string(),
        "electricity".to_string(), "explosions".to_string(),
        "food".to_string(), "animals".to_string(), "temperature".to_string(),
        "weapons".to_string()
    ]
}

/// Returns death reasons from file. Each reason is on separate lines. Lines
/// are trimmed of leading and trailing spaces.
///
/// If [`None`] was passed, a default death reasons returned.
///
/// # Errors
///
/// Returns [`std::io::Error`] if cannot read the file.
pub fn read_death_reasons(file_path: &Option<PathBuf>)
-> Result<Vec<String>, Error> {
    let file_path = match file_path {
        Some(x) => x,
        None => return Ok(default_death_reasons()),
    };

    let contents = fs::read_to_string(file_path)?;

    let mut res = vec![];

    for line in contents.lines() {
        let line = line.trim().to_string();
        if line.len() > 0 {
            res.push(line);
        }
    }

    if res.len() == 0 {
        Err(Error::new(ErrorKind::Other, "File is empty"))
    } else {
        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Error, ErrorKind};

    #[test]
    fn read_file() {
        // Generic file with different characters, empty lines and spaces
        let reference = vec![
            "sample1".to_string(), "sample2".to_string(),
            "sample3;".to_string(), "123".to_string(), "!@#$%^&*()".to_string(),
            "``".to_string(), "Sample With Spaces".to_string(),
            "It   Should  Trim Spaces".to_string(),
        ];

        let sample = read_death_reasons(&Some(PathBuf::from("tests/read_file.txt")))
            .unwrap();

        assert_eq!(sample, reference);

        // Empty file and file with spaces
        let error_ref = Error::new(ErrorKind::Other, "File is empty");

        match read_death_reasons(&Some(PathBuf::from("tests/spaces.txt"))) {
            Ok(_) => panic!("It is not an error as expected"),
            Err(e) => {
                assert_eq!(e.kind(), error_ref.kind());
                assert_eq!(e.to_string(), error_ref.to_string());
            },
        }

        match read_death_reasons(&Some(PathBuf::from("tests/empty.txt"))) {
            Ok(_) => panic!("It is not an error as expected"),
            Err(e) => {
                assert_eq!(e.kind(), error_ref.kind());
                assert_eq!(e.to_string(), error_ref.to_string());
            }
        }
    }
}