use anyhow::Result;
use std::{fs, process::ExitCode};

/// Sequence type.
#[derive(Clone, Copy)]
enum Sequence {
    Packet,
    Message,
}

impl From<Sequence> for usize {
    fn from(value: Sequence) -> Self {
        match value {
            Sequence::Packet => 4,
            Sequence::Message => 14,
        }
    }
}

/// DataStream Buffer
#[derive(Debug)]
struct StreamBuffer {
    chars: Vec<char>,
}

impl From<&str> for StreamBuffer {
    fn from(value: &str) -> Self {
        Self {
            chars: value.chars().collect(),
        }
    }
}

impl StreamBuffer {
    /// Check if sequence is a valid start-of-{packet,message} marker.
    ///
    /// # Note
    /// In order to be a valid start-of-{packet,message} marker the {4,14} chars sequence must
    /// not to have a duplicate character.
    fn check_marker(sequence: &[char]) -> bool {
        for (idx, val) in sequence.iter().enumerate() {
            if sequence[idx + 1..].contains(val) {
                return false;
            }
        }

        true
    }

    /// Return sequence of 4 characters starting at index.
    fn get_seq(&self, idx: usize, n: usize) -> Option<&[char]> {
        self.chars.get(idx..idx + n)
    }

    /// Return the number of characters to be processed before encountering the first
    /// [`Sequence`] marker (start-of-packet | start-of-message).
    fn chars_before(&self, sequence: Sequence) -> Option<usize> {
        let sequence_len = sequence.into();
        let mut idx: usize = 0;
        while let Some(slice) = self.get_seq(idx, sequence_len) {
            if Self::check_marker(slice) {
                return Some(idx + sequence_len);
            }
            idx += 1;
        }

        None
    }
}

fn run() -> Result<()> {
    let input = fs::read_to_string("./input/day6.dat")?;

    let stream = StreamBuffer::try_from(input.as_str())?;

    // Part 1
    if let Some(chars_num) = stream.chars_before(Sequence::Packet) {
        println!("First packet marker found after character: {chars_num}");
    } else {
        println!("No packet marker found");
    }

    // Part 2
    if let Some(chars_num) = stream.chars_before(Sequence::Message) {
        println!("First message marker found after character: {chars_num}");
    } else {
        println!("No message marker found");
    }

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
        const TEST_INPUT: [(&str, usize, usize); 5] = [
            ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7, 19),
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5, 23),
            ("nppdvjthqldpwncqszvftbrmjlhg", 6, 23),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10, 29),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11, 26),
        ];


        for (stream, packet, message) in TEST_INPUT {
            let stream = StreamBuffer::try_from(stream).unwrap();
            assert_eq!(stream.chars_before(Sequence::Packet).unwrap(), packet);
            assert_eq!(stream.chars_before(Sequence::Message).unwrap(), message);
        }
    }
}
