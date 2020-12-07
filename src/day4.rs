extern crate regex;
extern crate lazy_static;
use self::regex::Regex;
use self::lazy_static::lazy_static;

use std::str::FromStr;
use std::num;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct Passport {
  byr: u32, // byr (Birth Year)
  iyr: u32, // iyr (Issue Year)
  eyr: u32, // eyr (Expiration Year)
  hgt: String, // hgt (Height)
  hcl: String, // hcl (Hair Color)
  ecl: String, // ecl (Eye Color)
  pid: String, // pid (Passport ID)
  cid: u32, // cid (Country ID)
}

pub struct InvalidPassport {
    pub missing: &'static str,
}

pub enum ParsePassportError {
    InvalidHeight,
    InvalidPassport,
    ParseError(num::ParseIntError),
}

impl From<num::ParseIntError> for ParsePassportError {
    fn from(error: num::ParseIntError) -> Self {
        ParsePassportError::ParseError(error)
    }
}

impl FromStr for Passport {
    type Err = ParsePassportError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let passport = s.split(&[' ','\n',][..])
                        .map(|x| {
                            let y = x.split(':').collect::<Vec<&str>>();
                            (String::from(y[0]), String::from(y[1]))
                        })
                        .collect::<HashMap<String, String>>();

        let byr = match passport.get("byr") {
            Some(x) => x,
            None => return Err(ParsePassportError::InvalidPassport),
        }.parse::<u32>()?; // byr (Birth Year)
        let iyr = match passport.get("iyr") {
            Some(x) => x,
            None => return Err(ParsePassportError::InvalidPassport),
        }.parse::<u32>()?; // iyr (Issue Year)
        let eyr = match passport.get("eyr") {
            Some(x) => x,
            None => return Err(ParsePassportError::InvalidPassport),
        }.parse::<u32>()?; // eyr (Expiration Year)
        let hgt = match passport.get("hgt") {
            Some(x) => String::from(x),
            None => return Err(ParsePassportError::InvalidPassport),
        }; // hgt (Height)
        let hcl = match passport.get("hcl") {
            Some(x) => String::from(x),
            None => return Err(ParsePassportError::InvalidPassport),
        }; // hcl (Hair Color)
        let ecl = match passport.get("ecl") {
            Some(x) => String::from(x),
            None => return Err(ParsePassportError::InvalidPassport),
        }; // ecl (Eye Color)
        let pid = match passport.get("pid") {
            Some(x) => String::from(x),
            None => return Err(ParsePassportError::InvalidPassport),
        }; // pid (Passport ID)
        let cid = passport.get("cid").unwrap_or(&String::from("999999")).parse::<u32>()?; // cid (Country ID)

        Ok(Passport{byr, iyr, eyr, hgt, hcl, ecl, pid, cid})
    }
}


#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Vec<Passport> {
    input
        .split("\n\n")
        .map(Passport::from_str)
        .filter_map(Result::ok)
        .collect()
}

#[aoc(day4, part1)]
pub fn part1(passports: &[Passport]) -> usize {
    passports.iter().count()
}

lazy_static! {
  static ref PID_RE: Regex = Regex::new(r"^\d{9}$").unwrap();
  static ref HCL_RE: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
  static ref HGT_RE: Regex = Regex::new(r"^(\d+)(cm|in)$").unwrap();
}

#[allow(clippy::needless_lifetimes)]
pub fn is_valid<'p>(passport: &'p &Passport) -> bool {
    if !(1920..=2002).contains(&passport.byr) {
        return false
    }
    if !(2010..=2020).contains(&passport.iyr) {
        return false
    }
    if !(2020..=2030).contains(&passport.eyr) {
        return false
    }
    if ! ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&passport.ecl.as_str()) {
        return false
    }
    if ! PID_RE.is_match(passport.pid.as_str()) {
        return false
    }
    if ! HCL_RE.is_match(passport.hcl.as_str()) {
        return false
    }
    if let Some(hgt_captures) = HGT_RE.captures(passport.hgt.as_str()) {
        let hgt_value = hgt_captures
            .get(1)
            .unwrap()
            .as_str()
            .parse::<u32>()
            .unwrap();
        match hgt_captures.get(2).unwrap().as_str() {
            "cm" => return (150..=193).contains(&hgt_value),
            "in" => return (59..=76).contains(&hgt_value),
            _ => return false,
        };
    }
    false
}

#[aoc(day4, part2)]
pub fn part2(passports: &[Passport]) -> usize {
    passports
        .iter()
        .filter(is_valid)
        .count()
}

#[cfg(test)]
mod tests {
    use super::{input_generator, part1, part2};

    const SAMPLE_INPUT1: &str = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd\nbyr:1937 iyr:2017 cid:147 hgt:183cm\n\niyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884\nhcl:#cfa07d byr:1929\n\nhcl:#ae17e1 iyr:2013\neyr:2024\necl:brn pid:760753108 byr:1931\nhgt:179cm\n\nhcl:#cfa07d eyr:2025 pid:166559648\niyr:2011 ecl:brn hgt:59in";

    const SAMPLE_INPUT2: &str = "eyr:1972 cid:100\nhcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926\n\niyr:2019\nhcl:#602927 eyr:1967 hgt:170cm\necl:grn pid:012533040 byr:1946\n\nhcl:dab227 iyr:2012\necl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277\n\nhgt:59cm ecl:zzz\neyr:2038 hcl:74454a iyr:2023\npid:3556412378 byr:2007";

    const SAMPLE_INPUT3: &str = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980\nhcl:#623a2f\n\neyr:2029 ecl:blu cid:129 byr:1989\niyr:2014 pid:896056539 hcl:#a97842 hgt:165cm\n\nhcl:#888785\nhgt:164cm byr:2001 iyr:2015 cid:88\npid:545766238 ecl:hzl\neyr:2022\n\niyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";

    #[test]
    fn sample1() {
        assert_eq!(part1(&input_generator(SAMPLE_INPUT1)), 2);
    }

    #[test]
    fn sample2() {
        assert_eq!(part2(&input_generator(SAMPLE_INPUT2)), 0);
        assert_eq!(part2(&input_generator(SAMPLE_INPUT3)), 4);
    }
}
