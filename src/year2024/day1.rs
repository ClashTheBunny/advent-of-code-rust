#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<Vec<u32>>{
    input.split("\n").map(|y| y.split_whitespace().map(|x| x.parse::<u32>().unwrap() ).collect::<Vec<u32>>()).collect::<Vec<Vec<u32>>>()
}

#[aoc(day1, part1)]
pub fn part1(input: &[Vec<u32>]) -> u32 {
    let mut left: Vec<u32> = vec![];
    let mut right: Vec<u32> = vec![];
    let mut sum: u32 = 0;

    input.iter().for_each(|x| {
        left.push(x[0]);
        right.push(x[1]);
    });
    left.sort();
    right.sort();
    for (i, value) in left.iter().enumerate() {
        sum += value.abs_diff(right[i])
    }
    sum
}

#[aoc(day1, part2)]
pub fn part2(input: &[Vec<u32>]) -> u32 {
    let mut left: Vec<u32> = vec![];
    let mut right: Vec<u32> = vec![];
    let mut sum: u32 = 0;

    input.iter().for_each(|x| {
        left.push(x[0]);
        right.push(x[1]);
    });
    left.sort();
    right.sort();
    for value in left.iter() {
        sum += value*u32::try_from(right.iter().filter(|n| *n == value).count()).unwrap();
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::{input_generator, part1, part2};

    const SAMPLES: &[(&str, u32, u32)] = &[
        (r#"3   4
4   3
2   5
1   3
3   9
3   3"#, 11, 31)];

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
