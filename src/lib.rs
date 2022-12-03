use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

pub fn read_input_lines<P>(path: P) -> Result<Vec<String>, io::Error>
where
    P: AsRef<Path>,
{
    let input_file = File::open(path)?;
    BufReader::new(input_file).lines().collect()
}
