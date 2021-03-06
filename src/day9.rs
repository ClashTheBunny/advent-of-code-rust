use itertools::Itertools;
use itertools::MinMaxResult::MinMax;
use std::cmp::Ordering;

#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> Vec<u64> {
    input.lines().map(|x| { x.parse().unwrap() }).collect()

}

#[aoc(day9, part1)]
pub fn part1(input: &[u64]) -> u64 {
    let window_length: usize = match input.len() {
        20 => 5,
        _ => 25,
    };
    for (i, window) in input.windows(window_length).enumerate() {
        let sumvar = input[window_length+i];
        let valid_sumvar = window.iter().any(|x| { window.contains(&((sumvar as i64 - *x as i64) as u64)) } );
        if ! valid_sumvar {
            return sumvar
        }
    }
    6
}

#[aoc(day9, part2)]
pub fn part2(input: &[u64]) -> u64 {
    let input_len = input.len();
    let first_invalid = part1(input);
    for start in 0..(input_len-1) {
        for window_len in 0..(input_len - start) {
            let local_range = &input[start..(start + window_len)];
            let local_sum = local_range.iter().cloned().sum::<u64>();
            match local_sum.cmp(&first_invalid) {
                Ordering::Equal => {
                    return match local_range.iter().minmax() {
                        MinMax(x,y) => x + y,
                        _ => 0,
                    }
                },
                Ordering::Greater => break,
                Ordering::Less => {},
            }
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::{input_generator, part1, part2};

    const SAMPLE_1: &str = "35\n20\n15\n25\n47\n40\n62\n55\n65\n95\n102\n117\n150\n182\n127\n219\n299\n277\n309\n576";

    #[test]
    fn sample1() {
        assert_eq!(part1(&input_generator(SAMPLE_1)), 127);
    }

    #[test]
    fn sample2() {
        assert_eq!(part2(&input_generator(SAMPLE_1)), 62);
   }
}
