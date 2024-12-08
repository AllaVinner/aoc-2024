use derive_more::derive::Display;
use derive_more::From;
use itertools::Itertools;
use nom;
use nom::bytes::complete::tag;

type VerboseNomResult<'a, O> = nom::IResult<&'a str, O, nom::error::VerboseError<&'a str>>;
type Level = i32;
type Report = Vec<Level>;

#[derive(Debug, From, Display)]
pub enum AoCError<'a> {
    #[from]
    Parsing(nom::Err<nom::error::VerboseError<&'a str>>),
}

fn nom_parser(input: &str) -> VerboseNomResult<Vec<Report>> {
    nom::combinator::all_consuming(nom::sequence::terminated(
        nom::multi::separated_list1(
            nom::character::complete::newline,
            nom::multi::separated_list1(
                nom::character::complete::space1,
                nom::character::complete::i32,
            ),
        ),
        nom::branch::alt((tag("\n\n"), tag("\n"), tag(""))),
    ))(input)
}

fn is_safe(report: &[Level]) -> bool {
    let diffs: Vec<i32> = report
        .iter()
        .tuple_windows()
        .map(|(v1, v2)| v2 - v1)
        .collect();
    if !(diffs.iter().all(|diff| *diff > 0) || diffs.iter().all(|diff| *diff < 0)) {
        return false;
    }
    if diffs.iter().all(|diff| 1 <= diff.abs() && diff.abs() <= 3) {
        return true;
    }
    return false;
}

fn part1(input: &str) -> Result<String, AoCError> {
    let (_, reports) = nom_parser(input)?;
    let num_safe = reports.iter().filter(|r| is_safe(r)).count();
    return Ok(num_safe.to_string());
}

fn is_safe_2(report: &[Level]) -> bool {
    for skip_i in 0..report.len() {
        let diffs: Vec<i32> = report
            .iter()
            .enumerate()
            .filter(|(i, _)| *i != skip_i)
            .map(|(_, v)| v)
            .tuple_windows()
            .map(|(v1, v2)| v2 - v1)
            .collect();
        if diffs.iter().all(|diff| *diff > 0) || diffs.iter().all(|diff| *diff < 0) {
            if diffs.iter().all(|diff| 1 <= diff.abs() && diff.abs() <= 3) {
                return true;
            }
        }
    }
    return false;
}

fn part2(input: &str) -> Result<String, AoCError> {
    let (_, reports) = nom_parser(input)?;
    let num_safe = reports.iter().filter(|r| is_safe_2(r)).count();
    return Ok(num_safe.to_string());
}

pub fn solve(input: &str, part: i32) -> Result<String, String> {
    let res = match part {
        1 => part1(input),
        2 => part2(input),
        i => return Err(format!("day 2 part {i} is not implemented")),
    };
    match res {
        Ok(v) => Ok(v),
        Err(AoCError::Parsing(e)) => match e {
            nom::Err::Error(ve) => Err(nom::error::convert_error(input, ve)),
            nom::Err::Failure(ve) => Err(nom::error::convert_error(input, ve)),
            nom::Err::Incomplete(_) => Err("input is imcomplete".to_string()),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const TEST_INPUT: &str = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";

    #[test]
    fn test_part1() {
        let result = part1(TEST_INPUT);
        dbg!(&result);
        assert_eq!(result.is_ok(), true);
        assert_eq!(result.unwrap(), "2")
    }

    #[test]
    fn test_part2() {
        let result = part2(TEST_INPUT);
        assert_eq!(result.is_ok(), true);
        assert_eq!(result.unwrap(), "4")
    }
}
