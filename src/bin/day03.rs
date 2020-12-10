use advent_of_code_2020::Input;
use std::error;
use thiserror::Error;

#[derive(Debug, PartialEq, Eq)]
enum Tile {
    Space,
    Tree,
}

#[derive(Debug, Error)]
#[error("Invalid map tile `{0}`")]
struct InvalidMapTile(char);

#[derive(Debug)]
struct Map {
    area: Vec<Vec<Tile>>,
}

impl Map {
    fn parse(lines: &[impl AsRef<str>]) -> Result<Self, InvalidMapTile> {
        let area = lines
            .iter()
            .map(|line| {
                line.as_ref()
                    .chars()
                    .map(|ch| match ch {
                        '.' => Ok(Tile::Space),
                        '#' => Ok(Tile::Tree),
                        ch => Err(InvalidMapTile(ch)),
                    })
                    .collect::<Result<Vec<_>, _>>()
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self { area })
    }

    fn count_trees_on_slope(&self, slope: (usize, usize)) -> usize {
        let mut pos = (0, 0);
        let mut count = 0;
        while pos.0 + slope.0 < self.area.len() {
            pos = (pos.0 + slope.0, pos.1 + slope.1);
            let line = &self.area[pos.0];
            if line[pos.1 % line.len()] == Tile::Tree {
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

fn main() -> Result<(), Box<dyn error::Error>> {
    let map = Map::parse(&Input::day(3)?.lines()?)?;

    let count = map.count_trees_on_slope((1, 3));
    println!("Trees on slope: {}", count);

    let count = map.product_trees_on_slopes(&[(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)]);
    println!("Multiplied trees on slope: {}", count);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn part_1() {
        let map = Map::parse(&INPUT).unwrap();
        assert_eq!(map.count_trees_on_slope((1, 3)), 7);
    }

    #[test]
    fn part_2() {
        let map = Map::parse(&INPUT).unwrap();
        assert_eq!(
            map.product_trees_on_slopes(&[(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)]),
            336
        );
    }
}
