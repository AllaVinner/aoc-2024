use derive_more::derive::Display;
use derive_more::From;
use nom;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{i32 as str_i32, newline, space1};
use nom::combinator::{all_consuming, cut};
use nom::error::convert_error;
use nom::error::VerboseError;
use nom::multi::{separated_list0, separated_list1};
use nom::sequence::{separated_pair, terminated};
use nom::IResult as NomResult;
use std::collections::HashMap;

type VerboseNomResult<'a, O> = NomResult<&'a str, O, VerboseError<&'a str>>;

type LocationIds = Vec<i32>;

#[derive(Debug, From, Display)]
pub enum AoCError<'a> {
    #[from]
    // Parsing(nom::Err<NomError<&'a str>>),
    Parsing(nom::Err<VerboseError<&'a str>>),
}

fn nom_parser(input: &str) -> VerboseNomResult<Vec<(i32, i32)>> {
    all_consuming(terminated(
        separated_list0(newline, separated_pair(str_i32, cut(space1), str_i32)),
        alt((tag("\n\n"), tag("\n"), tag(""))),
    ))(input)
}

fn parse(input: &str) -> Result<(LocationIds, LocationIds), AoCError> {
    let (_, pairs) = nom_parser(input)?;
    let location_id_vectors =
        pairs
            .into_iter()
            .fold((Vec::new(), Vec::new()), |mut vs, (i0, i1)| {
                vs.0.push(i0);
                vs.1.push(i1);
                vs
            });
    Ok(location_id_vectors)
}

fn part1_internal(location_ids_1: &mut [i32], location_ids_2: &mut [i32]) -> i32 {
    location_ids_1.sort();
    location_ids_2.sort();
    location_ids_1
        .iter()
        .zip(location_ids_2.iter())
        .map(|(value1, value2)| (value1 - value2).abs())
        .sum()
}

fn part1(input: &str) -> Result<String, AoCError> {
    let (mut location_ids_1, mut location_ids_2) = parse(input)?;
    let distance = part1_internal(&mut location_ids_1, &mut location_ids_2);
    Ok(distance.to_string())
}

fn part2_internal(location_ids_1: &mut [i32], location_ids_2: &mut [i32]) -> i32 {
    let v2_counter = location_ids_2
        .iter()
        .fold(HashMap::new(), |mut counter, i| {
            counter.entry(i).and_modify(|v| *v += 1).or_insert(1);
            counter
        });
    location_ids_1
        .iter()
        .map(|v| v * v2_counter.get(v).unwrap_or(&0))
        .sum()
}

fn part2(input: &str) -> Result<String, AoCError> {
    let (mut location_ids_1, mut location_ids_2) = parse(input)?;
    let distance = part2_internal(&mut location_ids_1, &mut location_ids_2);
    Ok(distance.to_string())
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
            nom::Err::Error(ve) => Err(convert_error(input, ve)),
            nom::Err::Failure(ve) => Err(convert_error(input, ve)),
            nom::Err::Incomplete(_) => Err("input is imcomplete".to_string()),
        },
    }
}
