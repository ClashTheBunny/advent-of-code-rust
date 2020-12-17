// use itertools::Itertools;
// use itertools::MinMaxResult::MinMax;
// use std::cmp::Ordering;
use std::collections::HashMap;

#[aoc_generator(day11)]
pub fn input_generator(input: &str) -> HashMap<(usize, usize), bool> {
    let mut seat_map = HashMap::new();
    input.lines().enumerate().for_each(|(y , line)| { line.chars().enumerate().for_each(|(x, seatness)| { if seatness == 'L' { seat_map.insert((x,y),false); }; })});
    seat_map
}

fn close_seats_full(
    seat_map: &HashMap<(usize, usize), bool>,
    loc: (usize, usize),
    max_x: usize,
    max_y: usize,
    max_factor: i32) -> usize {
    let mut total = 0;
    for x in [-1_i32, 0, 1].iter() {
        for y in [-1_i32, 0, 1].iter() {
            if !((*x == 0) && (*y == 0)) {
                // println!("{} {}", x, y);
                let mut factor = 1;
                let mut loc_x = (*x + loc.0 as i32) as usize;
                let mut loc_y = (*y + loc.1 as i32) as usize;
                while seat_map.get(&(loc_x,loc_y)).is_none() {
                    factor += 1;
                    if factor > max_factor {
                        break;
                    }
                    loc_x = (*x * factor + loc.0 as i32) as usize;
                    loc_y = (*y * factor + loc.1 as i32) as usize;
                    if loc_x > max_x || loc_y > max_y {
                        break;
                    }
                }
                match seat_map.get(&(loc_x,loc_y)) {
                    Some(true) => total += 1,
                    Some(false) => {},
                    None => {},
                };
            }
        }
    }
    total
}

fn step_seats(
    mut seat_map: HashMap<(usize, usize), bool>,
    max_x: usize,
    max_y: usize,
    personal_bubble: usize,
    max_factor: i32
) -> (HashMap<(usize, usize), bool>, bool) {
    let static_seat = seat_map.clone();
    let mut changed: bool = false;
    for (loc, seat) in seat_map.iter_mut() {
        match (*seat, close_seats_full(&static_seat, *loc, max_x, max_y, max_factor)) {
            (true, x) if x >= personal_bubble => {*seat = false; changed = true},
            (false, 0) => {*seat = true; changed = true},
            (_, _) => {},
        }
    }
    (seat_map, changed)
}

#[allow(dead_code)]
fn print_seat_map(seat_map: &HashMap<(usize, usize), bool>) {
    let (max_x, max_y) = seat_map.iter().fold((0, 0), |(max_x, max_y),(x,_)| {
        match x {
            (x, y) if *x > max_x && *y > max_y => (*x, *y),
            (x, _) if *x > max_x => (*x, max_y),
            (_, y) if *y > max_y => (max_x, *y),
            (_, _) => (max_x, max_y),
        }
    });
    for y in 0..=max_y {
        for x in 0..=max_x {
                match seat_map.get(&(x as usize,y as usize)) {
                    Some(true) => print!("#"),
                    Some(false) => print!("L"),
                    None => print!("."),
                };
        }
        println!();
    }
    println!("{} {}", max_x, max_y);
}

#[aoc(day11, part1)]
pub fn part1(input: &HashMap<(usize, usize), bool>) -> u64 {

    let mut newseats: HashMap<(usize, usize), bool>;
    newseats = input.clone();

    let (max_x, max_y) = input.iter().fold((0, 0), |(max_x, max_y),(x,_)| {
        match x {
            (x, y) if *x > max_x && *y > max_y => (*x, *y),
            (x, _) if *x > max_x => (*x, max_y),
            (_, y) if *y > max_y => (max_x, *y),
            (_, _) => (max_x, max_y),
        }
    });

    let max_factor: i32 = 1;

    loop {
        match step_seats(newseats, max_x, max_y, 4, max_factor) {
            (x, true) => newseats = x,
            (x, false) => {newseats = x; break},
        }
    }
    newseats.iter().fold(0, |mut count, (_, val)| { if *val { count += 1 }; count })
}

#[aoc(day11, part2)]
pub fn part2(input: &HashMap<(usize, usize), bool>) -> u64 {

    let mut newseats: HashMap<(usize, usize), bool>;
    newseats = input.clone();

    let (max_x, max_y) = input.iter().fold((0, 0), |(max_x, max_y),(x,_)| {
        match x {
            (x, y) if *x > max_x && *y > max_y => (*x, *y),
            (x, _) if *x > max_x => (*x, max_y),
            (_, y) if *y > max_y => (max_x, *y),
            (_, _) => (max_x, max_y),
        }
    });

    let max_factor: i32;
    if max_x > max_y {
        max_factor = max_x as i32;
    } else {
        max_factor = max_y as i32;
    }

    loop {
        match step_seats(newseats, max_x, max_y, 5, max_factor) {
            (x, true) => newseats = x,
            (x, false) => {newseats = x; break},
        }
    }
    newseats.iter().fold(0, |mut count, (_, val)| { if *val { count += 1 }; count })
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::{input_generator, part1, part2};

    const SAMPLE_1: &str = "L.LL.LL.LL\nLLLLLLL.LL\nL.L.L..L..\nLLLL.LL.LL\nL.LL.LL.LL\nL.LLLLL.LL\n..L.L.....\nLLLLLLLLLL\nL.LLLLLL.L\nL.LLLLL.LL";

    #[test]
    fn sample1() {
        assert_eq!(part1(&input_generator(SAMPLE_1)), 37);
    }

    #[test]
    fn sample2() {
        assert_eq!(part2(&input_generator(SAMPLE_1)), 26);
   }
}
