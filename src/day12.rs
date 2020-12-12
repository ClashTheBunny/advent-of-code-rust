use std::str::FromStr;

#[derive(Debug)]
pub struct Instruction {
    verb: char,
    magnitude: i32,
}

#[aoc_generator(day12)]
pub fn input_generator(input: &str) -> Vec<Instruction> {
    input.lines().map(|x| {
        Instruction{
            verb: char::from_str(&x[0..=0]).unwrap(),
            magnitude: x[1..].parse::<i32>().unwrap()
        }
    }).collect::<Vec<Instruction>>()
}

fn left_turn(dir_in: (i32, i32), n: i32) -> (i32, i32) {
    if n == 0 {
        return dir_in
    }
    left_turn(
        match dir_in {
            (x, y) if x >= 0 && y >= 0 => (-y, x),
            (x, y) if x <= 0 && y >= 0 => (-y, x),
            (x, y) if x <= 0 && y <= 0 => (-y, x),
            (x, y) if x >= 0 && y <= 0 => (-y, x),
            // (0, 1) => (-1, 0),
            // (1, 0) => (0, 1),
            // (0, -1) => (1, 0),
            // (-1, 0) => (0, -1),
            _ => panic!("Directions other than left right up and down"),
        },
        n - 1
    )
}

fn right_turn(dir_in: (i32, i32), n: i32) -> (i32, i32) {
    if n == 0 {
        return dir_in
    }
    right_turn(
        match dir_in {
            (x, y) if x >= 0 && y >= 0 => (y, -x),
            (x, y) if x >= 0 && y <= 0 => (y, -x),
            (x, y) if x <= 0 && y <= 0 => (y, -x),
            (x, y) if x <= 0 && y >= 0 => (y, -x),
            // (0, 1) => (1, 0),
            // (1, 0) => (0, -1),
            // (0, -1) => (-1, 0),
            // (-1, 0) => (0, 1),
            _ => panic!("Directions other than left right up and down"),
        },
        n - 1
    )
}

#[aoc(day12, part1)]
pub fn part1(input: &[Instruction]) -> i32{
    let mut x = 0;
    let mut y = 0;
    let mut dir = (1_i32, 0_i32);
    for inst in input {
        match inst.verb {
            'F' => {
                x += inst.magnitude * dir.0;
                y += inst.magnitude * dir.1;
            },
            'N' => y += inst.magnitude,
            'S' => y -= inst.magnitude,
            'E' => x += inst.magnitude,
            'W' => x -= inst.magnitude,
            'R' => dir = right_turn(dir, inst.magnitude/90),
            'L' => dir = left_turn(dir, inst.magnitude/90),
            n => panic!("{}",n),
        }
    }
    x.abs() + y.abs()
}

#[aoc(day12, part2)]
pub fn part2(input: &[Instruction]) -> i32{
    let mut x = 0;
    let mut y = 0;
    let mut dir = (10_i32, 1_i32);
    for inst in input {
        match inst.verb {
            'F' => {
                x += inst.magnitude * dir.0;
                y += inst.magnitude * dir.1;
            },
            'N' => dir.1 += inst.magnitude,
            'S' => dir.1 -= inst.magnitude,
            'E' => dir.0 += inst.magnitude,
            'W' => dir.0 -= inst.magnitude,
            'R' => dir = right_turn(dir, inst.magnitude/90),
            'L' => dir = left_turn(dir, inst.magnitude/90),
            n => panic!("{}",n),
        }
    }
    x.abs() + y.abs()
}

#[cfg(test)]
mod tests {
    use super::{input_generator, part1, part2};

    const SAMPLE_1: &str = "F10\nN3\nF7\nR90\nF11";

    #[test]
    fn sample1() {
        assert_eq!(part1(&input_generator(SAMPLE_1)), 25);
   }
    // 567
    #[test]
    fn sample2() {
        assert_eq!(part2(&input_generator(SAMPLE_1)), 286);
   }
}
