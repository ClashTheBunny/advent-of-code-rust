use std::str::FromStr;
use std::cell::RefCell;
use std::num::ParseIntError;
use std::collections::HashMap;
use itertools::Itertools;

#[derive(Debug, PartialEq)]
pub struct Bitmask {
    mask: String,
    mem_loc: u64,
    mem_val: u64,
    mem_val_masked: u64,
    mem_loc_masked: u64,
}

impl FromStr for Bitmask {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {

        let mask = CURRENT_MASK.with(|cur_mask| {
            cur_mask.borrow().clone()
        }).unwrap();

        let ones_mask = mask.chars().map(|x| { match x {
            'X' => '0',
            '1' => '1',
            '0' => '0',
            _ => panic!(),
        }}).collect::<String>();
        let zero_mask = mask.chars().map(|x| { match x {
            'X' => '1',
            '1' => '1',
            '0' => '0',
            _ => panic!(),
        }}).collect::<String>();

        let ones_mask_u64 = u64::from_str_radix(&ones_mask, 2).unwrap();
        let zero_mask_u64 = u64::from_str_radix(&zero_mask, 2).unwrap();
        let answer_sets: Vec<&str> = s.split("] = ").collect();
        let mem_loc = answer_sets[0].split('[').collect::<Vec<&str>>()[1].parse::<u64>()?;
        let mem_val = answer_sets[1].parse::<u64>()?;
        Ok(Bitmask{mask, mem_loc, mem_val, mem_val_masked: mem_val & zero_mask_u64 | ones_mask_u64, mem_loc_masked: mem_val | ones_mask_u64})
    }
}

thread_local!(static CURRENT_MASK: RefCell<Option<String>> = RefCell::new(None));

#[aoc_generator(day14)]
pub fn input_generator(input: &str) -> Vec<Bitmask> {
    let mut masks: Vec<Result<Bitmask, ParseIntError>> = Vec::new();
    for line in input.lines() {
            match &line[0..4] {
                "mask" => {
                    CURRENT_MASK.with(|current_mask| {
                        *current_mask.borrow_mut() = Some(line[7..].to_string())
                    });
                },
                _ => masks.push(Bitmask::from_str(line)),
            }
    }
    masks.into_iter().map(|x| { Result::ok(x).unwrap() }).collect()

}

#[aoc(day14, part1)]
pub fn part1(memory: &[Bitmask]) -> usize {
    let mut memory_map: HashMap<u64, u64> = HashMap::new();
    for value in memory {
        memory_map.insert(value.mem_loc, value.mem_val_masked);
    }
    memory_map.values().sum::<u64>() as usize
}

#[aoc(day14, part2)]
pub fn part2(memory: &[Bitmask]) -> usize {
    let mut memory_map: HashMap<u64, u64> = HashMap::new();
    for value in memory {
        let mut options: Vec<(usize,u64)> = vec![];
        let count_x = value.mask.matches('X').count();
        let mut zero_out_x_mask: Vec<char> = vec!['1'; value.mask.len()];
        for bit in value.mask.match_indices('X') {
            if let Some(elem) = zero_out_x_mask.get_mut(bit.0) {
                *elem = '0';
            }
            options.push((bit.0,0));
            options.push((bit.0,1));
        }
        let zero_out = u64::from_str_radix(&zero_out_x_mask.iter().collect::<String>(), 2).unwrap();
        let unique_options = options.to_vec().into_iter().combinations(count_x).filter(|x| { x.iter().unique_by(|x| x.0).count() == count_x }).collect::<Vec<Vec<(usize,u64)>>>();
        for option in unique_options.to_vec() {
            let mut option_mask: Vec<char> = value.mask.chars().collect();
            for bit in option {
                if let Some(elem) = option_mask.get_mut(bit.0) {
                    *elem = match bit.1 {
                        0 => '0',
                        1 => '1',
                        _ => panic!(),
                    };
                }
            }
            let x_mask_u64 = u64::from_str_radix(&option_mask.iter().collect::<String>(), 2).unwrap();
            memory_map.insert((value.mem_loc & zero_out) | x_mask_u64, value.mem_val);
        }
    }
    memory_map.values().sum::<u64>() as usize
}

#[cfg(test)]
mod tests {
    use super::{input_generator, part1, part2};

    const SAMPLE_INPUT1: &str = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X\nmem[8] = 11\nmem[7] = 101\nmem[8] = 0";
    const SAMPLE_INPUT2: &str = "mask = 000000000000000000000000000000X1001X\nmem[42] = 100\nmask = 00000000000000000000000000000000X0XX\nmem[26] = 1";

    #[test]
    fn sample1() {
        assert_eq!(part1(&input_generator(SAMPLE_INPUT1)), 165);
    }

    #[test]
    fn sample2() {
        assert_eq!(part2(&input_generator(SAMPLE_INPUT2)), 208);
    }
    // too low: 181064495239
}
