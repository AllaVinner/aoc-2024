use derive_more::derive::Display;
use derive_more::From;
use nom::bytes::complete::tag;
use nom::combinator::all_consuming;
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

fn nom_parser(input: &str) -> VerboseNomResult<Vec<(u64, Vec<u64>)>> {
    all_consuming(separated_list1(
        tag("\n"),
        separated_pair(
            nom::character::complete::u64,
            tag(": "),
            separated_list1(tag(" "), nom::character::complete::u64),
        ),
    ))(input)
}

#[derive(PartialEq, Eq, Debug, Display)]
enum Op {
    Add,
    Mul,
}

impl Op {
    fn vec_iter(num_ops: u32) -> OpIter {
        OpIter { i: 0, num_ops }
    }
}

struct OpIter {
    i: u64,
    num_ops: u32,
}

impl Iterator for OpIter {
    type Item = Vec<Op>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i >= u32::pow(2, self.num_ops) as u64 {
            return None;
        }
        let mut ops = Vec::new();
        for op_i in 0..self.num_ops {
            let op = match (self.i / u32::pow(2, op_i) as u64) % 2 {
                0 => Op::Add,
                1 => Op::Mul,
                r => panic!("unreachable "),
            };
            ops.push(op);
        }
        self.i += 1;
        return Some(ops);
    }
}

fn part1(input: &str) -> Result<String, AoCError> {
    let (_, equations) = nom_parser(input)?;
    let mut calibration = 0;
    for (target, numbers) in equations {
        for ops in Op::vec_iter((numbers.len() - 1) as u32) {
            let mut eq = String::new();
            eq.push_str(&target.to_string());
            eq.push_str(": ");
            let mut num_iter = numbers.iter();
            let mut res = match num_iter.next() {
                Some(first) => *first,
                None => continue,
            };
            eq.push_str(&res.to_string());
            for (op, num) in ops.iter().zip(num_iter) {
                let s = match op {
                    Op::Add => format!(" + {num}"),
                    Op::Mul => format!(" * {num}"),
                };
                eq.push_str(&s);
                res = match op {
                    Op::Add => res + num,
                    Op::Mul => res * num,
                };
            }
            if target == res {
                calibration += target;
                println!("{}", eq);
                break;
            }
        }
    }
    return Ok(calibration.to_string());
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
        assert_eq!(result.unwrap(), "3749")
    }

    #[test]
    fn test_part2() {
        let result = part2(TEST_INPUT);
        assert_eq!(result.is_ok(), true);
        assert_eq!(result.unwrap(), "31")
    }
}
#[test]
fn test_op_iter() {
    let mut op_iter = Op::vec_iter(3);
    assert_eq!(op_iter.next(), Some(vec![Op::Add, Op::Add, Op::Add]));
    assert_eq!(op_iter.next(), Some(vec![Op::Mul, Op::Add, Op::Add]));
    assert_eq!(op_iter.next(), Some(vec![Op::Add, Op::Mul, Op::Add]));
    assert_eq!(op_iter.next(), Some(vec![Op::Mul, Op::Mul, Op::Add]));
    assert_eq!(op_iter.count(), 2_usize.pow(3) - 4);
}
