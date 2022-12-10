use anyhow::{anyhow, Ok, Result};
use std::process::ExitCode;

/// Device's CPU.
struct Cpu {
    register: isize,
}

impl Cpu {
    /// Construct a new instance.
    fn new() -> Self {
        Self { register: 1 }
    }

    /// Execute CPU instruction.
    fn execute(&mut self, instruction: CpuInstruction) {
        match instruction {
            CpuInstruction::Noop => {}
            CpuInstruction::Addx(i) => self.register += i,
        }
    }

    /// Parse and execute the given CPU instruction.
    fn exec(&mut self, instruction_string: &str) -> Result<()> {
        self.execute(CpuInstruction::try_from(instruction_string)?);
        Ok(())
    }
}

/// CPU instructions.
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
    fn example_test() {}
}
