use advent_of_code_2020::Input;
use std::collections::HashSet;
use std::error;
use std::str::FromStr;
use thiserror::Error;

#[derive(Debug, Error)]
#[error("Invalid answer")]
struct InvalidAnswer;

#[derive(Debug)]
struct GroupAnswers {
    any: HashSet<char>,
    every: HashSet<char>,
}

impl FromStr for GroupAnswers {
    type Err = InvalidAnswer;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let mut any = HashSet::new();
        let mut every = HashSet::new();
        if let Some(line) = lines.next() {
            let answers: String = line.chars().filter(|ch| !ch.is_whitespace()).collect();
            any.extend(answers.chars());
            every.extend(answers.chars());
        }
        for line in lines {
            let answers: String = line.chars().filter(|ch| !ch.is_whitespace()).collect();
            any.extend(answers.chars());
            every.retain(|ch| answers.contains(*ch));
        }
        Ok(Self { any, every })
    }
}

impl GroupAnswers {
    #[allow(dead_code)]
    fn any_answers(&self) -> String {
        let mut answers: Vec<char> = self.any.iter().copied().collect();
        answers.sort_unstable();
        answers.iter().collect()
    }

    fn any_answer_count(&self) -> usize {
        self.any.len()
    }

    #[allow(dead_code)]
    fn every_answers(&self) -> String {
        let mut answers: Vec<char> = self.every.iter().copied().collect();
        answers.sort_unstable();
        answers.iter().collect()
    }

    fn every_answer_count(&self) -> usize {
        self.every.len()
    }
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let answers: Vec<GroupAnswers> = Input::day(6)?
        .iter_parsed_blocks()
        .collect::<Result<_, _>>()?;

    let sum: usize = answers.iter().map(GroupAnswers::any_answer_count).sum();
    println!("Sum of group answer any counts: {}", sum);

    let sum: usize = answers.iter().map(GroupAnswers::every_answer_count).sum();
    println!("Sum of group answer every counts: {}", sum);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: [&str; 6] = [
        "abcx\nabcy\nabcz\n",
        "abc\n",
        "a\nb\nc\n",
        "ab\nac\n",
        "a\na\na\na\n",
        "b\n",
    ];

    #[test]
    fn part_1() {
        let answers = INPUT
            .iter()
            .map(|s| s.parse())
            .collect::<Result<Vec<GroupAnswers>, _>>()
            .unwrap();
        assert_eq!(answers[0].any_answers(), "abcxyz");
        assert_eq!(answers[0].any_answer_count(), 6);
        assert_eq!(answers[1].any_answers(), "abc");
        assert_eq!(answers[1].any_answer_count(), 3);
        assert_eq!(answers[2].any_answers(), "abc");
        assert_eq!(answers[2].any_answer_count(), 3);
        assert_eq!(answers[3].any_answers(), "abc");
        assert_eq!(answers[3].any_answer_count(), 3);
        assert_eq!(answers[4].any_answers(), "a");
        assert_eq!(answers[4].any_answer_count(), 1);
        assert_eq!(answers[5].any_answers(), "b");
        assert_eq!(answers[5].any_answer_count(), 1);
    }

    #[test]
    fn part_2() {
        let answers = INPUT
            .iter()
            .map(|s| s.parse())
            .collect::<Result<Vec<GroupAnswers>, _>>()
            .unwrap();
        assert_eq!(answers[0].every_answers(), "abc");
        assert_eq!(answers[0].every_answer_count(), 3);
        assert_eq!(answers[1].every_answers(), "abc");
        assert_eq!(answers[1].every_answer_count(), 3);
        assert_eq!(answers[2].every_answers(), "");
        assert_eq!(answers[2].every_answer_count(), 0);
        assert_eq!(answers[3].every_answers(), "a");
        assert_eq!(answers[3].every_answer_count(), 1);
        assert_eq!(answers[4].every_answers(), "a");
        assert_eq!(answers[4].every_answer_count(), 1);
        assert_eq!(answers[5].every_answers(), "b");
        assert_eq!(answers[5].every_answer_count(), 1);
    }
}
