use anyhow::Result;
use std::{fs, process::ExitCode};

/// CrateMover model.
#[derive(Debug)]
enum CrateMover {
    CrateMover9000,
    CrateMover9001,
}

impl CrateMover {
    /// Whether the [`CrateMover`] model can move multiple crates at the same time.
    fn multiple_crates(&self) -> bool {
        match *self {
            CrateMover::CrateMover9000 => true,
            CrateMover::CrateMover9001 => false,
        }
    }
}

/// Ship's cargo crane.
#[derive(Debug)]
struct Crane {
    /// CrateMover model.
    model: CrateMover,
    /// Storage configuration: list of stacks.
    storage: Storage,
    /// Crane's rearrangement procedure: sequence of moves.
    procedure: Procedure,
}

impl Crane {
    /// Construct a new instance.
    fn new(model: CrateMover, storage: Storage, procedure: Procedure) -> Self {
        Self { model, storage, procedure }
    }

    /// Consumes the crane object, applying the procedure and returning the new [`Storage`] state.
    fn execute_procedure(mut self) -> Result<Storage> {
        for m in self.procedure.moves {
            let moved_crates = self
                .storage
                .get_stack(m.origin)
                .ok_or(anyhow::format_err!("required origin stack does not exist"))?
                .pop_crates(m.amount)
                .ok_or(anyhow::format_err!("invalid instructions in procedure"))?;

            self.storage
                .get_stack(m.destination)
                .ok_or(anyhow::format_err!(
                    "required destination stack does not exist"
                ))?
                .append_stack(moved_crates, self.model.multiple_crates());
        }

        Ok(self.storage)
    }
}

#[derive(Debug)]
struct Procedure {
    moves: Vec<Move>,
}

impl Procedure {
    fn new(moves: Vec<Move>) -> Self {
        Self { moves }
    }
}

impl TryFrom<&str> for Procedure {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let moves: Result<Vec<Move>> = value
            .split('\n')
            .filter(|line| !line.is_empty())
            .map(|move_instruction| -> Result<Move> { Move::try_from(move_instruction) })
            .collect();

        Ok(Self::new(moves?))
    }
}

/// Crane move.
#[derive(Debug)]
struct Move {
    /// Number of [`Crate`]s to move.
    amount: usize,
    /// Stack index moving from.
    origin: usize,
    /// Stack index moving to.
    destination: usize,
}

impl Move {
    fn new(amount: usize, origin: usize, destination: usize) -> Self {
        Self {
            amount,
            origin,
            destination,
        }
    }
}

impl TryFrom<&str> for Move {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        // Example line to parse: 'move 6 from 5 to 7'
        let mut keywords = ["move", "from", "to"].iter();
        let values: Result<Vec<Option<usize>>> = value
            .split_ascii_whitespace()
            .enumerate()
            .map(|(idx, val)| -> Result<Option<usize>> {
                if idx % 2 == 0 {
                    if &val
                        != keywords
                            .next()
                            .ok_or(anyhow::format_err!("unexpected keyword `{}`", val))?
                    {
                        anyhow::bail!("badly formatted move instruction")
                    } else {
                        return Ok(None);
                    }
                }

                Ok(Some(val.parse()?))
            })
            .collect();

        let values: Vec<usize> = values?.iter().filter_map(|val| *val).collect();

        Ok(Move::new(values[0], values[1], values[2]))
    }
}

/// Storage configuration.
#[derive(Debug)]
struct Storage {
    /// Stacks in the storage.
    stacks: Vec<Stack>,
}

impl TryFrom<&str> for Storage {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        // Example:
        //     [D]
        // [N] [C]
        // [Z] [M] [P]
        //  1   2   3
        let mut rev_lines: Vec<&str> = value.split('\n').rev().collect();
        let size = rev_lines
            .remove(0)
            .split_ascii_whitespace()
            .last()
            .ok_or(anyhow::format_err!(
                "storage must contain at least one stack"
            ))?
            .parse::<usize>()
            .map_err(|_| anyhow::format_err!("unable to retrieve storage size"))?;

