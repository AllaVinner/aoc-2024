use crate::days;

pub fn solve(input: &str, day: i32, part: i32) -> Result<String, String> {
    match day {
        1 => days::day_01::solve(input, part),
        2 => days::day_02::solve(input, part),
        3 => days::day_03::solve(input, part),
        4 => days::day_04::solve(input, part),
        5 => days::day_05::solve(input, part),
        6 => days::day_06::solve(input, part),
        i => Err(format!("day {i} is not implemented.")),
    }
}
