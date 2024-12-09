use derive_more::derive::Display;
use derive_more::From;
use nom;
use nom::bytes::complete::tag;
use nom::multi::separated_list1;

type VerboseNomResult<'a, O> = nom::IResult<&'a str, O, nom::error::VerboseError<&'a str>>;

#[derive(Debug, From, Display)]
pub enum AoCError<'a> {
    #[from]
    // Parsing(nom::Err<NomError<&'a str>>),
    Parsing(nom::Err<nom::error::VerboseError<&'a str>>),
}

type PageRule = (i32, i32);

fn nom_parser(input: &str) -> VerboseNomResult<(Vec<PageRule>, Vec<Vec<i32>>)> {
    nom::sequence::separated_pair(
        nom::multi::separated_list1(
            tag("\n"),
            nom::sequence::separated_pair(
                nom::character::complete::i32,
                tag("|"),
                nom::character::complete::i32,
            ),
        ),
        tag("\n\n"),
        nom::multi::separated_list1(
            tag("\n"),
            nom::multi::separated_list1(tag(","), nom::character::complete::i32),
        ),
    )(input)
}

fn part1(input: &str) -> Result<String, AoCError> {
    let (_, (rules, update_lists)) = nom_parser(input)?;
    let mut score = 0;
    for updates in update_lists.iter() {
        let mut is_ordered = true;
        for rule in rules.iter() {
            println!("rule check {:?}", rule);
            let pre = match updates.iter().position(|v| *v == rule.0) {
                Some(p) => p,
                None => continue,
            };
            let post = match updates.iter().position(|v| *v == rule.1) {
                Some(p) => p,
                None => continue,
            };
            println!("pre psot {}:{}", pre, post);
            if pre > post {
                is_ordered = false;
                break;
            }
        }
        if is_ordered {
            score += updates.get(updates.len() / 2).unwrap()
        }
    }
    return Ok(score.to_string());
}

fn part2(input: &str) -> Result<String, AoCError> {
    return Ok("Part 5 Placeholder".to_string());
}

pub fn solve(input: &str, part: i32) -> Result<String, String> {
    let res = match part {
        1 => part1(input),
        2 => part2(input),
        i => return Err(format!("day 1 part {i} is not implemented")),
    };
    match res {
        Ok(v) => Ok(v),
        Err(e) => Err(format!("{e}")),
    }
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const TEST_INPUT: &str = "\
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn test_part1() {
        let result = part1(TEST_INPUT);
        assert_eq!(result.is_ok(), true);
        assert_eq!(result.unwrap(), "143")
    }

    #[test]
    fn test_part2() {
        let result = part2(TEST_INPUT);
        assert_eq!(result.is_ok(), true);
        assert_eq!(result.unwrap(), "31")
    }
}