        let mut stacks: Vec<Stack> = Vec::with_capacity(size);

        for line in rev_lines {
            let mut idx = 0;
            let mut a = 0;

            let mut line_chars = line.chars();
            while let Some(c) = line_chars.nth(1) {
                if a % 2 != 0 {
                    a += 1;
                    continue;
                }

                if c != ' ' {
                    let c = Crate::new(c);
                    if let Some(stack) = stacks.get_mut(idx) {
                        stack.append_crate(c);
                    } else {
                        stacks.push(Stack::new(c));
                    }
                };

                idx += 1;
                a += 1;
            }
        }

        Ok(Self { stacks })
    }
}

impl Storage {
    /// Return a mutable reference to `nth` [`Stack`] in the [`Storage`] or
    /// `None` if the index is out of bounds (indexing from 1).
    fn get_stack(&mut self, n: usize) -> Option<&mut Stack> {
        self.stacks.get_mut(n - 1)
    }

    /// Return the sequence of the top crates of each stack.
    fn top_crates_sequence(&self) -> String {
        self.stacks
            .iter()
            .map(|stack| stack.items.last().unwrap_or(&Crate::new(' ')).0)
            .collect()
    }
}

/// Storage stack of [`Crate`]s.
#[derive(Debug)]
struct Stack {
    /// Crates collected in the stack.
    items: Vec<Crate>,
}

impl Stack {
    /// Pop the last n [`Crate`]s in the stack and return them.
    fn pop_crates(&mut self, n: usize) -> Option<Vec<Crate>> {
        match self.items.len() >= n {
            true => Some(self.items.split_off(self.items.len() - n)),
            false => None,
        }
    }

    /// Append [`Stack`] to the top of the stack.
    fn append_stack(&mut self, mut crates: Vec<Crate>, reverse: bool) {
        if reverse {
            crates.reverse();
        }

        self.items.append(&mut crates);
    }

    /// Construct a new instance.
    fn new(c: Crate) -> Self {
        Self { items: vec![c] }
    }

    /// Append [`Crate`] to Stack
    fn append_crate(&mut self, c: Crate) {
        self.items.push(c)
    }
}

/// Storage Crate.
#[derive(Debug)]
struct Crate(char);

impl Crate {
    /// Construct a new instance.
    fn new(c: char) -> Self {
        Self(c)
    }
}

fn run() -> Result<()> {
    let input = fs::read_to_string("./input/day5.dat")?;

    let (storage_configuration, procedure_instructions) = input
        .split_once("\n\n")
        .ok_or(anyhow::format_err!("invalid input format"))?;

    // Part 1
    let storage = Crane::new(
        CrateMover::CrateMover9000,
        Storage::try_from(storage_configuration)?,
        Procedure::try_from(procedure_instructions)?,
    )
    .execute_procedure()?;
    println!(
        "The sequence of the top crates of each stack for CrateMover9000 is: {}",
        storage.top_crates_sequence()
    );

    // Part 2
    let storage = Crane::new(
        CrateMover::CrateMover9001,
        Storage::try_from(storage_configuration)?,
        Procedure::try_from(procedure_instructions)?,
    )
    .execute_procedure()?;
    println!(
        "The sequence of the top crates of each stack for CrateMover9001 is: {}",
        storage.top_crates_sequence()
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
        let procedure_instructions = r#"move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"#;

        let storage_configuration = r#"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 "#;

        let storage = Crane::new(
            CrateMover::CrateMover9000,
            Storage::try_from(storage_configuration).unwrap(),
            Procedure::try_from(procedure_instructions).unwrap(),
        )
        .execute_procedure().unwrap();
        assert_eq!("CMZ", storage.top_crates_sequence());

        let storage = Crane::new(
            CrateMover::CrateMover9001,
            Storage::try_from(storage_configuration).unwrap(),
            Procedure::try_from(procedure_instructions).unwrap(),
        )
        .execute_procedure().unwrap();
        assert_eq!("MCD", storage.top_crates_sequence());
    }
}
