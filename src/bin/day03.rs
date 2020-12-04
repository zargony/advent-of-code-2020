use advent_of_code_2020::Input;
use futures::stream::{Stream, TryStreamExt};
use std::{error, io};

#[derive(Debug)]
struct Map {
    area: Vec<Vec<bool>>,
}

impl Map {
    async fn read(lines: impl Stream<Item = io::Result<impl AsRef<str>>>) -> io::Result<Self> {
        let area: Vec<_> = lines
            .and_then(|line| async move {
                line.as_ref()
                    .chars()
                    .map(|ch| match ch {
                        '.' => Ok(false),
                        '#' => Ok(true),
                        ch => Err(io::Error::new(
                            io::ErrorKind::Other,
                            format!("Unexpected map field `{}`", ch),
                        )),
                    })
                    .collect()
            })
            .try_collect()
            .await?;
        Ok(Self { area })
    }

    fn count_trees_on_slope(&self, slope: (usize, usize)) -> usize {
        let mut pos = (0, 0);
        let mut count = 0;
        while pos.0 + slope.0 < self.area.len() {
            pos = (pos.0 + slope.0, pos.1 + slope.1);
            let line = &self.area[pos.0];
            if line[pos.1 % line.len()] {
                count += 1;
            }
        }
        count
    }

    fn product_trees_on_slopes(&self, slopes: &[(usize, usize)]) -> usize {
        slopes
            .iter()
            .map(|slope| self.count_trees_on_slope(*slope))
            .product()
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    let input = Input::day(3).await?;
    let map = Map::read(input.lines()).await?;

    let count = map.count_trees_on_slope((1, 3));
    println!("Trees on slope: {}", count);

    let count = map.product_trees_on_slopes(&[(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)]);
    println!("Multiplied trees on slope: {}", count);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::{stream, StreamExt};

    const INPUT: [&str; 11] = [
        "..##.......",
        "#...#...#..",
        ".#....#..#.",
        "..#.#...#.#",
        ".#...##..#.",
        "..#.##.....",
        ".#.#.#....#",
        ".#........#",
        "#.##...#...",
        "#...##....#",
        ".#..#...#.#",
    ];

    #[tokio::test]
    async fn part_1() {
        let lines = stream::iter(&INPUT).map(Ok);
        let map = Map::read(lines).await.unwrap();
        assert_eq!(map.count_trees_on_slope((1, 3)), 7);
    }

    #[tokio::test]
    async fn part_2() {
        let lines = stream::iter(&INPUT).map(Ok);
        let map = Map::read(lines).await.unwrap();
        assert_eq!(
            map.product_trees_on_slopes(&[(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)]),
            336
        );
    }
}
