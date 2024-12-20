use derive_more::derive::Display;
use derive_more::From;
use itertools::Itertools;
use ndarray::Array2;
use ndarray::ArrayView2;
use ndarray::ShapeError;
use std::fmt;

#[derive(Debug)]
struct CharLocation {
    c: char,
    row: usize,
    col: usize,
}

#[derive(Debug, Display)]
enum Corner {
    TopLeft,
    TopRight,
    BottomRight,
    BottomLeft,
}

#[derive(Debug)]
struct CornerOutOfBounds {
    row_i: char,
    col_i: usize,
    corner: Corner,
}

impl fmt::Display for CornerOutOfBounds {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} corner of {}:{} is out of bounds",
            self.corner, self.row_i, self.col_i
        )
    }
}

impl fmt::Display for CharLocation {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "character '{}', found at {}:{}",
            self.c, self.row, self.col
        )
    }
}

#[derive(Copy, Debug, Clone, PartialEq)]
enum Item {
    X,
    M,
    A,
    S,
}

struct RightUpMatrixIter<'a, I: Copy> {
    matrix: ArrayView2<'a, I>,
    diagonal_index: usize,
    item_index: usize,
}

impl<'a, I: Copy> Iterator for RightUpMatrixIter<'a, I> {
    type Item = I;
    fn next(&mut self) -> Option<Self::Item> {
        let row_root = if self.diagonal_index < self.matrix.nrows() {
            self.diagonal_index
        } else {
            self.matrix.nrows() - 1
        };
        let col_root = if self.diagonal_index < self.matrix.nrows() {
            0
        } else {
            self.diagonal_index - self.matrix.nrows() + 1
        };
        if row_root < self.item_index {
            return None;
        }
        let v = self
            .matrix
            .get((row_root - self.item_index, col_root + self.item_index))
            .map(|v| *v);
        self.item_index += 1;
        v
    }
}

fn iter_right_up<I: Copy>(matrix: ArrayView2<I>, diagonal_index: usize) -> RightUpMatrixIter<I> {
    return RightUpMatrixIter {
        matrix,
        diagonal_index,
        item_index: 0,
    };
}

struct RightDownMatrixIter<'a, I: Copy> {
    matrix: ArrayView2<'a, I>,
    diagonal_index: usize,
    item_index: usize,
}

impl<'a, I: Copy> Iterator for RightDownMatrixIter<'a, I> {
    type Item = I;
    fn next(&mut self) -> Option<Self::Item> {
        let col_root = if self.diagonal_index < self.matrix.nrows() {
            0
        } else {
            self.diagonal_index - self.matrix.nrows() + 1
        };
        let row_root = if self.diagonal_index < self.matrix.nrows() {
            self.matrix.nrows() - 1 - self.diagonal_index
        } else {
            0
        };
        let v = self
            .matrix
            .get((row_root + self.item_index, col_root + self.item_index))
            .map(|v| *v);
        self.item_index += 1;
        v
    }
}

fn iter_right_down<I: Copy>(
    matrix: ArrayView2<I>,
    diagonal_index: usize,
) -> RightDownMatrixIter<I> {
    return RightDownMatrixIter {
        matrix,
        diagonal_index,
        item_index: 0,
    };
}

#[derive(Debug, From, Display)]
pub enum AoCError {
    UnExpectedCharacter(CharLocation),
    #[from]
    ShapeError(ShapeError),
    MatrxiIndexError(CornerOutOfBounds),
}

fn parser(input: &str) -> Result<Array2<Item>, AoCError> {
    let mut row_i = 0;
    let mut col_i = 0;
    let mut items = vec![];
    for c in input.trim().chars() {
        let item = match c {
            'X' => Item::X,
            'M' => Item::M,
            'A' => Item::A,
            'S' => Item::S,
            '\n' => {
                row_i += 1;
                col_i = 0;
                continue;
            }
            _ => {
                return Err(AoCError::UnExpectedCharacter(CharLocation {
                    c,
                    row: row_i,
                    col: col_i,
                }))
            }
        };
        items.push(item);
        col_i += 1;
    }
    let num_row = row_i + 1;
    let num_col = items.len() / num_row;
    Ok(Array2::from_shape_vec((num_row, num_col), items)?)
}

fn is_xmas(items: &(Item, Item, Item, Item)) -> bool {
    *items == (Item::X, Item::M, Item::A, Item::S) || *items == (Item::S, Item::A, Item::M, Item::X)
}

fn part1(input: &str) -> Result<String, AoCError> {
    let mat = parser(input)?;
    let mut count = 0;
    for row in mat.rows() {
        count += row
            .iter()
            .map(|v| *v)
            .tuple_windows::<(_, _, _, _)>()
            .filter(|v| is_xmas(v))
            .count();
    }
    for col in mat.columns() {
        count += col
            .iter()
            .map(|v| *v)
            .tuple_windows::<(_, _, _, _)>()
            .filter(|v| is_xmas(v))
            .count();
    }
    for diag_i in 0..(mat.nrows() + mat.ncols() - 1) {
        count += iter_right_up((&mat).into(), diag_i)
            .tuple_windows()
            .map(|v| {
                dbg!(&v);
                v
            })
            .filter(|v| is_xmas(v))
            .count();
    }
    for diag_i in 0..(mat.nrows() + mat.ncols() - 1) {
        count += iter_right_down((&mat).into(), diag_i)
            .tuple_windows()
            .filter(|v| is_xmas(v))
            .count();
    }
    return Ok(count.to_string());
}

