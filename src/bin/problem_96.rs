use itertools::Itertools;
use std::collections::HashMap;
use rayon::prelude::*;

#[derive(Clone)]
struct SudokuPuzzle {
    board: Vec<Vec<Option<usize>>>,
}

impl std::fmt::Debug for SudokuPuzzle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let mut out_str = String::new();
        for row_idx in 0..9 {
            for col_idx in 0..9 {
                out_str += &self.board[row_idx][col_idx].map_or(".".to_string(), |x| x.to_string());
                if col_idx % 3 == 2 {
                    out_str += "|";
                }
            }
            out_str += "\n";
            if row_idx % 3 == 2 {
                out_str += "---+---+---\n";
            }
        }
        return write!(f, "{}", out_str);
    }
}

impl SudokuPuzzle {
    fn from_str(input: &str) -> Option<SudokuPuzzle> {
        let board = input
            .split_whitespace()
            .map(|line| {
                line.chars()
                    .map(|c| {
                        let entry = usize::from_str_radix(&c.to_string(), 10).unwrap();
                        if entry == 0 {
                            return None;
                        } else {
                            return Some(entry);
                        }
                    })
                    .collect()
            })
            .collect();
        return Some(SudokuPuzzle { board: board });
    }

    fn check_row(&self, row_idx: usize) -> bool {
        let counts = (0..9).fold(HashMap::new(), |mut accum, col_idx| {
            if let Some(val) = self.board[row_idx][col_idx] {
                *accum.entry(val).or_insert(0) += 1;
            }
            accum
        });
        return counts.into_iter().all(|(_, value)| value == 1);
    }

    fn check_col(&self, col_idx: usize) -> bool {
        let counts = (0..9).fold(HashMap::new(), |mut accum, row_idx| {
            if let Some(val) = self.board[row_idx][col_idx] {
                *accum.entry(val).or_insert(0) += 1;
            }
            accum
        });
        return counts.into_iter().all(|(_, value)| value == 1);
    }

    fn check_cell(&self, cell_idx: usize) -> bool {
        let row_start = (cell_idx / 3) * 3;
        let row_end = row_start + 3;
        let col_start = (cell_idx % 3) * 3;
        let col_end = col_start + 3;
        let counts = (row_start..row_end)
            .cartesian_product(col_start..col_end)
            .fold(HashMap::new(), |mut accum, (row_idx, col_idx)| {
                if let Some(val) = self.board[row_idx][col_idx] {
                    *accum.entry(val).or_insert(0) += 1;
                }
                accum
            });
        return counts.into_iter().all(|(_, value)| value == 1);
    }

    fn is_consistent(&self) -> bool {
        for idx in 0..9 {
            let is_idx_ok = self.check_row(idx) && self.check_col(idx) && self.check_cell(idx);
            if !is_idx_ok {
                return false;
            }
        }
        return true;
    }

    fn solve(&self) -> Option<SudokuPuzzle> {
        // Check if the board is consistent
        if !self.is_consistent() {
            return None;
        }
        // Find the first unset cell
        let maybe_unset = (0..9usize)
            .cartesian_product(0..9usize)
            .find_position(|&(row_idx, col_idx)| self.board[row_idx][col_idx].is_none());

        if maybe_unset.is_none() {
            return Some(self.clone());
        }

        for val in 1..=9 {
            // Make a board with the unset cell set to this new value
            let (row_idx, col_idx) = maybe_unset.unwrap().1;
            let mut updated_puzzle = self.clone();
            updated_puzzle.board[row_idx][col_idx] = Some(val);
            let maybe_solution = updated_puzzle.solve();
            if maybe_solution.is_some() {
                return maybe_solution;
            }
        }
        return None;
    }
}

fn parse_puzzle(input: &str) -> Option<SudokuPuzzle> {
    return match input.split_once("\n") {
        None => None,
        Some((_, suffix)) => SudokuPuzzle::from_str(suffix),
    };
}

fn parse_puzzles(input: &str) -> Vec<SudokuPuzzle> {
    return input
        .split("Grid")
        .map(parse_puzzle)
        .filter(|x| x.is_some())
        .map(|x| x.unwrap())
        .collect();
}

fn main() {
    let file_name = std::path::Path::new("src/bin/p096_sudoku.txt");
    let puzzles = std::fs::read_to_string(file_name).unwrap();
    let puzzles = parse_puzzles(&puzzles);

    let answer: usize = puzzles
        .par_iter()
        .enumerate()
        .map(|(idx, puzzle)| {
            println!("Grid: {}", idx);
            println!("{:#?}", puzzle);
            let solved = puzzle.solve().unwrap();
            println!("{:#?}", solved);
            return 100 * solved.board[0][0].unwrap()
                + 10 * solved.board[0][1].unwrap()
                + solved.board[0][2].unwrap();
        })
        .sum();
    println!("Sum: {}", answer);
}
