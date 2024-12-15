use derive_more::derive::Display;
use derive_more::From;
use nom;
use std::cmp::max;
use std::collections::HashMap;
use std::collections::HashSet;

type Pos = (usize, usize);
type AntennaMap = HashMap<char, Vec<Pos>>;

type VerboseNomResult<'a, O> = nom::IResult<&'a str, O, nom::error::VerboseError<&'a str>>;

#[derive(Debug, From, Display)]
pub enum AoCError<'a> {
    #[from]
    // Parsing(nom::Err<NomError<&'a str>>),
    Parsing(nom::Err<nom::error::VerboseError<&'a str>>),
}

fn nom_parser(input: &str) -> VerboseNomResult<String> {
    Ok(("A", "s".to_string()))
}

fn parse(input: &str) -> Result<(AntennaMap, Pos), AoCError> {
    let mut antenna_map: AntennaMap = HashMap::new();
    let mut row_i = 0;
    let mut max_col = 0;
    let mut col_i = 0;
    for c in input.trim().chars() {
        match c {
            '.' => (),
            '\n' => {
                col_i = 0;
                row_i += 1;
                continue;
            }
            antenna_c => match antenna_map.get_mut(&antenna_c) {
                Some(pos) => pos.push((row_i, col_i)),
                None => {
                    antenna_map.insert(antenna_c, vec![(row_i, col_i)]);
                }
            },
        }
        max_col = max(max_col, col_i);
        col_i += 1;
    }

    Ok((antenna_map, (row_i + 1, max_col + 1)))
}

fn get_antinodes(a1: Pos, a2: Pos, board_size: Pos) -> Option<Pos> {
    // firs
    let a1_close = if 2 * a1.0 < a2.0 || 2 * a1.1 < a2.1 {
        None
    } else {
        let a = (2 * a1.0 - a2.0, 2 * a1.1 - a2.1);
        if a.0 >= board_size.0 || a.1 >= board_size.1 {
            None
        } else {
            Some(a)
        }
    };
    a1_close
}

fn get_resonant_antinodes(a1: Pos, a2: Pos, board_size: Pos) -> Vec<Pos> {
    // firs
    let mut n1 = a1;
    let mut n2 = a2;
    let mut antinodes: Vec<Pos> = vec![n1, n2];
    loop {
        let n3 = if 2 * n2.0 < n1.0 || 2 * n2.1 < n1.1 {
            break;
        } else {
            let a = (2 * n2.0 - n1.0, 2 * n2.1 - n1.1);
            if a.0 >= board_size.0 || a.1 >= board_size.1 {
                break;
            } else {
                a
            }
        };
        antinodes.push(n3);
        n1 = n2.clone();
        n2 = n3.clone();
    }
    antinodes
}

fn explain_part1(input: &str) -> Result<String, AoCError> {
    let (antenna_map, (board_size)) = parse(input)?;
    let mut s = String::new();
    s.push_str(&format!(
        "Board Size: ({}, {})\n",
        board_size.0, board_size.1
    ));

    let mut occupied_pos: HashSet<Pos> = HashSet::new();
    for (antenna_name, antenna_pos) in antenna_map.iter() {
        for a1_pos in antenna_pos.iter() {
            for a2_pos in antenna_pos.iter() {
                if a1_pos == a2_pos {
                    continue;
                }
                if let Some(pos) = get_antinodes(*a1_pos, *a2_pos, board_size) {
                    occupied_pos.insert(pos);
                }
                if let Some(pos) = get_antinodes(*a2_pos, *a1_pos, board_size) {
                    occupied_pos.insert(pos);
                }
            }
        }
    }
    let mut num = 0;
    for row_i in 0..board_size.0 {
        for col_i in 0..board_size.1 {
            if occupied_pos.contains(&(row_i, col_i)) {
                s.push_str("#");
                num += 1;
            } else {
                s.push_str(".");
            }
        }
        s.push_str("\n")
    }
    s.push_str(&format!("Num matckes: {}", num));
    Ok(s)
}