fn part2(input: &str) -> Result<String, AoCError> {
    let mat = parser(input)?;
    let mut count = 0;
    for ((row_i, col_i), item) in mat.indexed_iter() {
        if col_i == 0 || row_i == 0 {
            continue;
        }
        if col_i == mat.ncols() - 1 || row_i == mat.nrows() - 1 {
            continue;
        }
        if *item != Item::A {
            continue;
        }
        let corners = vec![
            mat.get((row_i + 1, col_i - 1)).unwrap(),
            mat.get((row_i + 1, col_i + 1)).unwrap(),
            mat.get((row_i - 1, col_i + 1)).unwrap(),
            mat.get((row_i - 1, col_i - 1)).unwrap(),
        ];
        if corners.iter().filter(|&&v| *v == Item::M).count() != 2 {
            continue;
        }
        if corners.iter().filter(|&&v| *v == Item::S).count() != 2 {
            continue;
        }
        if corners[0] == corners[2] {
            continue;
        }
        if corners[1] == corners[3] {
            continue;
        }
        count += 1;
    }

    return Ok(count.to_string());
}

pub fn solve(input: &str, part: i32) -> Result<String, String> {
    let res = match part {
        1 => part1(input),
        2 => part2(input),
        i => return Err(format!("day 1 part {i} is not implemented")),
    };
    match res {
        Ok(v) => Ok(v),
        Err(e) => Err(format!("{}", e)),
    }
}

#[cfg(test)]
mod tests {
    use super::{iter_right_down, iter_right_up, parser, part1, part2};

    const TEST_INPUT: &str = "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn test_right_up() {
        let m = super::parser(TEST_INPUT).unwrap();
        let mut diter = super::iter_right_up((&m).into(), 0);
        assert_eq!(Some(super::Item::M), diter.next());
        assert_eq!(None, diter.next());

        let mut diter = super::iter_right_up((&m).into(), 1);
        assert_eq!(Some(super::Item::M), diter.next());
        assert_eq!(Some(super::Item::M), diter.next());
        assert_eq!(None, diter.next());

        let mut diter = super::iter_right_up((&m).into(), 2);
        assert_eq!(Some(super::Item::A), diter.next());
        assert_eq!(Some(super::Item::S), diter.next());
        assert_eq!(Some(super::Item::M), diter.next());
        assert_eq!(None, diter.next());

        let mut diter = super::iter_right_up((&m).into(), 9);
        assert_eq!(Some(super::Item::M), diter.next());
        assert_eq!(Some(super::Item::A), diter.next());
        assert_eq!(Some(super::Item::X), diter.next());
        assert_eq!(Some(super::Item::M), diter.last());

        let mut diter = super::iter_right_up((&m).into(), 10);
        assert_eq!(Some(super::Item::X), diter.next());
        assert_eq!(Some(super::Item::M), diter.next());
        assert_eq!(Some(super::Item::A), diter.next());
        assert_eq!(Some(super::Item::S), diter.next());
        assert_eq!(Some(super::Item::A), diter.last());

        let mut diter = super::iter_right_up((&m).into(), 17);
        assert_eq!(Some(super::Item::S), diter.next());
        assert_eq!(Some(super::Item::M), diter.next());
        assert_eq!(None, diter.next());
    }
    #[test]
    fn test_right_down() {
        let m = super::parser(TEST_INPUT).unwrap();
        let mut diter = super::iter_right_down((&m).into(), 0);
        assert_eq!(Some(super::Item::M), diter.next());
        assert_eq!(None, diter.next());

        let mut diter = super::iter_right_down((&m).into(), 1);
        assert_eq!(Some(super::Item::M), diter.next());
        assert_eq!(Some(super::Item::X), diter.next());
        assert_eq!(None, diter.next());

        let mut diter = super::iter_right_down((&m).into(), 2);
        assert_eq!(Some(super::Item::S), diter.next());
        assert_eq!(Some(super::Item::A), diter.next());
        assert_eq!(Some(super::Item::M), diter.next());
        assert_eq!(None, diter.next());

        let mut diter = super::iter_right_down((&m).into(), 8);
        assert_eq!(Some(super::Item::M), diter.next());
        assert_eq!(Some(super::Item::M), diter.next());
        assert_eq!(Some(super::Item::A), diter.next());
        assert_eq!(Some(super::Item::S), diter.last());

        let mut diter = super::iter_right_down((&m).into(), 9);
        assert_eq!(Some(super::Item::M), diter.next());
        assert_eq!(Some(super::Item::S), diter.next());
        assert_eq!(Some(super::Item::X), diter.next());
        assert_eq!(Some(super::Item::X), diter.last());

        let mut diter = super::iter_right_down((&m).into(), 17);
        assert_eq!(Some(super::Item::S), diter.next());
        assert_eq!(Some(super::Item::A), diter.next());
        assert_eq!(None, diter.next());

        let mut diter = super::iter_right_down((&m).into(), 117);
        assert_eq!(None, diter.next());
    }

    #[test]
    fn test_part1() {
        let result = part1(TEST_INPUT);
        assert_eq!(result.is_ok(), true);
        assert_eq!(result.unwrap(), "18")
    }

    #[test]
    fn test_part2() {
        let result = part2(TEST_INPUT);
        assert_eq!(result.is_ok(), true);
        assert_eq!(result.unwrap(), "9")
    }
}
