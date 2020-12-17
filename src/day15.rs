use std::collections::HashMap;

#[aoc_generator(day15)]
pub fn input_generator(input: &str) -> Vec<u32>{
    input.split(',').map(|x| x.parse::<u32>().unwrap() ).collect::<Vec<u32>>()
}

fn find_answer(input: &[u32], rounds: usize) -> usize {
    let mut input_map: HashMap<usize, usize> = HashMap::new();
    for (i, x) in input.iter().enumerate() {
        input_map.insert(*x as usize, i);
    }
    let mut i = input.len();
    let mut last_spoken: usize = 0;
    loop {
        if let Some(x) = input_map.get(&last_spoken) {
            let x = *x;
            input_map.insert(last_spoken, i);
            last_spoken = i - x;
        } else {
            input_map.insert(last_spoken,i);
            last_spoken = 0;
        }
        i+=1;
        if i >= rounds - 1 {
            break
        }
    }
    last_spoken
}

#[aoc(day15, part1)]
pub fn part1(input: &[u32]) -> usize {
    find_answer(input, 2020)
}

#[aoc(day15, part2)]
pub fn part2(input: &[u32]) -> usize {
    find_answer(input, 30000000)
}

#[cfg(test)]
mod tests {
    use super::{input_generator, part1, part2};

    const SAMPLES: &[(&str, usize)] = &[
        ("0,3,6", 436),
        ("1,3,2", 1),
        ("2,1,3", 10),
        ("1,2,3", 27),
        ("2,3,1", 78),
        ("3,2,1", 438),
        ("3,1,2", 1836),
    ];

    const SAMPLES_2: &[(&str, usize)] = &[
        ("0,3,6", 175594),
        ("1,3,2", 2578),
        ("2,1,3", 3544142),
        ("1,2,3", 261214),
        ("2,3,1", 6895259),
        ("3,2,1", 18),
        ("3,1,2", 362),
    ];

    #[test]
    fn sample1() {
        for (sample, answer) in SAMPLES {
            assert_eq!(part1(&input_generator(sample)), *answer);
        }
   }
    #[test]
    fn sample2() {
        for (sample, answer) in SAMPLES_2 {
            assert_eq!(part2(&input_generator(sample)), *answer);
        }
   }
}