fn explain_part2(input: &str) -> Result<String, AoCError> {
    let (antenna_map, (board_size)) = parse(input)?;
    let mut s = String::new();
    s.push_str(&format!(
        "Board Size: ({}, {})\n",
        board_size.0, board_size.1
    ));

    let mut occupied_pos: HashSet<Pos> = HashSet::new();
    for (antenna_name, antenna_pos) in antenna_map.iter() {
        for a1_pos in antenna_pos.iter() {
            for a2_pos in antenna_pos.iter() {
                if a1_pos == a2_pos {
                    continue;
                }
                for antinode in get_resonant_antinodes(*a1_pos, *a2_pos, board_size).iter() {
                    occupied_pos.insert(*antinode);
                }
                for antinode in get_resonant_antinodes(*a2_pos, *a1_pos, board_size).iter() {
                    occupied_pos.insert(*antinode);
                }
            }
        }
    }
    let mut num = 0;
    for row_i in 0..board_size.0 {
        for col_i in 0..board_size.1 {
            if occupied_pos.contains(&(row_i, col_i)) {
                s.push_str("#");
                num += 1;
            } else {
                s.push_str(".");
            }
        }
        s.push_str("\n")
    }
    s.push_str(&format!("Num matckes: {}", num));
    Ok(s)
}
fn part1(input: &str) -> Result<String, AoCError> {
    let (antenna_map, (board_size)) = parse(input)?;
    let mut occupied_pos: HashSet<Pos> = HashSet::new();
    for (antenna_name, antenna_pos) in antenna_map.iter() {
        for a1_pos in antenna_pos.iter() {
            for a2_pos in antenna_pos.iter() {
                if a1_pos == a2_pos {
                    break;
                }
                if let Some(pos) = get_antinodes(*a1_pos, *a2_pos, board_size) {
                    occupied_pos.insert(pos);
                }
                if let Some(pos) = get_antinodes(*a2_pos, *a1_pos, board_size) {
                    occupied_pos.insert(pos);
                }
            }
        }
    }
    return Ok(occupied_pos.len().to_string());
}

fn part2(input: &str) -> Result<String, AoCError> {
    let (antenna_map, (board_size)) = parse(input)?;
    let mut occupied_pos: HashSet<Pos> = HashSet::new();
    for (antenna_name, antenna_pos) in antenna_map.iter() {
        for a1_pos in antenna_pos.iter() {
            for a2_pos in antenna_pos.iter() {
                if a1_pos == a2_pos {
                    break;
                }
                for antinode in get_resonant_antinodes(*a1_pos, *a2_pos, board_size).iter() {
                    occupied_pos.insert(*antinode);
                }
                for antinode in get_resonant_antinodes(*a2_pos, *a1_pos, board_size).iter() {
                    occupied_pos.insert(*antinode);
                }
            }
        }
    }
    return Ok(occupied_pos.len().to_string());
}

pub fn solve(input: &str, part: i32) -> Result<String, String> {
    let res = match part {
        1 => part1(input),
        2 => explain_part2(input),
        i => return Err(format!("day 8 part {i} is not implemented")),
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
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    const BASIC_INPUT: &str = "\
##..";

    #[test]
    fn test_part1() {
        let result = part1(TEST_INPUT);
        assert_eq!(result.is_ok(), true);
        assert_eq!(result.unwrap(), "14")
    }

    #[test]
    fn test_part2() {
        let result = part2(TEST_INPUT);
        assert_eq!(result.is_ok(), true);
        assert_eq!(result.unwrap(), "34")
    }

    #[test]
    fn test_part_2_basic() {
        let result = part2(BASIC_INPUT);
        assert_eq!(result.is_ok(), true);
        assert_eq!(result.unwrap(), "4")
    }
}
