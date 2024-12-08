use derive_more::derive::Display;
use derive_more::From;
use nom;
use nom::bytes::complete::tag;
use regex::Regex;

type VerboseNomResult<'a, O> = nom::IResult<&'a str, O, nom::error::VerboseError<&'a str>>;

#[derive(Debug, From, Display)]
pub enum AoCError<'a> {
    #[from]
    // Parsing(nom::Err<NomError<&'a str>>),
    Parsing(nom::Err<nom::error::VerboseError<&'a str>>),
}

fn nom_parser(input: &str) -> VerboseNomResult<(i32, i32)> {
    nom::sequence::delimited(
        tag("mul("),
        nom::sequence::separated_pair(
            nom::character::complete::i32,
            tag(","),
            nom::character::complete::i32,
        ),
        tag(")"),
    )(input)
}

fn part1(input: &str) -> Result<String, AoCError> {
    let pattern = Regex::new(r"(mul\([0-9]{1,3},[0-9]{1,3}\))").unwrap();
    Ok(pattern
        .find_iter(input)
        .map(|r| r.as_str())
        .map(|s| nom_parser(s))
        .map(|r| r.expect("Expect complete parsing"))
        .map(|(_, (v1, v2))| v1 * v2)
        .sum::<i32>()
        .to_string())
}

fn part2(input: &str) -> Result<String, AoCError> {
    let mul_pattern = Regex::new(r"(mul\([0-9]{1,3},[0-9]{1,3}\))").unwrap();
    let do_pattern = Regex::new(r"(do\(\))").unwrap();
    let mut do_iter = do_pattern.find_iter(input);
    let mut mul_iter = mul_pattern.find_iter(input);
    let no_pattern = Regex::new(r"(don't\(\))").unwrap();
    let mut no_iter = no_pattern.find_iter(input);

    let mut mul_match = mul_iter.next();
    let mut do_match = do_iter.next();
    let mut no_match = no_iter.next();

    dbg!(&do_match);
    dbg!(&no_match);
    let mut is_enabled = true;

    let mut sum = 0;
    loop {
        let mul = match mul_match {
            Some(mul) => mul,
            None => break,
        };
        let do_start = match do_match {
            Some(m) => m.start(),
            None => input.len(),
        };
        let no_start = match no_match {
            Some(m) => m.start(),
            None => input.len(),
        };
        if mul.start() < do_start && mul.start() < no_start {
            if is_enabled {
                dbg!(&mul_match);
                let (_, (v1, v2)) = nom_parser(mul.as_str()).expect("should word");
                sum += v1 * v2;
            }
            mul_match = mul_iter.next();
        } else if do_start < mul.start() && do_start < no_start {
            dbg!(&do_match);
            is_enabled = true;
            do_match = do_iter.next();
        } else if no_start < mul.start() && no_start < do_start {
            dbg!(&no_match);
            is_enabled = false;
            no_match = no_iter.next();
        }
    }
    Ok(sum.to_string())
}

pub fn solve(input: &str, part: i32) -> Result<String, String> {
    let res = match part {
        1 => part1(input),
        2 => part2(input),
        i => return Err(format!("day 3 part {i} is not implemented")),
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

    const TEST_INPUT: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn test_part1() {
        let result = part1(TEST_INPUT);
        assert_eq!(result.is_ok(), true);
        assert_eq!(result.unwrap(), "161")
    }

    #[test]
    fn test_part2() {
        let result = part2(TEST_INPUT);
        assert_eq!(result.is_ok(), true);
        assert_eq!(result.unwrap(), "48")
    }
}
