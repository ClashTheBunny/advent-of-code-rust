use std::string::String;
use itertools::Itertools;

#[derive(Debug)]
pub enum Op {
    Null,
    Number(i64),
    String(String),
    Array(Vec<Op>),
}

use self::Op::Array;
use self::Op::String as Str;

#[aoc_generator(day18)]
pub fn input_generator(input: &str) -> Op {
   let (left, tmp) = input.splitn(2,'(').collect_tuple().unwrap();
   let (right, middle) = tmp.rsplitn(2,')').collect_tuple().unwrap();
   println!("Input: {:?}", input);
   println!("left {:?} middle {:?} right {:?}", left, middle, right);
   match (left, middle, right) {
       ("", _, "") => panic!("Should not have empty parens!"),
       (l , m, "") => Array(vec![Str(l.to_string()),input_generator(m)]),
       ("", m,  r) => Array(vec![input_generator(m),Str(r.to_string())]),
       (_ , _,  _) => panic!("Should not happen"),
   }
}

#[aoc(day18, part1)]
pub fn part1(input: &Op) -> usize {
    println!("{:?}", input);
    71
}

// #[aoc(day18, part2)]
pub fn part2(_input: &Op) -> usize {
    1
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::{input_generator, part1, part2};

    const SAMPLES: &[(&str, usize)] = &[
        ("1 + (2 * 3) + (4 * (5 + 6))", 51),
        ("1 + 2 * 3 + 4 * 5 + 6", 71),
        ("2 * 3 + (4 * 5)", 26),
        ("5 + (8 * 3 + 9 + 3 * 4 * 3)", 437),
        ("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))", 12240),
        ("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2", 13632),
    ];

    #[test]
    fn sample1() {
        for (sample, answer) in SAMPLES.to_vec() {
            assert_eq!(part1(&input_generator(sample)), answer);
        }
   }
    #[test]
    fn sample2() {
        // assert_eq!(part2(&input_generator("")), 336);
   }
}
