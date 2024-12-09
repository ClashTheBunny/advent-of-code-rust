#[aoc_generator(dayX)]
pub fn input_generator(_input: &str) -> i32 {
    3
}

#[aoc(dayX, part1)]
pub fn part1(_input: &i32) -> u32 {
    2
}

#[aoc(dayX, part2)]
pub fn part2(_input: &i32) -> u32 {
    1
}

#[cfg(test)]
mod tests {
    use super::{input_generator, part1, part2};

    const SAMPLES: &[(&str, u32, u32)] = &[
        (r#"multilineInputString"#, 4, 5)];

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
