use anyhow::{Ok, Result};
use day1::read_input_lines;
use std::process::ExitCode;

#[derive(Debug)]
struct Group(Vec<Rucksack>);

impl Group {
    fn new(rucksacks: &[Rucksack]) -> Result<Self> {
        if rucksacks.len() != 3 {
            anyhow::bail!("group is not formed by 3 elves");
        }

        Ok(Self(rucksacks.to_vec()))
    }

    fn badge(&self) -> Result<Item> {
        let rucksack_items: Vec<Vec<Item>> =
            self.0.iter().map(|rucksack| rucksack.items()).collect();

        for item in &rucksack_items[0] {
            if rucksack_items[1].contains(item) && rucksack_items[2].contains(item) {
                return Ok(*item);
            }
        }

        anyhow::bail!("badge not found");
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Item(char);

impl Item {
    fn new(c: char) -> Item {
        Self(c)
    }

    fn priority(&self) -> Result<usize> {
        let priority: usize = match self.0.to_lowercase().collect::<Vec<char>>()[0] {
            'a' => 1,
            'b' => 2,
            'c' => 3,
            'd' => 4,
            'e' => 5,
            'f' => 6,
            'g' => 7,
            'h' => 8,
            'i' => 9,
            'j' => 10,
            'k' => 11,
            'l' => 12,
            'm' => 13,
            'n' => 14,
            'o' => 15,
            'p' => 16,
            'q' => 17,
            'r' => 18,
            's' => 19,
            't' => 20,
            'u' => 21,
            'v' => 22,
            'w' => 23,
            'x' => 24,
            'y' => 25,
            'z' => 26,
            _ => anyhow::bail!("rucksack contains unexpected item"),
        };

        Ok(match self.0.is_uppercase() {
            true => priority + 26,
            false => priority,
        })
    }
}

#[derive(Debug, Clone)]
struct Rucksack(Vec<Item>, Vec<Item>);

impl Rucksack {
    fn new(items: &str) -> Result<Self> {
        let item_count = items.len();

        if item_count % 2 != 0 {
            anyhow::bail!("number of items in a rucksack must be even");
        }

        let items: Vec<char> = items.chars().collect();

        Ok(Rucksack(
            items[..item_count / 2]
                .iter()
                .map(|c| Item::new(*c))
                .collect(),
            items[item_count / 2..]
                .iter()
                .map(|c| Item::new(*c))
                .collect(),
        ))
    }

    /// Find shared item in the two compartments and return its priority.
    /// If Rucksack compartments have no items, return Err.
    fn find_shared_item(&self) -> Result<usize> {
        for item in &self.0 {
            if self.1.contains(item) {
                return item.priority();
            }
        }

        anyhow::bail!("rucksack is empty")
    }

    fn items(&self) -> Vec<Item> {
        [self.0.clone(), self.1.clone()].concat()
    }
}

fn run() -> Result<()> {
    let rucksacks: Result<Vec<Rucksack>> = read_input_lines("./input/day3.dat")?
        .iter()
        .map(|line| -> Result<Rucksack> { Rucksack::new(line) })
        .collect();

    let rucksacks = rucksacks?;

    // Part 1
    let priorities: Result<Vec<usize>> = rucksacks
        .iter()
        .map(|rucksack| rucksack.find_shared_item())
        .collect();
    println!(
        "Total priorities are: {}",
        priorities?.iter().sum::<usize>()
    );

    // Part 2
    let badges: Result<Vec<usize>> = rucksacks
        .chunks(3)
        .map(|group| -> Result<usize> { Group::new(group)?.badge()?.priority() })
        .collect();
    println!(
        "Total badge priorities are: {}",
        badges?.iter().sum::<usize>()
    );

    Ok(())
}

fn main() -> ExitCode {
    if let Err(e) = run() {
        eprintln!("{e}");
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}

#[cfg(test)]
mod test {
    use super::{Group, Rucksack};
    use anyhow::Result;

    #[test]
    fn example_test_day3() {
        let lines = [
            "vJrwpWtwJgWrhcsFMMfFFhFp",
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
            "PmmdzqPrVvPwwTWBwg",
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
            "ttgJtRGJQctTZtZT",
            "CrZsJsPPZsGzwwsLwLmpwMDw",
        ];

        let rucksacks: Result<Vec<Rucksack>> = lines
            .iter()
            .map(|line| -> Result<Rucksack> { Rucksack::new(line) })
            .collect();

        let rucksacks = rucksacks.unwrap();

        let priorities: Result<Vec<usize>> = rucksacks
            .iter()
            .map(|rucksack| rucksack.find_shared_item())
            .collect();
        assert_eq!(157, priorities.unwrap().iter().sum::<usize>());

        let badges: Result<Vec<usize>> = rucksacks
            .chunks(3)
            .map(|group| -> Result<usize> { Group::new(group)?.badge()?.priority() })
            .collect();
        assert_eq!(70, badges.unwrap().iter().sum::<usize>());
    }
}
