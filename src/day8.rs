use anyhow::Result;
use std::{fs, ops::Range, process::ExitCode};

#[derive(Debug)]
struct Forest {
    rows: usize,
    cols: usize,
    matrix: Vec<usize>,
}

impl From<&str> for Forest {
    fn from(s: &str) -> Self {
        let mut i: usize = 1;

        let matrix: Vec<usize> = s
            .trim_end()
            .chars()
            .filter_map(|c| {
                if c == '\n' {
                    i += 1;
                }

                c.to_digit(10).map(|d| d as usize)
            })
            .collect();

        Self {
            rows: i,
            cols: matrix.len() / i,
            matrix,
        }
    }
}

impl Forest {
    /// Get forest's matrix element value (returns None if indices are out of bounds).
    fn element(&self, i: usize, j: usize) -> Option<usize> {
        if i >= self.rows || j >= self.cols {
            return None;
        }

        Some(self.matrix[i * self.cols + j])
    }

    /// Check wheter the tree at given position is at the edge of the forest or not.
    fn is_edge(&self, i: usize, j: usize) -> bool {
        i == 0 || i == self.cols - 1 || j == 0 || j == self.rows - 1
    }

    /// Check wheter the tree at given position is visible from at least one side.
    fn is_visible(&self, i: usize, j: usize) -> bool {
        // Edges are always visible.
        if self.is_edge(i, j) {
            return true;
        }

        let horizontal = |range: Range<usize>| -> bool {
            let mut visible = true;
            for k in range {
                if self.element(i, k) >= self.element(i, j) {
                    visible = false;
                    break;
                }
            }

            visible
        };

        let vertical = |range: Range<usize>| -> bool {
            let mut visible = true;
            for k in range {
                if self.element(k, j) >= self.element(i, j) {
                    visible = false;
                    break;
                }
            }

            visible
        };

        // Look left.
        if horizontal(0..j) {
            return true;
        }

        // Look right.
        if horizontal(j + 1..self.cols) {
            return true;
        }

        // Look up.
        if vertical(0..i) {
            return true;
        }

        // Look down.
        if vertical(i + 1..self.rows) {
            return true;
        }

        false
    }

    /// Count the number of visible trees (including edges).
    fn count_visible_trees(&self) -> usize {
        // Edges are always visible: save on iteration loops.
        let mut count: usize = self.rows * 2 + (self.cols - 2) * 2;

        // Loop only on inner trees.
        for i in 1..self.rows - 1 {
            for j in 1..self.cols - 1 {
                if self.is_visible(i, j) {
                    count += 1;
                }
            }
        }

        count
    }

    /// Calculate the tree's scenic score.
    fn scenic_score(&self, i: usize, j: usize) -> usize {
        let horizontal = |range: &[usize]| -> usize {
            let mut score: usize = 0;
            for k in range {
                score += 1;
                if self.element(i, *k) >= self.element(i, j) {
                    break;
                }
            }
            score
        };

        let vertical = |range: &[usize]| -> usize {
            let mut score: usize = 0;
            for k in range {
                score += 1;
                if self.element(*k, j) >= self.element(i, j) {
                    break;
                }
            }
            score
        };

        horizontal(&(0..j).rev().collect::<Vec<usize>>())
            * horizontal(&(j + 1..self.cols).collect::<Vec<usize>>())
            * vertical(&(0..i).rev().collect::<Vec<usize>>())
            * vertical(&(i + 1..self.rows).collect::<Vec<usize>>())
    }

    /// Find the highest scenic score possible for any tree.
    fn highest_score(&self) -> usize {
        let mut scores: Vec<usize> = vec![];
        for i in 0..self.rows {
            for j in 0..self.cols {
                scores.push(self.scenic_score(i, j));
            }
        }

        *scores.iter().max().unwrap()
    }
}

fn run() -> Result<()> {
    let input = fs::read_to_string("./input/day8.dat")?;

    let forest = Forest::from(input.as_str());

    // Part 1
    println!("Number of visible trees: {}", forest.count_visible_trees());

    // Part 2
    println!("Highest scenic score for any tree is: {}", forest.highest_score());

    Ok(())
}

fn main() -> ExitCode {
    if let Err(e) = run() {
        eprintln!("error: {e}");
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_test() {
        const INPUT: &str = r#"30373
25512
65332
33549
35390"#;

        let forest = Forest::from(INPUT);

        assert_eq!(Some(5), forest.element(2, 1));
        assert_eq!(Some(3), forest.element(2, 2));
        assert_eq!(Some(4), forest.element(3, 3));
        assert_eq!(Some(0), forest.element(4, 4));
        assert_eq!(None, forest.element(6, 0));
        assert_eq!(None, forest.element(1, 6));

        assert!(forest.is_edge(0, 0));
        assert!(forest.is_edge(forest.rows - 1, forest.cols - 1));
        assert!(forest.is_edge(2, forest.cols - 1));
        assert!(forest.is_edge(forest.rows - 1, 2));
        assert!(!forest.is_edge(2, 2));
        assert!(!forest.is_edge(3, 3));
        assert!(!forest.is_edge(1, 2));

        assert_eq!(21, forest.count_visible_trees());
        assert_eq!(4, forest.scenic_score(1, 2));
        assert_eq!(8, forest.scenic_score(3, 2));
        assert_eq!(8, forest.highest_score());
    }
}
