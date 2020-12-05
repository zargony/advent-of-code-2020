//! Advent of Code: puzzle input reading

use std::error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::PathBuf;
use std::str::FromStr;

/// Path to puzzle input files
const INPUT_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/input");

/// Puzzle input
#[derive(Debug)]
pub struct Input {
    reader: BufReader<File>,
}

impl Input {
    /// Open puzzle input for the given day
    pub fn day(day: usize) -> io::Result<Self> {
        Self::open(&format!("day{:02}", day))
    }

    /// Open puzzle input with the given name
    pub fn open(name: &str) -> io::Result<Self> {
        let mut filename: PathBuf = INPUT_PATH.into();
        filename.push(name);
        filename.set_extension("txt");
        let reader = BufReader::new(File::open(filename)?);
        Ok(Input { reader })
    }

    /// Iterator of lines
    pub fn iter_lines(self) -> impl Iterator<Item = io::Result<String>> {
        self.reader.lines()
    }

    /// Vector of lines
    pub fn lines(self) -> io::Result<Vec<String>> {
        self.iter_lines().collect()
    }

    /// Iterator of parsed lines
    pub fn iter_parsed_lines<T>(self) -> impl Iterator<Item = io::Result<T>>
    where
        T: FromStr,
        T::Err: error::Error + Send + Sync + 'static,
    {
        self.iter_lines().map(|line| {
            line.and_then(|s| {
                s.parse()
                    .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
            })
        })
    }

    /// Vector of parsed lines
    pub fn parsed_lines<T>(self) -> io::Result<Vec<T>>
    where
        T: FromStr,
        T::Err: error::Error + Send + Sync + 'static,
    {
        self.iter_parsed_lines().collect()
    }
}
