use std::num::ParseIntError;

#[aoc_generator(day13)]
pub fn input_generator(input: &str) -> (u64, Vec<Result<u64, ParseIntError>>) {
    let mut lines = input.lines();
    let time = lines.next().unwrap().parse::<u64>().unwrap();
    let schedule = lines.next().unwrap().split(',').map(|x| x.parse::<u64>()).collect::<Vec<Result<u64, ParseIntError>>>();
    (time, schedule)
}

#[aoc(day13, part1)]
pub fn part1(input: &(u64, Vec<Result<u64, ParseIntError>>)) -> u64 {
    let analysis = input.1.iter().filter_map(|x| Result::ok(x.as_ref())).map(|bus| {
        (bus, (1 +(input.0 / bus)) * bus)
    }).min_by_key(|(_,v)| *v).unwrap();
    (analysis.1 - input.0) * analysis.0
}

#[aoc(day13, part2)]
pub fn part2(input: &(u64, Vec<Result<u64, ParseIntError>>)) -> u64 {
    for (i, number) in input.1.iter().enumerate() {
        if let Ok(x) = number {
            println!("(x + {})%{} == 0", i, x);
        }
    }
    3
}

#[cfg(test)]
mod tests {
    use super::{input_generator, part1, part2};

    const SAMPLE_1: &str = "939\n7,13,x,x,59,x,31,19";
    //wrong: 23

    #[test]
    fn sample1() {
        assert_eq!(part1(&input_generator(SAMPLE_1)), 295);
   }
    #[test]
    fn sample2() {
        assert_eq!(part2(&input_generator(SAMPLE_1)), 1068781);
   }
}
