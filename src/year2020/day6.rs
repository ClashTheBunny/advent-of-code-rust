use std::str::FromStr;
use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq)]
pub struct CustomsForm {
    answers: HashMap<char, u32>,
    answer_sets: HashSet<char>,
}

pub enum ParseCustomsFormError {
    InvalidCustomsForm,
}

impl FromStr for CustomsForm {
    type Err = ParseCustomsFormError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let answers = s.split('\n')
                        .fold(HashMap::new(), |mut form, x| {
                            for ch in x.chars() {
                                let counter = form.entry(ch).or_insert(0);
                                *counter += 1;
                            }
                            form
                        });
        let answer_sets = s.split('\n').enumerate()
                        .fold(HashSet::new(), |form, (i, x)| {
                            let current_chars: HashSet<char> = x.chars().collect();
                            if i == 0 {
                                current_chars
                            } else {
                                form.intersection(&current_chars).cloned().collect::<HashSet<char>>()
                            }
                        });
        Ok(CustomsForm{answers, answer_sets})
    }
}


#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> Vec<CustomsForm> {
    input
        .split("\n\n")
        .map(CustomsForm::from_str)
        .filter_map(Result::ok)
        .collect()
}

#[aoc(day6, part1)]
pub fn part1(forms: &[CustomsForm]) -> usize {
    forms.iter().fold(0, |mut count, x| {
        count += x.answers.keys().count() as usize;
        count
    })
}

#[aoc(day6, part2)]
pub fn part2(forms: &[CustomsForm]) -> usize {
    forms.iter().fold(0, |mut count, x| {
        count += x.answer_sets.iter().count() as usize;
        count
    })
}

#[cfg(test)]
mod tests {
    use super::{input_generator, part1, part2};

    const SAMPLE_INPUT1: &str = "abc\n\na\nb\nc\n\nab\nac\n\na\na\na\na\n\nb";

    #[test]
    fn sample1() {
        assert_eq!(part1(&input_generator(SAMPLE_INPUT1)), 11);
    }

    #[test]
    fn sample2() {
        assert_eq!(part2(&input_generator(SAMPLE_INPUT1)), 6);
    }
}
