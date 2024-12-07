use crate::days;

pub fn solve(input: &str, day: i32, part: i32) -> Result<String, String> {
    match day {
        1 => days::day_01::solve(input, part),
        i => Err(format!("day {i} is not implemented.")),
    }
}
