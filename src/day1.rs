// --- Day 1: Calorie Counting ---
// Santa's reindeer typically eat regular reindeer food, but they need a lot of magical energy to deliver presents on Christmas. For that, their favorite snack is a special type of star fruit that only grows deep in the jungle. The Elves have brought you on their annual expedition to the grove where the fruit grows.
//
// To supply enough magical energy, the expedition needs to retrieve a minimum of fifty stars by December 25th. Although the Elves assure you that the grove has plenty of fruit, you decide to grab any fruit you see along the way, just in case.
//
// Collect stars by solving puzzles. Two puzzles will be made available on each day in the Advent calendar; the second puzzle is unlocked when you complete the first. Each puzzle grants one star. Good luck!
//
// The jungle must be too overgrown and difficult to navigate in vehicles or access from the air; the Elves' expedition traditionally goes on foot. As your boats approach land, the Elves begin taking inventory of their supplies. One important consideration is food - in particular, the number of Calories each Elf is carrying (your puzzle input).
//
// The Elves take turns writing down the number of Calories contained by the various meals, snacks, rations, etc. that they've brought with them, one item per line. Each Elf separates their own inventory from the previous Elf's inventory (if any) by a blank line.
//
// For example, suppose the Elves finish writing their items' Calories and end up with the following list:
//
// 1000
// 2000
// 3000
//
// 4000
//
// 5000
// 6000
//
// 7000
// 8000
// 9000
//
// 10000
// This list represents the Calories of the food carried by five Elves:
//
// The first Elf is carrying food with 1000, 2000, and 3000 Calories, a total of 6000 Calories.
// The second Elf is carrying one food item with 4000 Calories.
// The third Elf is carrying food with 5000 and 6000 Calories, a total of 11000 Calories.
// The fourth Elf is carrying food with 7000, 8000, and 9000 Calories, a total of 24000 Calories.
// The fifth Elf is carrying one food item with 10000 Calories.
// In case the Elves get hungry and need extra snacks, they need to know which Elf to ask: they'd like to know how many Calories are being carried by the Elf carrying the most Calories. In the example above, this is 24000 (carried by the fourth Elf).
//
// Find the Elf carrying the most Calories. How many total Calories is that Elf carrying?

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
