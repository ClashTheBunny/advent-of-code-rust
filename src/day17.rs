// use itertools::Itertools;
// use itertools::MinMaxResult::MinMax;
// use std::cmp::Ordering;
use std::collections::HashMap;

#[aoc_generator(day17)]
pub fn input_generator(input: &str) -> HashMap<(i64, i64, i64, i64), bool> {
    let mut seat_map = HashMap::new();
    input.lines().enumerate().for_each(|(y , line)| { line.chars().enumerate().for_each(|(x, seatness)| {
        if seatness == '#' {
            seat_map.insert((x as i64,y as i64, 0_i64, 0_i64),true);
        } else {
            seat_map.insert((x as i64,y as i64, 0_i64, 0_i64),false);
        }; })});
    seat_map
}

fn find_min(input: &HashMap<(i64, i64, i64, i64), bool>) -> (i64, i64, i64, i64) {
    input.iter().fold((0, 0, 0, 0), |(mut min_x, mut min_y, mut min_z, mut min_w),(l,_)| {
        if l.0 < min_x {
            min_x = l.0;
        }
        if l.1 < min_y {
            min_y = l.1;
        }
        if l.2 < min_z {
            min_z = l.2;
        }
        if l.3 < min_w {
            min_w = l.3;
        }
        (min_x, min_y, min_z, min_w)
    })
}

fn find_max(input: &HashMap<(i64, i64, i64, i64), bool>) -> (i64, i64, i64, i64) {
    input.iter().fold((0, 0, 0, 0), |(mut max_x, mut max_y, mut max_z, mut max_w),(l,_)| {
        if l.0 > max_x {
            max_x = l.0;
        }
        if l.1 > max_y {
            max_y = l.1;
        }
        if l.2 > max_z {
            max_z = l.2;
        }
        if l.3 > max_w {
            max_w = l.3;
        }
        (max_x, max_y, max_z, max_w)
    })
}

fn close_seats_full(
    seat_map: &HashMap<(i64, i64, i64, i64), bool>,
    loc: (i64, i64, i64, i64),
    min_x: i64,
    min_y: i64,
    min_z: i64,
    min_w: i64,
    max_x: i64,
    max_y: i64,
    max_z: i64,
    max_w: i64,
    max_factor: i64) -> i64 {
    let mut total = 0;
    for x in [-1_i64, 0, 1].iter() {
        for y in [-1_i64, 0, 1].iter() {
            for z in [-1_i64, 0, 1].iter() {
                for w in [-1_i64, 0, 1].iter() {
                    if !((*x == 0) && (*y == 0) && (*z == 0) && (*w == 0)) {
                        // println!("{} {}", x, y);
                        let mut factor = 1;
                        let mut loc_x = *x + loc.0;
                        let mut loc_y = *y + loc.1;
                        let mut loc_z = *z + loc.2;
                        let mut loc_w = *w + loc.3;
                        while seat_map.get(&(loc_x,loc_y,loc_z,loc_w)).is_none() {
                            factor += 1;
                            if factor > max_factor {
                                break;
                            }
                            loc_x = *x * factor + loc.0;
                            loc_y = *y * factor + loc.1;
                            loc_z = *z * factor + loc.1;
                            loc_w = *w * factor + loc.2;
                            if loc_x > max_x || loc_y > max_y || loc_z > max_z ||loc_w > max_w ||
                                loc_x < min_x || loc_y < min_y || loc_z < min_z || loc_w > min_w {
                                    break;
                            }
                        }
                        match seat_map.get(&(loc_x,loc_y,loc_z,loc_w)) {
                            Some(true) => total += 1,
                            Some(false) => {},
                            None => {},
                        };
                    }
                }
            }
        }
    }
    total
}

