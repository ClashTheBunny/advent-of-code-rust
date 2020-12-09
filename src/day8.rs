use std::num::ParseIntError;
use std::str::FromStr;
use std::collections::HashSet;

#[derive(Debug, PartialEq, Clone)]
pub struct Instruction {
    name: String,
    arg: i32,
}

impl FromStr for Instruction {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let arg: i32 = s[4..].parse()?;
        Ok(Instruction{name: String::from(&s[0..3]), arg})
    }
}

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Vec<Instruction> {
    input.lines().map(|x| {
        Instruction::from_str(x)
    }).filter_map(Result::ok).collect()
}

pub fn run_computer(input: &[Instruction]) -> Result<i32, i32> {
    let mut pos = 0;
    let mut global_acc = 0;
    let mut visited = HashSet::new();
    loop {
        if pos >= input.len() {
            return Ok(global_acc);
        }
        if visited.contains(&pos){
            return Err(global_acc);
        }
        visited.insert(pos);
        match input.get(pos).unwrap().name.as_str() {
            "nop" => pos += 1,
            "acc" => { global_acc += input.get(pos).unwrap().arg; pos += 1 },
            "jmp" => pos = ((pos as i32) + input.get(pos).unwrap().arg) as usize,
            _ => panic!(),
        }
    }
}

#[aoc(day8, part1)]
pub fn part1(input: &[Instruction]) -> Option<i32> {
    match run_computer(input) {
        Ok(x) => panic!("Should not be okay, bad input? Resulted in: {}", x),
        Err(x) => Some(x),
    }
}


#[aoc(day8, part2)]
pub fn part2(input: &[Instruction]) -> Option<i32> {
    for (i, instruction) in input.iter().enumerate() {
        if instruction.name == "nop" || instruction.name == "jmp" {
              let mut test_instructions = input.to_vec();
              if let Some(instruction) = test_instructions.get_mut(i) {
                  *instruction = Instruction { name: match instruction.name.as_str() {
                      "nop" => "jmp".to_string(),
                      "jmp" => "nop".to_string(),
                      _ => panic!(),
                  }, arg: instruction.arg };
              }
              if let Ok(x) = run_computer(&test_instructions) {
                  return Some(x);
              }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::{input_generator, part1, part2};

    const SAMPLE_1: &str = "nop +0\nacc +1\njmp +4\nacc +3\njmp -3\nacc -99\nacc +1\njmp -4\nacc +6";

    #[test]
    fn sample1() {
        assert_eq!(part1(&input_generator(SAMPLE_1)), Some(5));
   }
    #[test]
    fn sample2() {
        assert_eq!(part2(&input_generator(SAMPLE_1)), Some(8));
   }
}
