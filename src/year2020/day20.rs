#[aoc_generator(day20)]
pub fn input_generator(input: &str) -> i32 {
    let puzzles = input.split("\n\n").collect::<Vec<&str>>();
    3
}

#[aoc(day20, part1)]
pub fn part1(input: &i32) -> usize {
    2
}

#[aoc(day20, part2)]
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
