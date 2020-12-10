use advent_of_code_2020::Input;
use std::str::FromStr;
use std::{error, fmt};

#[derive(Debug)]
struct InvalidSeat;

impl fmt::Display for InvalidSeat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid seat")
    }
}

impl error::Error for InvalidSeat {}

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq)]
struct SeatNumber(usize);

impl FromStr for SeatNumber {
    type Err = InvalidSeat;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        let mut row = 0..128;
        for _ in 0..7 {
            match chars.next() {
                Some('F') => row.end = row.end - row.len() / 2,
                Some('B') => row.start = row.start + row.len() / 2,
                _ => return Err(InvalidSeat),
            }
        }
        let mut column = 0..8;
        for _ in 0..3 {
            match chars.next() {
                Some('L') => column.end = column.end - column.len() / 2,
                Some('R') => column.start = column.start + column.len() / 2,
                _ => return Err(InvalidSeat),
            }
        }
        if row.len() != 1 || column.len() != 1 {
            Err(InvalidSeat)
        } else {
            Ok(Self(row.start * 8 + column.start))
        }
    }
}

impl SeatNumber {
    const fn id(&self) -> usize {
        self.0
    }

    #[allow(dead_code)]
    const fn row(&self) -> usize {
        self.id() / 8
    }

    #[allow(dead_code)]
    const fn column(&self) -> usize {
        self.id() % 8
    }
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let mut seats: Vec<SeatNumber> = Input::day(5)?.parsed_lines()?;

    let max = seats.iter().max_by_key(|s| s.id()).unwrap().id();
    println!("Highest seat id: {}", max);

    seats.sort();
    let my = (1..seats.len())
        .find(|i| seats[*i - 1].id() + 2 == seats[*i].id())
        .map(|i| seats[i - 1].id() + 1)
        .unwrap();
    println!("My seat id: {}", my);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let seat: SeatNumber = "FBFBBFFRLR".parse().unwrap();
        assert_eq!(seat.row(), 44);
        assert_eq!(seat.column(), 5);
        assert_eq!(seat.id(), 357);

        let seat: SeatNumber = "BFFFBBFRRR".parse().unwrap();
        assert_eq!(seat.row(), 70);
        assert_eq!(seat.column(), 7);
        assert_eq!(seat.id(), 567);

        let seat: SeatNumber = "FFFBBBFRRR".parse().unwrap();
        assert_eq!(seat.row(), 14);
        assert_eq!(seat.column(), 7);
        assert_eq!(seat.id(), 119);

        let seat: SeatNumber = "BBFFBBFRLL".parse().unwrap();
        assert_eq!(seat.row(), 102);
        assert_eq!(seat.column(), 4);
        assert_eq!(seat.id(), 820);
    }
}
