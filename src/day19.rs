use std::collections::{HashMap, HashSet};
use std::ops::RangeInclusive;
use std::iter::FromIterator;

#[derive(Debug)]
pub struct Input {
   rules: HashMap<u32, Vec<String>>,
   messages: Vec<String>,
}

fn parse_rules(rules: &str) -> HashMap<u32, Vec<String>> {
    let mut valid_numbers = HashMap::new();
    for line in rules.lines() {
        let mut range: Vec<RangeInclusive<u32>> = vec![];
        let mut entry: &str = "";
        for (i, chunk) in line.split(": ").enumerate() {
            match (i, chunk) {
                (0, _) => entry = chunk,
                (1..=usize::MAX, "or") => {},
                (1..=usize::MAX, _) => {
                    chunk.split(" or ").for_each(|chunk| {
                        let start_end = &mut chunk
                            .split('-')
                            .map(|x| x.parse::<u32>().unwrap() )
                            .collect::<Vec<u32>>();
                        range.push(start_end[0]..=start_end[1]);
                    });
                },
                (_, _) => panic!("{:?}", chunk),
            }
        }
        valid_numbers.insert(String::from(entry), range);
    }
    valid_numbers
}

fn parse_message(ticket_line: &str) -> Vec<String> {
    ticket_line.split(',').map(|x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>()
}

#[aoc_generator(day19)]
pub fn input_generator(input: &str) -> Input {
    let mut rules = HashMap::new();
    let mut messages = vec![];
    for (i, chunk) in input.split("\n\n").enumerate() {
        match i {
            0 => rules = parse_rules(chunk), // valid numbers
            1 => messages = parse_message(chunk.split('\n').unwrap()), // my ticket
        }
    }
    Input{ rules, messages}
}

#[aoc(day19, part1)]
pub fn part1(input: &Input) -> usize {
    2
}

#[aoc(day19, part2)]
pub fn part2(input: &Input) -> usize {
    1
}

#[cfg(test)]
mod tests {
    use super::{input_generator, part1, part2};

    #[test]
    fn sample1() {
        assert_eq!(part1(&input_generator("")), 7);
   }
    #[test]
    fn sample2() {
        assert_eq!(part2(&input_generator("")), 336);
   }
}