fn step_seats(
    mut seat_map: HashMap<(i64, i64, i64, i64), bool>,
    min_x: i64,
    min_y: i64,
    min_z: i64,
    min_w: i64,
    max_x: i64,
    max_y: i64,
    max_z: i64,
    max_w: i64,
    personal_bubble: i64,
    max_factor: i64
) -> (HashMap<(i64, i64, i64, i64), bool>, bool) {
    let static_seat = seat_map.clone();
    let mut changed: bool = false;
    for (loc, seat) in static_seat.iter() {
        for x in [-1_i64, 0, 1].iter() {
            for y in [-1_i64, 0, 1].iter() {
                for z in [-1_i64, 0, 1].iter() {
                    for w in [-1_i64, 0, 1].iter() {
                        if !((*x == 0) && (*y == 0) &&(*z == 0)) {
                            let new_loc = (loc.0 + x, loc.1 + y, loc.2 + z, loc.3 + w);
                            if ! static_seat.contains_key(&new_loc) {
                                seat_map.insert(new_loc, false);
                            }
                        }
                    }
                }
            }
        }
    }
    for (loc, seat) in seat_map.iter_mut() {
        match (*seat, close_seats_full(&static_seat, *loc, min_x, min_y, min_z, min_w, max_x, max_y, max_z, max_w, max_factor)) {
            (true, x) if (2..=3).contains(&x) => {},
            (false, 3) => {*seat = true; changed = true},
            (true, _) => {*seat = false; changed = true},
            (false, _) => {},
            (_, _) => panic!("I hadn't considered this..."),
        }
    }
    (seat_map, changed)
}

#[allow(dead_code)]
fn print_seat_map(seat_map: &HashMap<(i64, i64, i64, i64), bool>) {
    let (max_x, max_y, max_z, max_w) = find_max(&seat_map);
    let (min_x, min_y, min_z, min_w) = find_min(&seat_map);
    for w in min_w..=max_w {
        for z in min_z..=max_z {
            for y in min_y..=max_y {
                for x in min_x..=max_x {
                    match seat_map.get(&(x as i64,y as i64, z as i64, w as i64)) {
                        Some(true) => print!("#"),
                        Some(false) => print!("."),
                        None => print!("."),
                    };
                }
                println!();
            }
            println!();
        }
        println!();
    }
    println!("{} {}", max_x, max_y);
}

#[aoc(day17, part1)]
pub fn part1(input: &HashMap<(i64, i64, i64, i64), bool>) -> u64 {
    let mut newseats: HashMap<(i64, i64, i64, i64), bool>;
    newseats = input.clone();

    // print_seat_map(&input);

    let max_factor: i64 = 1;

    for _ in 0..6 {
        let (mut max_x, mut max_y, mut max_z, mut max_w) = find_max(&input);
        let (mut min_x, mut min_y, mut min_z, mut min_w) = find_min(&input);

        match step_seats(newseats, min_x, min_y, min_z, min_w, max_x, max_y, max_z, max_w, 4, max_factor) {
            (x, true) => newseats = x,
            (x, false) => {newseats = x; break},
        }
    }
    // print_seat_map(&newseats);
    newseats.iter().fold(0, |mut count, (_, val)| { if *val { count += 1 }; count })
}

#[aoc(day17, part2)]
pub fn part2(input: &HashMap<(i64, i64, i64, i64), bool>) -> u64 {
    let mut newseats: HashMap<(i64, i64, i64, i64), bool>;
    newseats = input.clone();

    // print_seat_map(&input);

    let max_factor: i64 = 1;

    for _ in 0..6 {
        let (mut max_x, mut max_y, mut max_z, mut max_w) = find_max(&input);
        let (mut min_x, mut min_y, mut min_z, mut min_w) = find_min(&input);

        match step_seats(newseats, min_x, min_y, min_z, 0, max_x, max_y, max_z, 0, 4, max_factor) {
            (x, true) => newseats = x,
            (x, false) => {newseats = x; break},
        }
    }
    // print_seat_map(&newseats);
    newseats.iter().fold(0, |mut count, (_, val)| { if *val { count += 1 }; count })
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::{input_generator, part1, part2};

    const SAMPLE_1: &str = ".#.\n..#\n###";

    #[test]
    fn sample1() {
        assert_eq!(part1(&input_generator(SAMPLE_1)), 112);
    }

    #[test]
    fn sample2() {
        assert_eq!(part2(&input_generator(SAMPLE_1)), 26);
    }
}
