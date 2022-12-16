use anyhow::{Ok, Result};
use std::{
    fmt::{self, Display},
    fs,
    process::ExitCode,
};

const SOURCE: Point = Point { x: 500, y: 0 };

/// Abyss kind.
enum Abyss {
    Void,
    Floor,
}

/// Point on the rock slice.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    /// Construct a new instance.
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    /// Traslate point.
    fn traslate(&mut self, x: isize, y: isize) -> Result<()> {
        self.x = self
            .x
            .checked_add_signed(x)
            .ok_or_else(|| anyhow::format_err!("invalid traslation"))?;
        self.y = self
            .y
            .checked_add_signed(y)
            .ok_or_else(|| anyhow::format_err!("invalid traslation"))?;

        Ok(())
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl TryFrom<&str> for Point {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (x, y) = value
            .split_once(',')
            .ok_or_else(|| anyhow::format_err!("missing coordinates"))?;

        Ok(Point::new(
            x.parse::<usize>()
                .map_err(|_| anyhow::format_err!("coordinates must be unsigned integers"))?,
            y.parse::<usize>()
                .map_err(|_| anyhow::format_err!("coordinates must be unsigned integers"))?,
        ))
    }
}

/// Path of rock.
#[derive(Debug, Clone)]
struct RockPath {
    /// Vertices of the rock paths.
    verts: Vec<Point>,
}

impl RockPath {
    /// Construct a new instance.
    fn new(points: &[Point]) -> Self {
        Self {
            verts: points.to_vec(),
        }
    }

    /// Check wheter `self` contains [`Point`].
    fn contains(&self, point: Point) -> bool {
        let mut idx: usize = 0;
        while idx < self.verts.len() - 1 {
            if self.verts[idx].x == self.verts[idx + 1].x && self.verts[idx].x == point.x {
                // Vertices are vertically aligned.
                if (self.verts[idx].y <= point.y && point.y <= self.verts[idx + 1].y)
                    || (self.verts[idx + 1].y <= point.y && point.y <= self.verts[idx].y)
                {
                    return true;
                }
            } else if self.verts[idx].y == self.verts[idx + 1].y && self.verts[idx].y == point.y {
                // Vertices are horizontaly aligned.
                if (self.verts[idx].x <= point.x && point.x <= self.verts[idx + 1].x)
                    || (self.verts[idx + 1].x <= point.x && point.x <= self.verts[idx].x)
                {
                    return true;
                }
            }

            idx += 1;
        }

        false
    }
}

/// Cave slice.
#[derive(Debug, Clone)]
struct CaveSlice {
    /// Rock paths.
    rock_paths: Vec<RockPath>,
    /// Max y coordinate before the void.
    max_y: usize,
    /// Falling grain of sand.
    falling: SandGrain,
    /// Deposited grains of sand.
    sand: Vec<SandGrain>,
}

impl CaveSlice {
    /// Construct a new instance.
    fn new(paths: &[RockPath]) -> Self {
        let mut max_y: usize = 0;
        for path in paths {
            for point in &path.verts {
                if point.y > max_y {
                    max_y = point.y;
                }
            }
        }

        Self {
            rock_paths: paths.to_vec(),
            max_y,
            falling: SandGrain::new(),
            sand: vec![],
        }
    }

