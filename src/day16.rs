use std::collections::{HashMap, HashSet};
use std::ops::RangeInclusive;
use std::iter::FromIterator;

#[derive(Debug)]
pub struct Input {
   validations: HashMap<String, Vec<RangeInclusive<u32>>>,
   my_ticket: Vec<u32>,
   other_tickets: Vec<Vec<u32>>,
}

fn parse_validations(validations: &str) -> HashMap<String, Vec<RangeInclusive<u32>>> {
    let mut valid_numbers = HashMap::new();
    for line in validations.lines() {
        let mut range: Vec<RangeInclusive<u32>> = vec![];
        let mut entry: &str = "";
        for (i, chunk) in line.split(": ").enumerate() {
            match (i, chunk) {
                (0, _) => entry = chunk,
                (1..=usize::MAX, "or") => {},
                (1..=usize::MAX, _) => {
                    chunk.split(" or ").for_each(|chunk| {
                        let start_end = &mut chunk
                            .split('-')
                            .map(|x| x.parse::<u32>().unwrap() )
                            .collect::<Vec<u32>>();
                        range.push(start_end[0]..=start_end[1]);
                    });
                },
                (_, _) => panic!("{:?}", chunk),
            }
        }
        valid_numbers.insert(String::from(entry), range);
    }
    valid_numbers
}

fn parse_ticket(ticket_line: &str) -> Vec<u32> {
    ticket_line.split(',').map(|x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>()
}

#[aoc_generator(day16)]
pub fn input_generator(input: &str) -> Input {
    let mut validations = HashMap::new();
    let mut my_ticket = vec![];
    let mut other_tickets = vec![];
    for (i, chunk) in input.split("\n\n").enumerate() {
        match i {
            0 => validations = parse_validations(chunk), // valid numbers
            1 => my_ticket = parse_ticket(chunk.split('\n').last().unwrap()), // my ticket
            2 => other_tickets = chunk.
                split('\n').
                collect::<Vec<&str>>()[1..].
                iter().map(|x| x.split(',')
                    .map(|y| y.parse::<u32>().
                        unwrap())
                    .collect())
                .collect::<Vec<Vec<u32>>>(), //other tickets
            _ => panic!("{:?}", chunk),
        }
    }
    Input{ validations, my_ticket, other_tickets }
}

#[aoc(day16, part1)]
pub fn part1(input: &Input) -> u32 {
    let mut total = 0;
    for ticket in &input.other_tickets {
        for item in ticket {
            let mut item_valid = false;
            'outer: for (_, value) in input.validations.iter() {
                for range in value {
                    if range.contains(item) {
                        item_valid = true;
                        break 'outer;
                    }
                }
            }
            if ! item_valid {
                total += item
            }
        }
    }
    total
}

#[aoc(day16, part2)]
pub fn part2(input: &Input) -> u64 {
    let mut total = 0;
    let mut valid_tickets = vec![];
    for ticket in &input.other_tickets {
        let mut valid_ticket = true;
        for item in ticket {
            let mut item_valid = false;
            'outer: for (_, value) in input.validations.iter() {
                for range in value {
                    if range.contains(item) {
                        item_valid = true;
                        break 'outer;
                    }
                }
            }
            if ! item_valid {
                total += item;
                valid_ticket = false;
                break;
            }
        }
        if valid_ticket {
            valid_tickets.push(ticket);
        }
    }
    let ticket_len = input.my_ticket.len();
    let mut answers: HashMap<String, Vec<usize>> = HashMap::new();
    for (key, value) in input.validations.iter() {
        for i in 0..ticket_len {
            let mut item_valid = true;
            for ticket in &valid_tickets {
                #[allow(clippy::blocks_in_if_conditions)]
                if ! value.iter().any(|range| {
                    range.contains(&ticket[i])
                }) {
                    item_valid = false;
                    break;
                }
            }
            if item_valid {
                if let Some(x) = answers.get_mut(&*key) {
                    x.push(i);
                } else {
                    answers.insert(key.to_owned(), vec![i]);
                }
            }
        }
    }
    let mut answer_vec = answers.into_iter().collect::<Vec<(String, Vec<usize>)>>();
    answer_vec.sort_by(|x,y| x.1.len().cmp(&y.1.len()));
    let answer_hashsets = answer_vec.into_iter().map(|(x,y)| (x,HashSet::from_iter(y))).collect::<Vec<(String, HashSet<usize>)>>();

    let mut total: u64 = 1;
    for (i, (name, data_item)) in answer_hashsets.iter().enumerate() {
        let value;
        if i>0 {
            value = data_item.difference(&answer_hashsets[i-1].1).collect::<Vec<&usize>>()[0];
        } else {
            value = data_item.iter().next().unwrap();
        }
        if name.contains("departure") {
            total *= input.my_ticket[*value as usize] as u64;
        }
    }
    total
}

#[cfg(test)]
mod tests {
    use super::{input_generator, part1, part2};

    const SAMPLE_1: &str = "class: 1-3 or 5-7\nrow: 6-11 or 33-44\nseat: 13-40 or 45-50\n\nyour ticket:\n7,1,14\n\nnearby tickets:\n7,3,47\n40,4,50\n55,2,20\n38,6,12";

    #[test]
    fn sample1() {
        assert_eq!(part1(&input_generator(SAMPLE_1)), 71);
   }
    #[test]
    fn sample2() {
        assert_eq!(part2(&input_generator(SAMPLE_1)), 1);
   }
}
