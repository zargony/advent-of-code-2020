use advent_of_code_2020::Input;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::error;
use std::str::FromStr;
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Bag(String);

impl From<&str> for Bag {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

impl PartialEq<&str> for Bag {
    fn eq(&self, other: &&str) -> bool {
        self.0 == *other
    }
}

#[derive(Debug, Error)]
#[error("Invalid rule")]
struct InvalidRule;

#[derive(Debug)]
struct Rule {
    bag: Bag,
    contains: Vec<(usize, Bag)>,
}

impl FromStr for Rule {
    type Err = InvalidRule;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE_RULE: Regex = Regex::new(
                r#"^(\w+ \w+) bags contain (no other bags|(\d \w+ \w+ bags?(, )?)+)\.$"#
            )
            .unwrap();
            static ref RE_CONTAINS: Regex = Regex::new(r#"(\d) (\w+ \w+) bags?"#).unwrap();
        }

        let caps = RE_RULE.captures(s).ok_or(InvalidRule)?;
        let bag = Bag::from(caps.get(1).ok_or(InvalidRule)?.as_str());
        let s2 = caps.get(2).ok_or(InvalidRule)?.as_str();
        let contains: Vec<_> = RE_CONTAINS
            .captures_iter(s2)
            .map(|caps| {
                let count = caps.get(1).and_then(|m| m.as_str().parse::<usize>().ok());
                let bag = caps.get(2).map(|m| Bag::from(m.as_str()));
                if let (Some(c), Some(b)) = (count, bag) {
                    Ok((c, b))
                } else {
                    Err(InvalidRule)
                }
            })
            .collect::<Result<_, _>>()?;
        Ok(Self { bag, contains })
    }
}

#[derive(Debug)]
struct RuleSet {
    contains: HashMap<Bag, Vec<(usize, Bag)>>,
    contained: HashMap<Bag, HashSet<Bag>>,
}

impl From<Vec<Rule>> for RuleSet {
    fn from(rules: Vec<Rule>) -> Self {
        let mut contains = HashMap::new();
        let mut contained = HashMap::new();
        for rule in rules {
            for (_c, b) in &rule.contains {
                contained
                    .entry(b.clone())
                    .or_insert_with(|| HashSet::new())
                    .insert(rule.bag.clone());
            }
            contains.insert(rule.bag, rule.contains);
        }
        Self {
            contains,
            contained,
        }
    }
}

impl RuleSet {
    fn which_contains(&self, bag: &str, deep: bool) -> HashSet<Bag> {
        let mut bags = self
            .contained
            .get(&Bag::from(bag))
            .map(Clone::clone)
            .unwrap_or(HashSet::new());
        if deep {
            let deep_bags = bags
                .iter()
                .map(|bag| self.which_contains(&bag.0, true))
                .fold(HashSet::new(), |mut db, b| {
                    db.extend(b);
                    db
                });
            bags.extend(deep_bags);
        }
        bags
    }

    fn count_bags(&self, bag: &str) -> usize {
        self.contains
            .get(&Bag::from(bag))
            .map(|bags| {
                bags.iter()
                    .map(|(c, b)| c * self.count_bags(&b.0))
                    .sum::<usize>()
                    + 1
            })
            .unwrap_or(0)
    }
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let rules: Vec<Rule> = Input::day(7)?.parsed_lines()?;
    let rules = RuleSet::from(rules);

    let count = rules.which_contains("shiny gold", true).len();
    println!(
        "Number of bags that can eventually contain at least one shiny gold bag: {}",
        count
    );

    let count = rules.count_bags("shiny gold") - 1;
    println!(
        "Number of individual bags inside single shiny gold bag: {}",
        count
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_1: [&str; 9] = [
        "light red bags contain 1 bright white bag, 2 muted yellow bags.",
        "dark orange bags contain 3 bright white bags, 4 muted yellow bags.",
        "bright white bags contain 1 shiny gold bag.",
        "muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.",
        "shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.",
        "dark olive bags contain 3 faded blue bags, 4 dotted black bags.",
        "vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.",
        "faded blue bags contain no other bags.",
        "dotted black bags contain no other bags.",
    ];

    #[test]
    fn part_1() {
        let rules: Vec<Rule> = INPUT_1
            .iter()
            .map(|s| s.parse())
            .collect::<Result<_, _>>()
            .unwrap();

        assert_eq!(rules[0].bag, "light red");
        assert_eq!(rules[0].contains[0].0, 1);
        assert_eq!(rules[0].contains[0].1, "bright white");
        assert_eq!(rules[0].contains[1].0, 2);
        assert_eq!(rules[0].contains[1].1, "muted yellow");
        assert_eq!(rules[7].bag, "faded blue");
        assert_eq!(rules[7].contains, []);

        let rules = RuleSet::from(rules);
        assert_eq!(rules.which_contains("shiny gold", true).len(), 4);
    }

    const INPUT_2: [&str; 7] = [
        "shiny gold bags contain 2 dark red bags.",
        "dark red bags contain 2 dark orange bags.",
        "dark orange bags contain 2 dark yellow bags.",
        "dark yellow bags contain 2 dark green bags.",
        "dark green bags contain 2 dark blue bags.",
        "dark blue bags contain 2 dark violet bags.",
        "dark violet bags contain no other bags.",
    ];

    #[test]
    fn part_2() {
        let rules: Vec<Rule> = INPUT_2
            .iter()
            .map(|s| s.parse())
            .collect::<Result<_, _>>()
            .unwrap();
        let rules = RuleSet::from(rules);

        assert_eq!(rules.count_bags("shiny gold") - 1, 126);
    }
}
