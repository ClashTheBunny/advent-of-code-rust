#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> Vec<u64> {
    input.chars().map(|m| m.to_digit(10).unwrap() as u64).collect()
}

// too low 2798413113 - woops, needed bigger numbers, u32 too small to hold answer

#[aoc(day9, part1)]
pub fn part1(input: &[u64]) -> u64 {
    let mut counter = 0;
    // println!("{:?}", input);
    let mut expanded = input.iter().fold(vec![], |mut acc: Vec<u64>, x| {
        if counter % 2 == 0 {
            counter+=1;
            acc.append(& mut [counter/2].repeat((*x).try_into().unwrap()));
            acc
        } else {
            counter+=1;
            acc.append(& mut [u64::MAX].repeat((*x).try_into().unwrap()));
            acc
        }
    });
    while let Some(i) = expanded.iter().position(|&x| x == u64::MAX) {
        expanded.swap_remove(i);
    }
    // println!("{:?}", expanded);
    expanded.iter().enumerate().fold(0u64, |mut acc, (i, x)| {
        acc += x*TryInto::<u64>::try_into(i).unwrap();
        acc
    })
}

#[aoc(day9, part2)]
pub fn part2(_input: &[u64]) -> u64 {
    1
}

#[cfg(test)]
mod tests {
    use super::{input_generator, part1, part2};

    const SAMPLES: &[(&str, u64, u64)] = &[
        (r#"2333133121414131402"#, 1928, 5)];

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
