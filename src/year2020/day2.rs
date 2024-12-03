use std::str::FromStr;
use std::num::ParseIntError;

#[derive(Debug, PartialEq)]
pub struct PasswordAndPolicy {
  min: u32,
  max: u32,
  letter: char,
  password: String,
}

impl FromStr for PasswordAndPolicy {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec<&str> = s.split(&[':',' ','-',][..])
                                 .collect();

        let min = coords[0].parse::<u32>()?;
        let max = coords[1].parse::<u32>()?;
        let letter = match char::from_str(coords[2]) {
            Ok(x) => x,
            Err(_) => panic!(),
        };
        let password = String::from(coords[4]);

        Ok(PasswordAndPolicy{ min, max, letter, password})
    }
}


#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<PasswordAndPolicy> {
    input.lines().map(|x|
        PasswordAndPolicy::from_str(x).unwrap()
    ).collect()
}

#[aoc(day2, part1)]
pub fn part1(passwords: &[PasswordAndPolicy]) -> i32 {
    passwords.iter().fold(0, |mut valid, x| {
        let count = x.password.matches(x.letter).count();
        if (x.min <= count as u32) && (x.max >= count as u32) {
            valid+=1
        }
        valid
    })
}

pub fn nth_char(string: &str, loc: usize) -> char {
    char::from_str(&string[loc-1..=loc-1]).unwrap()
}

#[aoc(day2, part2)]
pub fn part2(passwords: &[PasswordAndPolicy]) -> i32 {
    passwords.iter().fold(0, |mut valid, x| {
        if (nth_char(&x.password[..], x.min as usize) == x.letter) ^ (nth_char(&x.password[..], x.max as usize) == x.letter) {
            valid += 1
        }
        valid
    })
}

#[cfg(test)]
mod tests {
    use super::{input_generator, part1, part2};

    #[test]
    fn sample1() {
        assert_eq!(part1(&input_generator("1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc")), 2);
    }

    #[test]
    fn sample2() {
        assert_eq!(part2(&input_generator("1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc")), 1);
    }

    //too high:
    //491
}
