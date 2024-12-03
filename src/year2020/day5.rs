use std::num::ParseIntError;
use std::str::FromStr;
use std::collections::HashSet;

#[derive(Debug, PartialEq)]
pub struct Seat {
    row: u32,
    col: u32,
    input: String,
    id: usize,
}

impl FromStr for Seat {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let binary_s = s.replace(&['F', 'L'][..],"0").replace(&['B', 'R'][..],"1");
        let row: u32 = u32::from_str_radix(&binary_s[0..7],2)?;
        let col: u32 = u32::from_str_radix(&binary_s[7..],2)?;
        let id: usize = ((row * 8) + col) as usize;
        // println!("{:?}", Seat{row, col, input: String::from(s), id});
        Ok(Seat{row, col, input: String::from(s), id})
    }
}

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Vec<Seat> {
    input.lines().map(|x| {
        // println!("{}", x);
        Seat::from_str(x)
    }).filter_map(Result::ok).collect()
}

#[aoc(day5, part1)]
pub fn part1(input: &[Seat]) -> usize {
    (*input.iter().max_by(|x, y| x.id.cmp(&y.id)).unwrap()).id as usize
}

#[aoc(day5, part2)]
pub fn part2(input: &[Seat]) -> usize {
    let max_seat = (*input.iter().max_by(|x, y| x.id.cmp(&y.id)).unwrap()).id as usize;
    let min_seat = (*input.iter().min_by(|x, y| x.id.cmp(&y.id)).unwrap()).id as usize;
    let all_seats: HashSet<usize> = (min_seat..=max_seat).collect();
    let tickets: HashSet<usize> = input.iter().map(|x| x.id).collect();
    // println!("{:?}", tickets);
    *all_seats.difference(&tickets).min().unwrap()
}

#[cfg(test)]
mod tests {
   use super::{input_generator, part1, part2};

   #[test]
   fn sample1() {
       assert_eq!(part1(&input_generator("BFFFBBFRRR")), 567);
       assert_eq!(part1(&input_generator("FFFBBBFRRR")), 119);
       assert_eq!(part1(&input_generator("BBFFBBFRLL")), 820);
   }
   #[test]
   fn sample2() {
       assert_eq!(part2(&input_generator("BBFFBBFRLL\nFFFBBBFRRR\nBFFFBBFRRR")), 120);
   }
}
