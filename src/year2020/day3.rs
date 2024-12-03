use std::collections::HashMap;

pub struct TreeMap {
    map: HashMap<(usize,usize), bool>,
    x: usize,
    y: usize,
}

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> TreeMap {
    let y = input.lines().count();
    let x = input.lines().next().unwrap().len();
    let mut map = HashMap::new();
    for (i, line) in input.lines().enumerate() {
        for (j, char) in line.chars().enumerate() {
            map.insert((i,j),char == '#');
        }
    }
    TreeMap{map, x, y}
}

pub fn count_trees(tree_map: &TreeMap, rise: usize, run: usize) -> usize {
    let mut x: usize = 0;
    let mut y: usize = 0;
    let mut tree_count = 0;
    loop {
        x+=run;
        y+=rise;
        if y >= tree_map.y {
            break
        }
        if tree_map.map.get(&(y,x)).is_none() {
            x %= tree_map.x;
        }
        if *tree_map.map.get(&(y,x)).unwrap() {
            tree_count += 1;
        }
    }
    tree_count
}

#[aoc(day3, part1)]
pub fn part1(tree_map: &TreeMap) -> usize {
    let rise = 1;
    let run = 3;
    count_trees(tree_map, rise, run)
}

#[aoc(day3, part2)]
pub fn part2(tree_map: &TreeMap) -> usize {
    vec![(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)].iter().map(|(rise, run)| {
        count_trees(tree_map, *rise, *run)
    }).product()
}

#[cfg(test)]
mod tests {
    use super::{input_generator, part1, part2};

    #[test]
    fn sample1() {
        assert_eq!(part1(&input_generator("..##.......\n#...#...#..\n.#....#..#.\n..#.#...#.#\n.#...##..#.\n..#.##.....\n.#.#.#....#\n.#........#\n#.##...#...\n#...##....#\n.#..#...#.#")), 7);
   }
    #[test]
    fn sample2() {
        assert_eq!(part2(&input_generator("..##.......\n#...#...#..\n.#....#..#.\n..#.#...#.#\n.#...##..#.\n..#.##.....\n.#.#.#....#\n.#........#\n#.##...#...\n#...##....#\n.#..#...#.#")), 336);
   }
    // guessed 410
}
