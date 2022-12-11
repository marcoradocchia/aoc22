use anyhow::{anyhow, Result};
use std::{
    fmt::{self, Display},
    fs,
    process::ExitCode,
};

/// Sprite of 3 pixels.
#[derive(Debug)]
struct Sprite {
    /// Positions of the central pixel of the sprite on the screen (sprite are 3 pixel wide).
    central_pixel: isize,
}

impl Sprite {
    /// Construct a new instance.
    fn new() -> Self {
        Self {
            // Initial position of the central pixel of the sprite must be 1, so the first pixel
            // of the sprite is 0.
            central_pixel: 1,
        }
    }

    /// Check wheter the sprite occupies given pixel.
    fn is_visible(&self, pixel: usize) -> bool {
        [
            self.central_pixel - 1,
            self.central_pixel,
            self.central_pixel + 1,
        ]
        .contains(&(pixel as isize))
    }
}

/// CRT screen.
#[derive(Debug)]
struct Crt {
    /// Pixels of the CRT screen.
    pixels: [bool; 240],
    /// Sprite on the screen.
    sprite: Sprite,
}

impl Crt {
    const CRT_PIXEL_ROWS: usize = 40;

    /// Construct a new instance.
    fn new() -> Self {
        Self {
            pixels: [false; 240], // All pixels initially black.
            sprite: Sprite::new(),
        }
    }

    /// Update position of the central pixel of the [`Sprite`] on the [`Crt`] screen.
    fn update_sprite_central_pixel(&mut self, pixel: isize) -> Result<()> {
        if pixel < -1 || pixel > self.pixels.len() as isize - 2 {
            anyhow::bail!("illegal sprite position");
        }

        self.sprite.central_pixel = pixel;

        Ok(())
    }

    /// Draw pixel on the screen at given index based on the position of the sprite.
    fn update_pixel(&mut self, pixel: usize) -> Result<()> {
        if pixel > self.pixels.len() - 1 {
            anyhow::bail!("invalid pixel index");
        }

        // Determine wheter the sprite is visible while updating the pixel.
        if self.sprite.is_visible(pixel % Self::CRT_PIXEL_ROWS) {
            self.pixels[pixel] = !self.pixels[pixel];
        };

        Ok(())
    }
}

impl Display for Crt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let row_len = self.pixels.len() / 6;
        for (idx, pixel) in self.pixels.iter().enumerate() {
            if idx % row_len == 0 {
                write!(f, "\n")?;
            }
            write!(
                f,
                "{}",
                match pixel {
                    true => "#",  // Pixel on.
                    false => ".", // Pixel off.
                }
            )?;
        }

        Ok(())
    }
}

/// Device's CPU.
#[derive(Debug)]
struct Cpu {
    /// CPU register.
    register: isize,
    /// Total cycle count.
    tot_cycles: usize,
    /// Number of cycles elapsed for current operation.
    elapsed_cycles: usize,
    /// Next [`CpuInstruction`].
    instruction_memory: Option<CpuInstruction>,
    /// Sum of signal strenghts.
    tot_signal_strenght: isize,
    /// CRT screen.
    crt: Crt,
}

impl Cpu {
    /// Construct a new instance.
    fn new() -> Self {
        Self {
            register: 1,
            tot_cycles: 0,
            elapsed_cycles: 0,
            instruction_memory: None,
            tot_signal_strenght: 0,
            crt: Crt::new(),
        }
    }

    /// Perform a CPU cycle.
    fn cycle(&mut self) -> Result<()> {
        self.crt.update_sprite_central_pixel(self.register)?;
        self.crt.update_pixel(self.tot_cycles)?;

        self.tot_cycles += 1;
        self.elapsed_cycles += 1;

        if [20, 60, 100, 140, 180, 220].contains(&self.tot_cycles) {
            self.tot_signal_strenght += self.signal_strenght();
        }

        self.execute()?;

        Ok(())
    }

    /// Execute CPU instruction in `self.instruction_memory`.
    fn execute(&mut self) -> Result<()> {
        // Execute instruction.
        match self.instruction_memory {
            Some(CpuInstruction::Noop) => {}
            Some(CpuInstruction::Addx(i)) => {
                if self.elapsed_cycles < 2 {
                    return Ok(self.cycle()?);
                }
                self.register += i;
            }
            None => anyhow::bail!("no instruction in memory"),
        }

        // Reset instruction memory & elapsed_cycles.
        self.instruction_memory = None;
        self.elapsed_cycles = 0;

        Ok(())
    }

    /// Parse and load the given CPU instruction.
    fn load(&mut self, instruction_string: &str) -> Result<()> {
        self.instruction_memory = Some(CpuInstruction::try_from(instruction_string)?);

        Ok(())
    }

    /// Return signal strenght at current machine state.
    /// Signal strenght is tot_cycles * register.
    fn signal_strenght(&self) -> isize {
        self.tot_cycles as isize * self.register
    }
}

/// CPU instructions.
#[derive(Debug, Clone, Copy)]
enum CpuInstruction {
    Noop,
    Addx(isize),
}

impl TryFrom<&str> for CpuInstruction {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut tokens = value.split_ascii_whitespace();
        Ok(match tokens.next() {
            Some("noop") => Self::Noop,
            Some("addx") => Self::Addx(
                tokens
                    .next()
                    .ok_or(anyhow!(
                        "`addx` instruction must be followed by an argument of type `isize`"
                    ))?
                    .parse::<isize>()?,
            ),
            Some(i) => anyhow::bail!("`{}` is not a valid CPU instruction", i),
            None => anyhow::bail!("empty CPU instruction"),
        })
    }
}

fn run() -> Result<()> {
    let input = fs::read_to_string("./input/day10.dat")?;

    let mut cpu = Cpu::new();
    for instruction_string in input.lines() {
        cpu.load(instruction_string).unwrap();
        cpu.cycle().unwrap();
    }

    // Part 1
    println!(
        "The sum of signal strenghts is: {}",
        cpu.tot_signal_strenght
    );

    // Part 2
    println!("The eight capital letters are: {}", cpu.crt);

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
        const INPUT: &str = r#"addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop"#;

        const PART_TWO_OUTPUT: &str = r#"
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######....."#;

        let mut cpu = Cpu::new();
        for instruction_string in INPUT.lines() {
            cpu.load(instruction_string).unwrap();
            cpu.cycle().unwrap();
        }

        // Part 1
        assert_eq!(13140, cpu.tot_signal_strenght);

        // Part 2
        assert_eq!(PART_TWO_OUTPUT, cpu.crt.to_string());
    }
}
