extern crate regex;
extern crate lazy_static;
use self::regex::Regex;
use self::lazy_static::lazy_static;
use std::num::ParseIntError;
use std::str::FromStr;

lazy_static! {
  static ref MUL_RE: Regex = Regex::new(r"(do|don't|mul)\((\d+,\d+|)\)").unwrap();
}

#[derive(Debug, PartialEq)]
pub struct Instruction {
    op: String,
    arg0: Option<u32>,
    arg1: Option<u32>,
}

impl FromStr for Instruction {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split_answer: Vec<&str> = s.split(&['(', ',', ')',][..]).collect();
        match split_answer[0] {
            "mul" => Ok(Instruction{arg0: Some(split_answer[1].parse::<u32>().unwrap()), arg1: Some(split_answer[2].parse::<u32>().unwrap()), op: String::from(split_answer[0])}),
            _ => Ok(Instruction{arg0: None, arg1: None, op: String::from(split_answer[0])}),
        }
    }
}

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<Instruction> {
    MUL_RE.find_iter(input).map(|y|
        Instruction::from_str(y.as_str())
    ).filter_map(Result::ok).collect()
}

#[aoc(day3, part1)]
pub fn part1(input: &[Instruction]) -> u32 {
    input.iter().filter(|instruction| instruction.op == "mul").map(|instruction| instruction.arg0.unwrap() * instruction.arg1.unwrap()).sum()
}

#[aoc(day3, part2)]
pub fn part2(input: &[Instruction]) -> u32 {
    let mut process = true;
    let mut sum = 0;
    for instruction in input {
        match instruction.op.as_str() {
          "don't" => process = false,
          "do" => process = true,
          "mul" => if process { sum += instruction.arg1.unwrap() * instruction.arg0.unwrap() },
          _ => panic!("oops"),
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::{input_generator, part1, part2};

    const SAMPLES: &[(&str, u32, u32)] = &[
        (r#"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"#, 161, 161),
        (r#"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"#, 161, 48)];

    #[test]
    fn sample1() {
        for (sample, answer, _) in SAMPLES {
            assert_eq!(part1(&input_generator(sample)), *answer);
        }
   }
    #[test]
    fn sample2() {
        for (sample, _, answer) in SAMPLES {
            assert_eq!(part2(&input_generator(sample)), *answer);
        }
   }
}
