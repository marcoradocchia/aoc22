use anyhow::Result;
use std::{cmp::Ordering, fs, process::ExitCode};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Position {
    x: i64,
    y: i64,
}

impl Position {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    fn update_position(&mut self, direction: &Direction) {
        match direction {
            Direction::Up => self.y += 1,
            Direction::Down => self.y -= 1,
            Direction::Left => self.x -= 1,
            Direction::Right => self.x += 1,
        }
    }
}

/// Number of knots in the (new)rope.
const KNOTS_NUM: usize = 10;

#[derive(Debug, Clone)]
struct NewRope {
    knots: [Position; KNOTS_NUM],
    tail_history: Vec<Position>,
}

impl NewRope {
    fn new() -> Self {
        Self {
            knots: [Position::new(0, 0); KNOTS_NUM],
            tail_history: vec![Position::new(0, 0)], // Starting position is visited.
        }
    }

    /// Check if knots at given idices share the same row.
    fn same_row(&self, i: usize, j: usize) -> bool {
        self.knots[i].y == self.knots[j].y
    }

    /// Check if knots at given idices share the same column.
    fn same_col(&self, i: usize, j: usize) -> bool {
        self.knots[i].x == self.knots[j].x
    }

    /// Check if knots at given indices are touching.
    fn touching(&self, i: usize, j: usize) -> bool {
        self.knots[i].x.abs_diff(self.knots[j].x) <= 1
            && self.knots[i].y.abs_diff(self.knots[j].y) <= 1
    }

    /// Move [`NewRope`] head.
    fn move_head(&mut self, direction: &Direction) {
        self.knots[0].update_position(direction)
    }

    /// Move [`NewRope`] knot.
    ///
    /// # Panic
    /// Panics if trying to move knot at index 0. Use `move_head` method instead.
    fn move_knot(&mut self, i: usize) {
        if self.same_row(i, i - 1) {
            // Tail moves on the same row to catch up or stays in place if head and tail overlap.
            match self.knots[i - 1].x.cmp(&self.knots[i].x) {
                Ordering::Less => self.knots[i].update_position(&Direction::Left),
                Ordering::Equal => {} // Head and tail overlap.
                Ordering::Greater => self.knots[i].update_position(&Direction::Right),
            }
        } else if self.same_col(i, i - 1) {
            // Tail moves on the same col to catch up or stays in place if head and tail overlap.
            match self.knots[i - 1].y.cmp(&self.knots[i].y) {
                Ordering::Less => self.knots[i].update_position(&Direction::Down),
                Ordering::Equal => {} // Head and tail overlap.
                Ordering::Greater => self.knots[i].update_position(&Direction::Up),
            }
        } else {
            // Tail moves diagonally to catch up.
            match self.knots[i - 1].x.cmp(&self.knots[i].x) {
                Ordering::Less => self.knots[i].update_position(&Direction::Left),
                Ordering::Greater => self.knots[i].update_position(&Direction::Right),
                Ordering::Equal => unreachable!(), // Head and tail can't overlap at this point.
            }
            match self.knots[i - 1].y.cmp(&self.knots[i].y) {
                Ordering::Less => self.knots[i].update_position(&Direction::Down),
                Ordering::Greater => self.knots[i].update_position(&Direction::Up),
                Ordering::Equal => unreachable!(), // Head and tail can't overlap at this point.
            }
        }
    }

    /// Start movements.
    fn start(&mut self, movements: &[Movement]) {
        for movement in movements {
            for _ in 0..movement.amount {
                // Move head.
                self.move_head(&movement.direction);
                // Move other knots accordingly.
                for i in 1..KNOTS_NUM {
                    if !self.touching(i, i - 1) {
                        self.move_knot(i);
                        // Update tail position history.
                        if i == KNOTS_NUM - 1 {
                            self.tail_history.push(self.knots[KNOTS_NUM - 1]);
                        }
                    }
                }
            }
        }
    }

    /// Count unique tail visited positions.
    fn unique_visited_positions(&self) -> usize {
        let mut unique_visited_positions: Vec<Position> = vec![];
        for position in &self.tail_history {
            if !unique_visited_positions.contains(position) {
                unique_visited_positions.push(*position);
            }
        }

        unique_visited_positions.len()
    }
}

#[derive(Debug, Clone)]
struct Rope {
    head: Position,
    tail: Position,
    tail_history: Vec<Position>,
}

impl Rope {
    fn new() -> Self {
        Self {
            head: Position::new(0, 0),
            tail: Position::new(0, 0),
            tail_history: vec![Position::new(0, 0)], // Starting position is visited.
        }
    }

    /// Check if [`Rope`]s head ant tail are on the same row.
    fn head_tail_same_row(&self) -> bool {
        self.head.y == self.tail.y
    }

    /// Check if [`Rope`]s head ant tail are on the same col.
    fn head_tail_same_col(&self) -> bool {
        self.head.x == self.tail.x
    }

