use std::{
    fmt::Display,
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

fn read_input_lines<P>(path: P) -> Result<Vec<String>, io::Error>
where
    P: AsRef<Path>,
{
    let input_file = File::open(path)?;
    BufReader::new(input_file).lines().collect()
}

#[derive(Debug)]
struct Elf {
    idx: usize,
    cals: usize,
}

impl Elf {
    fn new(idx: usize, cals: usize) -> Self {
        Self { idx, cals }
    }
}

impl Default for Elf {
    fn default() -> Self {
        Elf::new(0, 0)
    }
}

impl Display for Elf {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Elf #{} carries {} cals", self.idx, self.cals)
    }
}

/// Returns a vector of elves, sorted by cals.
fn elves_cals(lines: &[String]) -> Vec<Elf> {
    let mut elfs: Vec<Elf> = Default::default();
    let mut idx: usize = 1;
    let mut cals: usize = 0;

    for line in lines {
        if line.is_empty() {
            elfs.push(Elf::new(idx, cals));
            idx += 1;
            cals = 0;
            continue;
        }

        cals += line.parse::<usize>().unwrap();
    }

    // Inverted sort by cals.
    elfs.sort_by(|a, b| {
        b.cals
            .partial_cmp(&a.cals)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    elfs
}

fn main() -> io::Result<()> {
    let input_lines = read_input_lines("./input/day1.dat")?;
    let elves_cals = elves_cals(&input_lines);

    // Part 1 & Part 2
    println!(
        "Top three Elves carry a total of {} calories",
        elves_cals
            .iter()
            .take(3)
            .map(|elf| {
                println!("{}", elf);
                elf.cals
            })
            .sum::<usize>()
    );

    Ok(())
}

#[cfg(test)]
mod test {
    use super::elves_cals;

    #[test]
    fn test_on_example() {
        let lines = [
            "1000", "2000", "3000", "", "4000", "", "5000", "6000", "", "7000", "8000", "9000", "",
            "10000",
        ];

        let elf = &elves_cals(&lines.iter().map(|str| str.to_string()).collect::<Vec<_>>())[0];

        assert_eq!(4, elf.idx);
        assert_eq!(24000, elf.cals);
    }
}
