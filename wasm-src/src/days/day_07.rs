use derive_more::derive::Display;
use derive_more::From;
use nom::bytes::complete::tag;
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::{self, character};

type VerboseNomResult<'a, O> = nom::IResult<&'a str, O, nom::error::VerboseError<&'a str>>;

#[derive(Debug, From, Display)]
pub enum AoCError<'a> {
    #[from]
    // Parsing(nom::Err<NomError<&'a str>>),
    Parsing(nom::Err<nom::error::VerboseError<&'a str>>),
}

fn nom_parser(input: &str) -> VerboseNomResult<Vec<(i32, Vec<i32>)>> {
    separated_list1(
        tag("\n"),
        separated_pair(
            nom::character::complete::i32,
            tag(": "),
            separated_list1(tag(" "), nom::character::complete::i32),
        ),
    )(input)
}

enum Op {
    Add,
    Mul,
}

fn part1(input: &str) -> Result<String, AoCError> {
    let (_, equations) = nom_parser(input)?;
    for (target, numbers) in equations {}
    return Ok("Part 1 Placeholder".to_string());
}

fn part2(input: &str) -> Result<String, AoCError> {
    return Ok("Part 1 Placeholder".to_string());
}

pub fn solve(input: &str, part: i32) -> Result<String, String> {
    let res = match part {
        1 => part1(input),
        2 => part2(input),
        i => return Err(format!("day 1 part {i} is not implemented")),
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
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn test_part1() {
        let result = part1(TEST_INPUT);
        assert_eq!(result.is_ok(), true);
        assert_eq!(result.unwrap(), "11")
    }

    #[test]
    fn test_part2() {
        let result = part2(TEST_INPUT);
        assert_eq!(result.is_ok(), true);
        assert_eq!(result.unwrap(), "31")
    }
}
