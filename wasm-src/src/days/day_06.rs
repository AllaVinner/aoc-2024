use derive_more::derive::Display;
use derive_more::From;
use std::cmp::max;
use std::collections::HashSet;

type VerboseNomResult<'a, O> = nom::IResult<&'a str, O, nom::error::VerboseError<&'a str>>;

type Position = (i32, i32);

#[derive(Debug, From, Display)]
pub enum AoCError {
    #[display(
        "found multiple starting positions ('^'). \n\tStart 1: ({0},{1}) \n\tStart 2: ({2},{3})",
        start_1.0,
        start_1.1,
        start_2.0,
        start_2.1,
    )]
    MultipleStartPositions {
        start_1: Position,
        start_2: Position,
    },
    #[display("found no start postion ('^')")]
    NoStartPosition,
    #[display(
        "found unexpected character.\n\tCharacter: {c:#?}\n\tposition: ({0},{1})\n\tExpected: {expected}",
        pos.0,
        pos.1
    )]
    UnExpectedCharacter {
        c: char,
        pos: Position,
        expected: String,
    },
}

fn parser(input: &str) -> Result<(HashSet<Position>, Position, Position), AoCError> {
    let mut row_i: i32 = 0;
    let mut col_i: i32 = 0;
    let mut max_col: i32 = 0;
    let mut start_position: Option<Position> = None;
    let mut occupied_positions: HashSet<Position> = HashSet::new();

    for c in input.trim().chars() {
        let item = match c {
            '.' => None,
            '#' => Some((row_i, col_i)),
            '^' => {
                start_position = match start_position {
                    Some(p) => {
                        return Err(AoCError::MultipleStartPositions {
                            start_1: p,
                            start_2: (row_i, col_i),
                        })
                    }
                    None => Some((row_i, col_i)),
                };
                None
            }
            '\n' => {
                row_i += 1;
                max_col = max(max_col, col_i - 1);
                col_i = 0;
                continue;
            }
            _ => {
                return Err(AoCError::UnExpectedCharacter {
                    c,
                    pos: (row_i, col_i),
                    expected: "one of '.', '#', '^', '\\n'".to_string(),
                })
            }
        };
        match item {
            Some(pos) => {
                occupied_positions.insert(pos);
            }
            None => (),
        }
        col_i += 1;
    }
    max_col = max(max_col, col_i - 1);
    let board_size = (row_i + 1, max_col + 1);
    let start_position = match start_position {
        Some(pos) => pos,
        None => return Err(AoCError::NoStartPosition),
    };
    Ok((occupied_positions, start_position, board_size))
}

#[derive(Debug, Display, Hash, PartialEq, Eq, Clone, Copy)]
enum Direction {
    N,
    E,
    S,
    W,
}

fn part1(input: &str) -> Result<String, AoCError> {
    let (occupied_positions, start_position, board_size) = parser(input)?;
    let mut visited: HashSet<Position> = HashSet::new();
    let mut direction = Direction::N;
    let mut current_pos = start_position;
    loop {
        if current_pos.0 < 0 || board_size.0 <= current_pos.0 {
            break;
        }
        if current_pos.1 < 0 || board_size.1 <= current_pos.1 {
            break;
        }
        visited.insert(current_pos);
        let next_pos = match direction {
            Direction::N => (current_pos.0 - 1, current_pos.1),
            Direction::E => (current_pos.0, current_pos.1 + 1),
            Direction::S => (current_pos.0 + 1, current_pos.1),
            Direction::W => (current_pos.0, current_pos.1 - 1),
        };
        if occupied_positions.contains(&next_pos) {
            direction = match direction {
                Direction::N => Direction::E,
                Direction::E => Direction::S,
                Direction::S => Direction::W,
                Direction::W => Direction::N,
            };
            continue;
        }
        current_pos = next_pos;
    }

    return Ok(visited.len().to_string());
}

fn part2(input: &str) -> Result<String, AoCError> {
    let (mut occupied_positions, start_position, board_size) = parser(input)?;
    let mut num_loops = 0;
    for obs_row_i in 0..board_size.0 {
        for obs_col_i in 0..board_size.1 {
            let obs_pos = (obs_row_i, obs_col_i);
            if obs_pos == start_position {
                continue;
            }
            if occupied_positions.contains(&obs_pos) {
                continue;
            }
            occupied_positions.insert(obs_pos);
            let mut visited_states: HashSet<(Position, Direction)> = HashSet::new();
            let mut direction = Direction::N;
            let mut current_pos = start_position;
            let mut created_loop = false;
            loop {
                if obs_pos == (6, 3) {
                    println!(
                        "Pos: {},{}, direction {}",
                        current_pos.0, current_pos.1, direction
                    );
                }
                if current_pos.0 < 0 || board_size.0 <= current_pos.0 {
                    break;
                }
                if current_pos.1 < 0 || board_size.1 <= current_pos.1 {
                    break;
                }
                if !visited_states.insert((current_pos, direction)) {
                    created_loop = true;
                    break;
                }
                let next_pos = match direction {
                    Direction::N => (current_pos.0 - 1, current_pos.1),
                    Direction::E => (current_pos.0, current_pos.1 + 1),
                    Direction::S => (current_pos.0 + 1, current_pos.1),
                    Direction::W => (current_pos.0, current_pos.1 - 1),
                };
                if occupied_positions.contains(&next_pos) {
                    direction = match direction {
                        Direction::N => Direction::E,
                        Direction::E => Direction::S,
                        Direction::S => Direction::W,
                        Direction::W => Direction::N,
                    };
                    continue;
                }
                current_pos = next_pos;
            }
            if created_loop {
                num_loops += 1;
            }

            occupied_positions.remove(&obs_pos);
        }
    }
    return Ok(num_loops.to_string());
}

pub fn solve(input: &str, part: i32) -> Result<String, String> {
    let res = match part {
        1 => part1(input),
        2 => part2(input),
        i => return Err(format!("day 6 part {i} is not implemented")),
    };
    match res {
        Ok(v) => Ok(v),
        Err(e) => Err(e.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const TEST_INPUT: &str = "\
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn test_part1() {
        let result = part1(TEST_INPUT);
        assert_eq!(result.is_ok(), true);
        assert_eq!(result.unwrap(), "41")
    }

    #[test]
    fn test_part2() {
        let result = part2(TEST_INPUT);
        assert_eq!(result.is_ok(), true);
        assert_eq!(result.unwrap(), "6")
    }
}
