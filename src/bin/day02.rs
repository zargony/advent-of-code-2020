use advent_of_code_2020::Input;
use std::error;
use std::str::FromStr;
use thiserror::Error;

/// Split string into two at the given delimiter
fn split1(s: &str, delimiter: char) -> Option<(&str, &str)> {
    let pos = s.find(delimiter)?;
    Some((s[..pos].trim(), s[pos + 1..].trim()))
}

#[derive(Debug, Error)]
#[error("Password parse error")]
struct PasswordParseError;

#[derive(Debug)]
struct Password {
    min: usize,
    max: usize,
    ch: char,
    password: String,
}

impl FromStr for Password {
    type Err = PasswordParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (rule, password) = split1(s, ':').ok_or(PasswordParseError)?;
        let (range, ch) = split1(rule, ' ').ok_or(PasswordParseError)?;
        let (min, max) = split1(range, '-').ok_or(PasswordParseError)?;
        Ok(Self {
            min: min.parse().map_err(|_e| PasswordParseError)?,
            max: max.parse().map_err(|_e| PasswordParseError)?,
            ch: ch.chars().next().ok_or(PasswordParseError)?,
            password: password.into(),
        })
    }
}

impl Password {
    fn is_valid(&self) -> bool {
        (self.min..=self.max).contains(&self.password.matches(self.ch).count())
    }

    fn is_valid_new(&self) -> bool {
        self.password
            .chars()
            .enumerate()
            .filter(|(i, ch)| [self.min, self.max].contains(&(*i + 1)) && *ch == self.ch)
            .count()
            == 1
    }
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let passwords: Vec<Password> = Input::day(2)?.parsed_lines()?;

    let count = passwords.iter().filter(|p| p.is_valid()).count();
    println!("Number of valid password (old rules): {}", count);

    let count = passwords.iter().filter(|p| p.is_valid_new()).count();
    println!("Number of valid password (new rules): {}", count);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: [&str; 3] = ["1-3 a: abcde", "1-3 b: cdefg", "2-9 c: ccccccccc"];

    #[test]
    fn splitting() {
        assert_eq!(split1("foo: bar", ':'), Some(("foo", "bar")));
    }

    #[test]
    fn part_1() {
        let passwords: Vec<Password> = INPUT
            .iter()
            .map(|s| s.parse())
            .collect::<Result<_, _>>()
            .unwrap();
        assert!(passwords[0].is_valid());
        assert!(!passwords[1].is_valid());
        assert!(passwords[2].is_valid());
        assert_eq!(passwords.iter().filter(|p| p.is_valid()).count(), 2);
    }

    #[test]
    fn part_2() {
        let passwords: Vec<Password> = INPUT
            .iter()
            .map(|s| s.parse())
            .collect::<Result<_, _>>()
            .unwrap();
        assert!(passwords[0].is_valid_new());
        assert!(!passwords[1].is_valid_new());
        assert!(!passwords[2].is_valid_new());
        assert_eq!(passwords.iter().filter(|p| p.is_valid_new()).count(), 1);
    }
}
