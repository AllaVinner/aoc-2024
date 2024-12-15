use derive_more::derive::Display;
use derive_more::From;
use nom;
use std::fs;

type VerboseNomResult<'a, O> = nom::IResult<&'a str, O, nom::error::VerboseError<&'a str>>;

#[derive(Debug, From, Display)]
pub enum AoCError {
    #[from]
    #[display("found unexpected character, expected digit.\n\tChar: 'c'\n\tPosition: {position}")]
    Parsing { c: char, position: usize },
}

fn nom_parser(input: &str) -> VerboseNomResult<String> {
    Ok(("A", "s".to_string()))
}

fn parse(input: &str) -> Result<Vec<u32>, AoCError> {
    input
        .trim()
        .chars()
        .enumerate()
        .map(|(i, c)| {
            c.to_digit(10)
                .ok_or_else(|| AoCError::Parsing { c, position: i })
        })
        .collect()
}

fn part1_setup(disk_map: &[u32]) -> Vec<Option<u32>> {
    let mut memory_map: Vec<Option<u32>> = Vec::with_capacity(10 * disk_map.len());
    let mut disk_iter = disk_map.iter();
    let mut file_id = 0;
    loop {
        let file_size = match disk_iter.next() {
            Some(s) => *s,
            None => break,
        };
        for _ in 0..file_size {
            memory_map.push(Some(file_id));
        }
        file_id += 1;
        let mem_size = match disk_iter.next() {
            Some(s) => *s,
            None => break,
        };
        for _ in 0..mem_size {
            memory_map.push(None);
        }
    }
    return memory_map;
}

fn part1_compress(mut memory_map: Vec<Option<u32>>) -> Vec<Option<u32>> {
    let mut right_pointer = memory_map.len() - 1;
    let mut left_pointer = 0;
    'main: loop {
        let right = 'right_loop: loop {
            match memory_map[right_pointer] {
                Some(v) => break 'right_loop v,
                None => {
                    if right_pointer == 0 {
                        break 'main;
                    }
                    right_pointer -= 1;
                    continue;
                }
            };
        };
        'left_loop: loop {
            match memory_map[left_pointer] {
                Some(_) => {
                    left_pointer += 1;
                    if left_pointer == memory_map.len() {
                        break 'main;
                    }
                    continue;
                }
                None => break 'left_loop,
            };
        }
        if right_pointer <= left_pointer {
            break 'main;
        }
        memory_map[left_pointer] = Some(right);
        memory_map[right_pointer] = None;
    }
    memory_map
}

fn part2_compress(mut memory_map: Vec<Option<u32>>) -> Vec<Option<u32>> {
    let mut right_pointer = memory_map.len() - 1;
    let mut left_pointer = 0;
    'main: loop {
        let right = 'right_loop: loop {
            match memory_map[right_pointer] {
                Some(v) => break 'right_loop v,
                None => {
                    if right_pointer == 0 {
                        break 'main;
                    }
                    right_pointer -= 1;
                    continue;
                }
            };
        };
        'left_loop: loop {
            match memory_map[left_pointer] {
                Some(_) => {
                    left_pointer += 1;
                    if left_pointer == memory_map.len() {
                        break 'main;
                    }
                    continue;
                }
                None => break 'left_loop,
            };
        }
        if right_pointer <= left_pointer {
            break 'main;
        }
        memory_map[left_pointer] = Some(right);
        memory_map[right_pointer] = None;
    }
    memory_map
}

fn part1_calculate_score(memory_map: &[Option<u32>]) -> u64 {
    memory_map
        .iter()
        .enumerate()
        .map(|(i, op)| match op {
            Some(v) => *v as u64 * i as u64,
            None => 0,
        })
        .sum()
}

fn part1(input: &str) -> Result<String, AoCError> {
    let disk_map = parse(input)?;
    let memory_map = part1_setup(disk_map.as_slice());
    let memory_map = part1_compress(memory_map);
    let score = part1_calculate_score(memory_map.as_slice());
    Ok(score.to_string())
}

fn part2(input: &str) -> Result<String, AoCError> {
    let disk_map = parse(input)?;
    let memory_map = part1_setup(disk_map.as_slice());
    let memory_map = part2_compress(memory_map);
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
        Err(e) => Err(e.to_string()),
    }
}

#[cfg(test)]
mod tests {

    use super::{part1, part2};
    use std::fs;

    const TEST_INPUT: &str = "2333133121414131402";

    #[test]
    fn test_part1() {
        let result = part1(TEST_INPUT);
        assert_eq!(result.is_ok(), true);
        assert_eq!(result.unwrap(), "1928");
    }

    #[test]
    fn test_part1_file() {
        let input = fs::read_to_string("./../data/day_09/main.txt").unwrap();
        let result = part1(&input);
        assert_eq!(result.is_ok(), true);
        assert_eq!(result.unwrap(), "6279058075753");
    }

    #[test]
    fn test_part1_setup() {
        let result = super::part1_setup(&[1, 2, 3, 4, 5]);
        assert_eq!(
            result,
            vec![
                Some(0),
                None,
                None,
                Some(1),
                Some(1),
                Some(1),
                None,
                None,
                None,
                None,
                Some(2),
                Some(2),
                Some(2),
                Some(2),
                Some(2)
            ]
        );
    }
    #[test]
    fn test_part1_compress() {
        let disk_map = super::parse("12345").unwrap();
        let mut memory_map = super::part1_setup(disk_map.as_slice());
        let memory_map = super::part1_compress(memory_map);
        assert_eq!(
            memory_map,
            vec![
                Some(0),
                Some(2),
                Some(2),
                Some(1),
                Some(1),
                Some(1),
                Some(2),
                Some(2),
                Some(2),
                None,
                None,
                None,
                None,
                None,
                None,
            ]
        );
    }

    #[test]
    fn test_part2() {
        let result = part2(TEST_INPUT);
        assert_eq!(result.is_ok(), true);
        assert_eq!(result.unwrap(), "2858");
    }
}