    /// Calculates next valid position for falling grain of sand and updates `falling` grain
    /// position accordingly. Return `FallingState`.
    fn fall(&mut self, abyss_kind: &Abyss) -> Result<FallingState> {
        for move_option in [
            MoveOption::Down,
            MoveOption::DownLeft,
            MoveOption::DownRight,
        ] {
            // Check if virtual position is a valid position.
            let virtual_position = self.falling.virtual_position(move_option)?;
            let mut occupied = false;

            // Check if deposited grain of sand occupies position.
            // Iterate over deposited grains reverse because sand accumulates from bottom to top,
            // so it will take less iteations to find a deposited grain reversing.
            for sand_grain in self.sand.iter().rev() {
                if virtual_position == sand_grain.position {
                    occupied = true;
                }
            }

            // Check if rock path occupies position.
            if self.sand.is_empty() || !occupied {
                for path in &self.rock_paths {
                    if path.contains(virtual_position) {
                        occupied = true;
                    }
                }
            }

            // Virtual position is a valid position.
            if !occupied {
                return Ok(if virtual_position.y < self.max_y {
                    self.falling.update(virtual_position);
                    FallingState::Falling
                } else {
                    match abyss_kind {
                        Abyss::Void => FallingState::IntoTheVoid,
                        Abyss::Floor => {
                            if virtual_position.y >= self.max_y + 2 {
                                self.sand.push(self.falling);
                                FallingState::Deposited
                            } else {
                                self.falling.update(virtual_position);
                                FallingState::Falling
                            }
                        }
                    }
                });
            }
        }

        if self.falling.position == SOURCE {
            self.sand.push(self.falling);
            return Ok(FallingState::Blocking);
        }

        // No valid virtual position found. The grain of sand reached its rest state: append
        // `self.falling` to `self.sand`.
        self.sand.push(self.falling);
        Ok(FallingState::Deposited)
    }

    /// Count how many units of sand come to rest before start falling into the void or source
    /// blocked.
    fn count_sand_grains(&mut self, abyss_kind: Abyss) -> Result<usize> {
        loop {
            let fall_result = self.fall(&abyss_kind)?;
            match fall_result {
                FallingState::Falling => {}
                FallingState::Deposited => self.falling = SandGrain::new(),
                FallingState::IntoTheVoid | FallingState::Blocking => break,
            }
        }

        Ok(self.sand.len())
    }
}

impl TryFrom<&str> for CaveSlice {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(CaveSlice::new(
            &value
                .trim()
                .split('\n')
                .map(|line| -> Result<RockPath> {
                    Ok(RockPath::new(
                        &line
                            .replace(' ', "")
                            .split("->")
                            .map(|coordinates| -> Result<Point> { Point::try_from(coordinates) })
                            .collect::<Result<Vec<Point>>>()?,
                    ))
                })
                .collect::<Result<Vec<RockPath>>>()?,
        ))
    }
}

/// Sand grain dropping down the cave slice.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct SandGrain {
    position: Point,
}

impl SandGrain {
    /// Construct a new instance.
    fn new() -> Self {
        // The sand is pouring into the cave from point 500,0.
        Self { position: SOURCE }
    }

    /// Update `self` position.
    fn update(&mut self, point: Point) {
        self.position = point;
    }

    /// Return sand grain virtual position.
    fn virtual_position(&self, move_option: MoveOption) -> Result<Point> {
        let mut virtual_position = self.position;

        match move_option {
            MoveOption::Down => virtual_position.traslate(0, 1)?,
            MoveOption::DownLeft => virtual_position.traslate(-1, 1)?,
            MoveOption::DownRight => virtual_position.traslate(1, 1)?,
        };

        Ok(virtual_position)
    }
}

/// Move options of the grain of sand.
#[derive(Debug)]
enum MoveOption {
    Down,
    DownLeft,
    DownRight,
}

/// Falling state of the grain of sand.
#[derive(Debug)]
enum FallingState {
    /// Grain of sand falling.
    Falling,
    /// Grain of sand deposited.
    Deposited,
    /// Grain of sand into the void.
    IntoTheVoid,
    /// Grain of sand blocks sand source.
    Blocking,
}

fn run() -> Result<()> {
    let input = fs::read_to_string("./input/day14.dat")?;

    // Part 1
    let mut cave_slice = CaveSlice::try_from(input.as_str())?;
    println!(
        "Number of deposited grains of sand before falling into the abyss is: {}",
        cave_slice.count_sand_grains(Abyss::Void)?
    );

    // Part 2
    let mut cave_slice = CaveSlice::try_from(input.as_str())?;
    println!(
        "Number of deposited grains of sand before blocking the sand source is: {}",
        cave_slice.count_sand_grains(Abyss::Floor)?
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
        const INPUT: &str = r#"498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9"#;

        let mut cave_slice = CaveSlice::try_from(INPUT).unwrap();
        assert_eq!(24, cave_slice.count_sand_grains(Abyss::Void).unwrap());
        assert_eq!(93, cave_slice.count_sand_grains(Abyss::Floor).unwrap());
    }
}
