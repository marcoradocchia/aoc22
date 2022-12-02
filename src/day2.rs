use anyhow::Result;
use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
    process::ExitCode,
};

#[derive(Debug)]
/// Turn shapes.
struct Turn {
    player: Shape,
    opponent: Shape,
}

impl TryFrom<&str> for Turn {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let chars: Vec<char> = value.chars().collect();
        if value.len() != 3 || chars[1] != ' ' {
            anyhow::bail!("input contains invalid turn format")
        };

        Ok(Self {
            player: Shape::player(chars[2])?,
            opponent: Shape::opponent(chars[0])?,
        })
    }
}

impl Turn {
    fn try_from_part_two(value: &str) -> Result<Self> {
        let chars: Vec<char> = value.chars().collect();
        if value.len() != 3 || chars[1] != ' ' {
            anyhow::bail!("input contains invalid turn format")
        };

        Ok(Self {
            player: Shape::player_part_two(chars[2], chars[0])?,
            opponent: Shape::opponent(chars[0])?,
        })
    }
}

/// Player.
struct Player {
    score: usize,
}

impl Player {
    fn new() -> Self {
        Player { score: 0 }
    }

    fn play(&mut self, player: &Shape, opponent: &Shape) {
        self.score += Into::<usize>::into(Outcome::new(*player, *opponent));
        self.score += Into::<usize>::into(*player);
    }
}

#[derive(Debug, PartialEq, Eq)]
/// Turn possible outcomes.
enum Outcome {
    Win,
    Draw,
    Lose,
}

impl Outcome {
    fn new(player: Shape, opponent: Shape) -> Outcome {
        match (player, opponent) {
            (Shape::Rock, Shape::Scissor) => Outcome::Win,
            (Shape::Rock, Shape::Paper) => Outcome::Lose,

            (Shape::Paper, Shape::Rock) => Outcome::Win,
            (Shape::Paper, Shape::Scissor) => Outcome::Lose,

            (Shape::Scissor, Shape::Paper) => Outcome::Win,
            (Shape::Scissor, Shape::Rock) => Outcome::Lose,

            _ => Outcome::Draw,
        }
    }
}

impl From<Outcome> for usize {
    /// Convert [`Outcome`] into points.
    fn from(value: Outcome) -> Self {
        match value {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Lose => 0,
        }
    }
    
}

impl TryFrom<char> for Outcome {
    type Error = anyhow::Error;

    /// Returns the outcome matching the Elf hints rules.
    /// X -> LOSE
    /// Y -> DRAW
    /// Z -> WIN
    fn try_from(c: char) -> Result<Self> {
        Ok(match c {
            'X' => Outcome::Lose,
            'Y' => Outcome::Draw,
            'Z' => Outcome::Win,
            s => anyhow::bail!("'{}' is not a valid sign for outcome", s),
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Game signs.
enum Shape {
    Rock,
    Paper,
    Scissor,
}

impl From<Shape> for usize {
    /// Convert [`Shape`] into points.
    fn from(value: Shape) -> Self {
        match value {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissor => 3,
        }
    }
}

impl Shape {
    /// Takes the opponent's shape and returns the shape the player needs to play based on the
    /// desierd outcome.
    fn from_outcome(opponent: Shape, outcome: Outcome) -> Self {
        match (outcome, opponent) {
            (Outcome::Draw, shape) => shape,

            (Outcome::Win, Self::Rock) => Self::Paper,
            (Outcome::Lose, Self::Rock) => Self::Scissor,

            (Outcome::Win, Shape::Paper) => Self::Scissor,
            (Outcome::Lose, Shape::Paper) => Self::Rock,

            (Outcome::Win, Shape::Scissor) => Self::Rock,
            (Outcome::Lose, Shape::Scissor) => Self::Paper,
        }
    }

    fn opponent(o: char) -> Result<Self> {
        Ok(match o {
            'A' => Self::Rock,
            'B' => Self::Paper,
            'C' => Self::Scissor,
            s => anyhow::bail!("'{}' is not a valid sign for opponent", s),
        })
    }

    fn player(p: char) -> Result<Self> {
        Ok(match p {
            'X' => Self::Rock,
            'Y' => Self::Paper,
            'Z' => Self::Scissor,
            s => anyhow::bail!("'{}' is not a valid sign for player", s),
        })
    }

    fn player_part_two(p: char, o: char) -> Result<Self> {
        Ok(Self::from_outcome(
            Self::opponent(o)?,
            Outcome::try_from(p)?,
        ))
    }
}

fn game(lines: &[String]) -> Result<Player> {
    let turns: Result<Vec<Turn>> = lines
        .iter()
        .map(|line| -> Result<Turn> { Turn::try_from(line.as_str()) })
        .collect();

    let mut player = Player::new();
    turns?
        .iter()
        .for_each(|turn| player.play(&turn.player, &turn.opponent));

    Ok(player)
}

fn game_part_two(lines: &[String]) -> Result<Player> {
    let turns: Result<Vec<Turn>> = lines
        .iter()
        .map(|line| -> Result<Turn> { Turn::try_from_part_two(line.as_str()) })
        .collect();

    let mut player = Player::new();
    turns?
        .iter()
        .for_each(|turn| player.play(&turn.player, &turn.opponent));

    Ok(player)
}

fn read_input_lines<P>(path: P) -> Result<Vec<String>, io::Error>
where
    P: AsRef<Path>,
{
    let input_file = File::open(path)?;
    BufReader::new(input_file).lines().collect()
}

fn run() -> Result<()> {
    let lines = read_input_lines("./input/day2.dat")?;

    // Part 1
    let player = game(&lines)?;
    println!("Part 1: Player scored {} points", player.score);

    // Part 2
    // let player
    let player = game_part_two(&lines)?;
    println!("Part 2: Player scored {} points", player.score);

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
    use super::{game, game_part_two, Outcome, Shape};

    #[test]
    fn compare_shapes() {
        assert_eq!(Outcome::new(Shape::Rock, Shape::Scissor), Outcome::Win);
        assert_eq!(Outcome::new(Shape::Rock, Shape::Paper), Outcome::Lose);
        assert_eq!(Outcome::new(Shape::Rock, Shape::Rock), Outcome::Draw);

        assert_eq!(Outcome::new(Shape::Paper, Shape::Rock), Outcome::Win);
        assert_eq!(Outcome::new(Shape::Paper, Shape::Scissor), Outcome::Lose);
        assert_eq!(Outcome::new(Shape::Paper, Shape::Paper), Outcome::Draw);

        assert_eq!(Outcome::new(Shape::Scissor, Shape::Paper), Outcome::Win);
        assert_eq!(Outcome::new(Shape::Scissor, Shape::Rock), Outcome::Lose);
        assert_eq!(Outcome::new(Shape::Scissor, Shape::Scissor), Outcome::Draw);

        assert_ne!(Outcome::new(Shape::Scissor, Shape::Rock), Outcome::Win);
        assert_ne!(Outcome::new(Shape::Paper, Shape::Paper), Outcome::Lose);
        assert_ne!(Outcome::new(Shape::Rock, Shape::Paper), Outcome::Win);
    }

    #[test]
    fn test_example() {
        let input = ["A Y", "B X", "C Z"];

        let lines: Vec<String> = input
                .iter()
                .map(|line| line.to_string())
                .collect();

        let player = game(&lines).unwrap();
        assert_eq!(player.score, 15);

        let player = game_part_two(&lines).unwrap();
        assert_eq!(player.score, 12);
    }
}
