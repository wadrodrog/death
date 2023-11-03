pub mod date;
pub mod user;
pub mod cli;

pub const DEFAULT_DEATH_REASONS: &'static [&'static str] = &[
    "cars", "illness", "height", "darkness", "fire", "water", "nature",
    "building", "electricity", "explosions", "food", "animals", "temperature",
    "weapons"];

/// Returns death reasons from file.
///
/// # Errors
///
/// Returns an error if the file was failed to read
pub fn read_death_reasons(_file_path: &String) -> Vec<&'static str> {
    // TODO: read the file

    /* let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    for line in contents.split("\n") {

    }
    println!("With text:\n{contents}"); */
    DEFAULT_DEATH_REASONS.to_vec()
}
