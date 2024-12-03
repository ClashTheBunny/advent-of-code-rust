use itertools::Itertools;


#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Vec<u32>>{
    input.split("\n").map(|y| y.split_whitespace().map(|x| x.parse::<u32>().unwrap() ).collect::<Vec<u32>>()).collect::<Vec<Vec<u32>>>()
}

#[aoc(day2, part1)]
pub fn part1(input: &[Vec<u32>]) -> u32 {
    let mut sum: u32 = 0;

    input.iter().for_each(|x| {
        if x.is_sorted_by(|a, b| (a>b) && (a - b < 4)) {
            sum+=1
        };
        if x.is_sorted_by(|a, b| (a<b) && (b - a < 4)) {
            sum+=1
        };
    });
    sum
}

// too high 591
// too low  472

#[aoc(day2, part2)]
pub fn part2(input: &[Vec<u32>]) -> u32 {
    let mut sum: u32 = 0;

    input.iter().for_each(|x| {
        let mut success: bool = false;
        println!(": {:?}", x);
        for y in x.iter().combinations(x.len()-1) {
            println!("! {:?}", y);
            if y.is_sorted_by(|a, b| (a>b) && (**a - **b < 4)) {
                success = true
            };
            if y.is_sorted_by(|a, b| (a<b) && (**b - **a < 4)) {
                success = true
            };
            if success {
                println!("; {:?}", y);
                println!("");
                break;
            }
        };
        if success {
            sum+=1
        }
    });
    sum
}

#[cfg(test)]
mod tests {
    use super::{input_generator, part1, part2};

    const SAMPLES: &[(&str, u32, u32)] = &[
        (r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"#, 2, 4)];

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
