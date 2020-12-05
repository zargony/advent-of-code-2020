use advent_of_code_2020::Input;
use lazy_static::lazy_static;
use regex::Regex;
use std::str::FromStr;
use std::{error, fmt};

/// Split string into two at the given delimiter
fn split1(s: &str, delimiter: char) -> Option<(&str, &str)> {
    let pos = s.find(delimiter)?;
    Some((s[..pos].trim(), s[pos + 1..].trim()))
}

#[derive(Debug)]
struct InvalidPassport;

impl fmt::Display for InvalidPassport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid passport")
    }
}

impl error::Error for InvalidPassport {}

#[derive(Debug)]
struct Passport {
    byr: String,
    iyr: String,
    eyr: String,
    hgt: String,
    hcl: String,
    ecl: String,
    pid: String,
    cid: Option<String>,
}

impl FromStr for Passport {
    type Err = InvalidPassport;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (mut byr, mut iyr, mut eyr, mut hgt, mut hcl, mut ecl, mut pid, mut cid) =
            (None, None, None, None, None, None, None, None);
        for set in s.split_whitespace() {
            let (key, value) = split1(set, ':').ok_or(InvalidPassport)?;
            match key {
                "byr" => byr = Some(value.to_string()),
                "iyr" => iyr = Some(value.to_string()),
                "eyr" => eyr = Some(value.to_string()),
                "hgt" => hgt = Some(value.to_string()),
                "hcl" => hcl = Some(value.to_string()),
                "ecl" => ecl = Some(value.to_string()),
                "pid" => pid = Some(value.to_string()),
                "cid" => cid = Some(value.to_string()),
                _ => return Err(InvalidPassport),
            }
        }
        Ok(Self {
            byr: byr.ok_or(InvalidPassport)?,
            iyr: iyr.ok_or(InvalidPassport)?,
            eyr: eyr.ok_or(InvalidPassport)?,
            hgt: hgt.ok_or(InvalidPassport)?,
            hcl: hcl.ok_or(InvalidPassport)?,
            ecl: ecl.ok_or(InvalidPassport)?,
            pid: pid.ok_or(InvalidPassport)?,
            cid,
        })
    }
}

#[derive(Debug)]
struct StrictPassport {
    byr: String,
    iyr: String,
    eyr: String,
    hgt: String,
    hcl: String,
    ecl: String,
    pid: String,
    cid: Option<String>,
}

impl FromStr for StrictPassport {
    type Err = InvalidPassport;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE_BYR: Regex = Regex::new(r#"^(19[2-9]\d|200[0-2])$"#).unwrap();
            static ref RE_IYR: Regex = Regex::new(r#"^20(1[0-9]|20)$"#).unwrap();
            static ref RE_EYR: Regex = Regex::new(r#"^20(2[0-9]|30)$"#).unwrap();
            static ref RE_HGT: Regex =
                Regex::new(r#"^(1([5-8]\d|9[0-3])cm|(59|6\d|7[0-6])in)$"#).unwrap();
            static ref RE_HCL: Regex = Regex::new(r#"^#[0-9a-f]{6}$"#).unwrap();
            static ref RE_ECL: Regex = Regex::new(r#"^(amb|blu|brn|gry|grn|hzl|oth)$"#).unwrap();
            static ref RE_PID: Regex = Regex::new(r#"^[0-9]{9}$"#).unwrap();
        }
        fn check(re: &Regex, s: String) -> Result<String, InvalidPassport> {
            Some(s).filter(|s| re.is_match(&s)).ok_or(InvalidPassport)
        }

        let passport = Passport::from_str(s)?;
        Ok(Self {
            byr: check(&RE_BYR, passport.byr)?,
            iyr: check(&RE_IYR, passport.iyr)?,
            eyr: check(&RE_EYR, passport.eyr)?,
            hgt: check(&RE_HGT, passport.hgt)?,
            hcl: check(&RE_HCL, passport.hcl)?,
            ecl: check(&RE_ECL, passport.ecl)?,
            pid: check(&RE_PID, passport.pid)?,
            cid: passport.cid,
        })
    }
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let passports: Vec<Result<Passport, _>> = Input::day(4)?.iter_parsed_blocks().collect();
    let count = passports.iter().filter(|p| p.is_ok()).count();
    println!("Valid passports: {}", count);

    let passports: Vec<Result<StrictPassport, _>> = Input::day(4)?.iter_parsed_blocks().collect();
    let count = passports.iter().filter(|p| p.is_ok()).count();
    println!("Valid strict passports: {}", count);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_1: [&str; 4] = [
        "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd\nbyr:1937 iyr:2017 cid:147 hgt:183cm",
        "iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884\nhcl:#cfa07d byr:1929",
        "hcl:#ae17e1 iyr:2013\neyr:2024\necl:brn pid:760753108 byr:1931\nhgt:179cm",
        "hcl:#cfa07d eyr:2025 pid:166559648\niyr:2011 ecl:brn hgt:59in",
    ];

    #[test]
    fn part_1() {
        let passports: Vec<Result<Passport, _>> = INPUT_1.iter().map(|s| s.parse()).collect();
        assert!(passports[0].is_ok());
        assert!(!passports[1].is_ok());
        assert!(passports[2].is_ok());
        assert!(!passports[3].is_ok());
        assert_eq!(passports.iter().filter(|p| p.is_ok()).count(), 2);
    }

    const INPUT_2: [&str; 8] = [
        "eyr:1972 cid:100 hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926",
        "iyr:2019 hcl:#602927 eyr:1967 hgt:170cm ecl:grn pid:012533040 byr:1946",
        "hcl:dab227 iyr:2012 ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277",
        "hgt:59cm ecl:zzz eyr:2038 hcl:74454a iyr:2023 pid:3556412378 byr:2007",
        "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980 hcl:#623a2f",
        "eyr:2029 ecl:blu cid:129 byr:1989 iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm",
        "hcl:#888785 hgt:164cm byr:2001 iyr:2015 cid:88 pid:545766238 ecl:hzl eyr:2022",
        "iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719",
    ];

    #[test]
    fn part_2() {
        let passports: Vec<Result<StrictPassport, _>> = INPUT_2.iter().map(|s| s.parse()).collect();
        assert!(!passports[0].is_ok());
        assert!(!passports[1].is_ok());
        assert!(!passports[2].is_ok());
        assert!(!passports[3].is_ok());
        assert!(passports[4].is_ok());
        assert!(passports[5].is_ok());
        assert!(passports[6].is_ok());
        assert!(passports[7].is_ok());
    }
}
