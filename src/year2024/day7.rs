use itertools::Itertools;
use itertools::repeat_n;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Calibration {
   result: u64,
   values: Vec<u64>,
}

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> Vec<Calibration> {
    let output: Vec<Calibration> = input.split("\n").map(|x| {
        let result = x.split(": ").nth(0).unwrap().parse::<u64>().unwrap();
        let values = x.split(": ").nth(1).unwrap().split_whitespace().map(|x| x.parse::<u64>().unwrap()).collect();
        Calibration{ result, values }
    }).collect::<Vec<Calibration>>();
    output
}

#[aoc(day7, part1)]
pub fn part1(input: &[Calibration]) -> u64 {
    let mut total_calibration = 0;
    for calibration in input {
        for operations in repeat_n([u64::checked_add,u64::checked_mul].into_iter(), calibration.values.len() - 1).multi_cartesian_product().collect::<Vec<_>>() {
            let mut answer = 0;
            for i in 0..calibration.values.len() {
                if i == 0 {
                    answer = calibration.values[i]
                } else {
                    answer = (operations[i-1])(answer,calibration.values[i]).unwrap();
                }
            }
            if answer == calibration.result {
                total_calibration += answer;
                break;
            }
        }
    }
    total_calibration
}

pub fn concat_u64(first: u64, second: u64) -> Option<u64> {
    Some((first.to_string() + &second.to_string()).parse::<u64>().unwrap())
}

#[aoc(day7, part2)]
pub fn part2(input: &[Calibration]) -> u64 {
    let mut total_calibration = 0;
    for calibration in input {
        for operations in repeat_n([u64::checked_add, u64::checked_mul, concat_u64].into_iter(), calibration.values.len() - 1).multi_cartesian_product().collect::<Vec<_>>() {
            let mut answer = 0;
            for i in 0..calibration.values.len() {
                if i == 0 {
                    answer = calibration.values[i]
                } else {
                    answer = (operations[i-1])(answer,calibration.values[i]).unwrap();
                }
            }
            if answer == calibration.result {
                total_calibration += answer;
                break;
            }
        }
    }
    total_calibration
}

#[cfg(test)]
mod tests {
    use super::{input_generator, part1, part2};

    const SAMPLES: &[(&str, u64, u64)] = &[
        (r#"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"#, 3749, 11387)];

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