    /// Check if [`Rope`]s head ant tail are touching.
    fn head_tail_touching(&self) -> bool {
        self.head.x.abs_diff(self.tail.x) <= 1 && self.head.y.abs_diff(self.tail.y) <= 1
    }

    /// Move [`Rope`]s head.
    fn move_head(&mut self, direction: &Direction) {
        self.head.update_position(direction)
    }

    /// Move tail.
    fn move_tail(&mut self) {
        if self.head_tail_same_row() {
            // Tail moves on the same row to catch up or stays in place if head and tail overlap.
            match self.head.x.cmp(&self.tail.x) {
                Ordering::Less => self.tail.update_position(&Direction::Left),
                Ordering::Equal => {} // Head and tail overlap.
                Ordering::Greater => self.tail.update_position(&Direction::Right),
            }
        } else if self.head_tail_same_col() {
            // Tail moves on the same col to catch up or stays in place if head and tail overlap.
            match self.head.y.cmp(&self.tail.y) {
                Ordering::Less => self.tail.update_position(&Direction::Down),
                Ordering::Equal => {} // Head and tail overlap.
                Ordering::Greater => self.tail.update_position(&Direction::Up),
            }
        } else {
            // Tail moves diagonally to catch up.
            match self.head.x.cmp(&self.tail.x) {
                Ordering::Less => self.tail.update_position(&Direction::Left),
                Ordering::Greater => self.tail.update_position(&Direction::Right),
                Ordering::Equal => unreachable!(), // Head and tail can't overlap at this point.
            }
            match self.head.y.cmp(&self.tail.y) {
                Ordering::Less => self.tail.update_position(&Direction::Down),
                Ordering::Greater => self.tail.update_position(&Direction::Up),
                Ordering::Equal => unreachable!(), // Head and tail can't overlap at this point.
            }
        }
    }

    /// Start movements.
    fn start(&mut self, movements: &[Movement]) {
        for movement in movements {
            for _ in 0..movement.amount {
                // Move head.
                self.move_head(&movement.direction);
                // Move tail accordingly if head and tail are no longer touching after head's move.
                if !self.head_tail_touching() {
                    self.move_tail();
                    // Update tail position history.
                    self.tail_history.push(self.tail);
                }
            }
        }
    }

    /// Count unique tail visited positions.
    fn unique_visited_positions(&self) -> usize {
        let mut unique_visited_positions: Vec<Position> = vec![];
        for position in &self.tail_history {
            if !unique_visited_positions.contains(position) {
                unique_visited_positions.push(*position);
            }
        }

        unique_visited_positions.len()
    }
}

/// Direction of the movement.
#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl TryFrom<&str> for Direction {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(match value {
            "U" => Self::Up,
            "D" => Self::Down,
            "L" => Self::Left,
            "R" => Self::Right,
            d => anyhow::bail!("invalid direction '{d}'"),
        })
    }
}

/// Head movement.
#[derive(Debug, Clone)]
struct Movement {
    amount: usize,
    direction: Direction,
}

impl TryFrom<&str> for Movement {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self> {
        let (direction, amount) = value
            .split_once(' ')
            .ok_or(anyhow::format_err!("badly formatted input"))?;

        Ok(Self {
            amount: amount
                .parse()
                .map_err(|_| anyhow::format_err!("invalid amount '{amount}'"))?,
            direction: Direction::try_from(direction)?,
        })
    }
}

fn run() -> Result<()> {
    let input = fs::read_to_string("./input/day9.dat")?;
    let movements: Result<Vec<Movement>> = input
        .lines()
        .map(|line| -> Result<Movement> { Movement::try_from(line) })
        .collect();
    let movements = movements?;

    // Part 1
    let mut rope = Rope::new();
    rope.start(&movements);
    println!(
        "Unique tail visited positions are: {}",
        rope.unique_visited_positions()
    );

    // Part 2
    let mut new_rope = NewRope::new();
    new_rope.start(&movements);
    println!(
        "Unique tail visited position (10 knots rope) are: {}",
        new_rope.unique_visited_positions()
    );

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
        // Part 1
        const INPUT: &str = r#"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2"#;

        let movements: Result<Vec<Movement>> = INPUT
            .lines()
            .map(|line| -> Result<Movement> { Movement::try_from(line) })
            .collect();
        let movements = movements.unwrap();

        let mut rope = Rope::new();
        rope.start(&movements);

        assert_eq!(13, rope.unique_visited_positions());

        // Part 2
        const NEW_INPUT: &str = r#"R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20"#;

        let movements: Result<Vec<Movement>> = NEW_INPUT
            .lines()
            .map(|line| -> Result<Movement> { Movement::try_from(line) })
            .collect();
        let movements = movements.unwrap();

        let mut new_rope = NewRope::new();
        new_rope.start(&movements);

        assert_eq!(36, new_rope.unique_visited_positions());
    }
}
