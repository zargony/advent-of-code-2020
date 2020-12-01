//! Advent of Code: puzzle input reading

use std::path::PathBuf;
use std::str::FromStr;
use std::{error, io};
use tokio::fs::File;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::stream::{Stream, StreamExt};

/// Path to puzzle input files
const INPUT_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/input");

/// Puzzle input
#[derive(Debug)]
pub struct Input {
    reader: BufReader<File>,
}

impl Input {
    /// Open puzzle input for the given day
    pub async fn day(day: usize) -> io::Result<Self> {
        Self::open(&format!("day{:02}", day)).await
    }

    /// Open puzzle input with the given name
    pub async fn open(name: &str) -> io::Result<Self> {
        let mut filename: PathBuf = INPUT_PATH.into();
        filename.push(name);
        filename.set_extension("txt");
        let reader = BufReader::new(File::open(filename).await?);
        Ok(Input { reader })
    }

    /// Stream of lines
    pub fn lines(self) -> impl Stream<Item = io::Result<String>> {
        self.reader.lines()
    }

    /// Stream of parsed lines
    pub fn parsed_lines<T>(self) -> impl Stream<Item = io::Result<T>>
    where
        T: FromStr,
        T::Err: error::Error + Send + Sync + 'static,
    {
        self.lines().map(|line| {
            line.and_then(|s| {
                s.parse()
                    .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
            })
        })
    }
}
