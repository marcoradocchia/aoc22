#![feature(is_sorted)]

use anyhow::Result;
use day1::read_input_lines;
use std::process::ExitCode;

#[derive(Debug)]
/// Pair of elves and their respective [`Range`]s.
struct Pair(Range, Range);

impl TryFrom<&str> for Pair {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let pair = value.split_once(',').ok_or(anyhow::format_err!(
            "unable to find two elves in this group"
        ))?;

        Ok(Self(Range::new(pair.0)?, Range::new(pair.1)?))
    }
}

impl Pair {
    /// Determine whether the range of one of the two elves in the [`Pair`] are fully contained
    /// in the range of the other elf.
    fn fully_contained(&self) -> bool {
        let check_contained = |first: &Range, second: &Range| -> bool {
            first.min <= second.min && second.max <= first.max
        };

        // Check if one is contained in the other or viceversa.
        check_contained(&self.0, &self.1) || check_contained(&self.1, &self.0)
    }

    /// Determine whether the two elves in the [`Group`] have overlapping range.
    fn overlap(&self) -> bool {
        let check_overlap = |first: &Range, second: &Range| -> bool {
            first.min <= second.min && second.min <= first.max
        };

        // Check if one's minimum is contained in the other's range or viceversa.
        check_overlap(&self.0, &self.1) || check_overlap(&self.1, &self.0)
    }
}

#[derive(Debug)]
/// Range of sections IDs.
struct Range {
    min: usize,
    max: usize,
}

impl Range {
    /// Construct a new instance.
    fn new(range: &str) -> Result<Self> {
        let bounds: Result<Vec<usize>> = range
            .splitn(2, '-')
            .map(|bound| -> Result<usize> {
                bound
                    .parse::<usize>()
                    .map_err(|_| anyhow::format_err!("invalid range format"))
            })
            .collect();

        let bounds = bounds?;

        if !bounds.is_sorted() {
            anyhow::bail!("range of IDs for each elf must be expressed as `a-b`, where a <= b")
        };

        Ok(Self {
            min: bounds[0],
            max: bounds[1],
        })
    }
}

fn run() -> Result<()> {
    let lines = read_input_lines("./input/day4.dat")?;

    let elves_pairs: Result<Vec<Pair>> = lines
        .iter()
        .map(|line| -> Result<Pair> { Pair::try_from(line.as_str()) })
        .collect();

    let elves_pairs = elves_pairs?;

    // Part 1
    let fully_contained_count = elves_pairs
        .iter()
        .filter(|pair| pair.fully_contained())
        .count();
    println!("Number of ranges fully contained by other elf's range: {fully_contained_count}");

    // Part 2
    let overlap_count = elves_pairs.iter().filter(|pair| pair.overlap()).count();
    println!("Number of overlapping ranges: {overlap_count}");

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
        let lines = [
            "2-4,6-8", "2-3,4-5", "5-7,7-9", "2-8,3-7", "6-6,4-6", "2-6,4-8",
        ];

        assert!(Pair::try_from(lines[4]).unwrap().fully_contained());
        assert!(!Pair::try_from(lines[5]).unwrap().fully_contained());

        assert_eq!(2, Pair::try_from(lines[0]).unwrap().0.min);
        assert_eq!(4, Pair::try_from(lines[0]).unwrap().0.max);

        assert_eq!(5, Pair::try_from(lines[2]).unwrap().0.min);
        assert_eq!(9, Pair::try_from(lines[2]).unwrap().1.max);

        let elves_pairs: Vec<Pair> = lines
            .iter()
            .map(|line| Pair::try_from(*line).unwrap())
            .collect();

        assert_eq!(
            2,
            elves_pairs
                .iter()
                .filter(|pair| pair.fully_contained())
                .count()
        );

        assert_eq!(4, elves_pairs.iter().filter(|pair| pair.overlap()).count());
    }
}
