#[aoc_generator(day)]
pub fn input_generator(input: &str) -> i32 {
    3
}

#[aoc(day, part1)]
pub fn part1(input: &i32) -> usize {
    2
}

#[aoc(day, part2)]
pub fn part2(input: &i32) -> usize {
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
