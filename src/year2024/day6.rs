use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> Vec<Vec<u32>> {
    input.split("\n").map(|n| n.chars().map(|m| {
        match m {
            '#' => 1,
            '.' => 0,
            '^' => 2,
            _ => panic!("unknown map character"),
        }
    }).collect::<Vec<u32>>()).collect::<Vec<Vec<u32>>>()
}

#[aoc(day6, part1)]
pub fn part1(input: &[Vec<u32>]) -> usize {
    let mut guard_pos = (0usize, 0usize);
    let mut visited: HashSet<(usize, usize)> = Default::default();
    let mut guard_dir = Direction::Up;
    for (y, row) in input.iter().enumerate() {
        for (x, pos) in row.iter().enumerate() {
            if *pos == 2 {
                guard_pos = (x, y);
                visited.insert(guard_pos);
                break;
            }
        }
    }
    while guard_pos.1 < input[0].len() && guard_pos.0 < input.len() {

        let (x,y) = guard_pos;

        match guard_dir {
            Direction::Up => if y == 0 { break },
            Direction::Right => if x + 1 == input[0].len() { break },
            Direction::Down => if y + 1 == input.len() { break },
            Direction::Left => if x == 0 { break },
        };

        let next_pos = match guard_dir {
            Direction::Up => (x, y - 1),
            Direction::Right => (x + 1, y),
            Direction::Down => (x, y + 1),
            Direction::Left => (x - 1, y),
        };

        if next_pos.1 < input.len() && next_pos.0 < input[0].len() && input[next_pos.1][next_pos.0] != 1 {
            guard_pos = next_pos;
            visited.insert(guard_pos);
        } else {
            guard_dir = match guard_dir {
                Direction::Up => Direction::Right,
                Direction::Right => Direction::Down,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Up,

            }
        }
    }
    // println!("{:?} {:?} {:?} {:?}", input, guard_pos, guard_dir, visited);
    visited.len()
}

#[aoc(day6, part2)]
pub fn part2(orig_input: &[Vec<u32>]) -> usize {
    let orig_guard_dir = Direction::Up;
    let mut orig_guard_pos = (0usize, 0usize, orig_guard_dir);
    for (y, row) in orig_input.iter().enumerate() {
        for (x, pos) in row.iter().enumerate() {
            if *pos == 2 {
                orig_guard_pos = (x, y, orig_guard_dir);
                break;
            }
        }
    }


    let mut obs_pos: HashSet<(usize, usize)> = Default::default();
    for y_obst in 0..orig_input.len() {
        for x_obst in 0..orig_input[0].len() {
            if orig_input[y_obst][x_obst] > 0 {
               continue; 
            }
            let mut count = 0;
            let input: &mut Vec<Vec<u32>> = &mut orig_input.to_vec();
            let new_input = input.get_mut(y_obst).unwrap().get_mut(x_obst).unwrap();
            *new_input = 1;
            let mut guard_pos = orig_guard_pos;
            let mut guard_dir = orig_guard_dir;
            let mut visited: HashSet<(usize, usize, Direction)> = Default::default();
            visited.insert(guard_pos);
            while guard_pos.1 < input[0].len() && guard_pos.0 < input.len() {

                let (x,y, _) = guard_pos;

                match guard_dir {
                    Direction::Up => if y == 0 { break },
                    Direction::Right => if x + 1 == input[0].len() { break },
                    Direction::Down => if y + 1 == input.len() { break },
                    Direction::Left => if x == 0 { break },
                };

                let next_pos = match guard_dir {
                    Direction::Up => (x, y - 1, guard_dir),
                    Direction::Right => (x + 1, y, guard_dir),
                    Direction::Down => (x, y + 1, guard_dir),
                    Direction::Left => (x - 1, y, guard_dir),
                };


                if next_pos.1 < input.len() && next_pos.0 < input[0].len() && input[next_pos.1][next_pos.0] != 1 {
                    guard_pos = next_pos;
                    if visited.contains(&next_pos) {
                        obs_pos.insert((x_obst,y_obst));
                        break;
                    }
                    visited.insert(guard_pos);
                } else {
                    guard_dir = match guard_dir {
                        Direction::Up => Direction::Right,
                        Direction::Right => Direction::Down,
                        Direction::Down => Direction::Left,
                        Direction::Left => Direction::Up,

                    }
                }
                count+=1;
                if count % 100000 == 0 {
                    println!("{:?}", visited);
                }
            }
            // println!("{:?}", obs_pos);
        }
    }
    obs_pos.len()
}

#[cfg(test)]
mod tests {
    use super::{input_generator, part1, part2};

    const SAMPLES: &[(&str, usize, usize)] = &[
        (r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."#, 41, 6)];

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
