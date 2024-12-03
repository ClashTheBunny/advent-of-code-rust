extern crate regex;
extern crate lazy_static;
use self::regex::Regex;
use self::lazy_static::lazy_static;

use std::collections::HashMap;

lazy_static! {
  static ref BAG_RE: Regex = Regex::new(r"^(.*) bags contain (.*)\.$").unwrap();
  static ref INNER_BAG_RE: Regex = Regex::new(r"^(\d+) (.*) bag").unwrap();
}

const MY_BAG: &str = "shiny gold";

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> HashMap<String , HashMap<String, u32>> {
    input.lines().map(|line| {
        let caps = BAG_RE.captures(line).unwrap();
        let container = caps.get(1).map_or("", |m| m.as_str());
        let containee = caps.get(2).map_or("", |m| m.as_str());
        let containee = containee.split(", ").map(|x| {
            if let Some(caps) = INNER_BAG_RE.captures(x) {
            (
              String::from(caps.get(2).map_or("", |m| m.as_str())),
              caps.get(1).map_or("", |m| m.as_str()).parse::<u32>().unwrap(),
            ) } else {
                (String::from("None"), 0)
            }
        }).collect::<HashMap<String, u32>>();
        (String::from(container), containee)
    })
    .collect::<HashMap<String, HashMap<String, u32>>>()
}

pub fn find_containers(input: &HashMap<String, HashMap<String, u32>>, bag: &str) -> Vec<String> {
    let mut container_keys: Vec<String> = input.iter().clone().filter(|(_,y)| { y.contains_key(bag) }).map(|(x,_)| String::from(x)).collect();
    container_keys.to_vec().iter().for_each(|x| {
        container_keys.extend(find_containers(input, x));
    });
    container_keys.sort_unstable();
    container_keys.dedup();
    container_keys
}

#[aoc(day7, part1)]
pub fn part1(input: &HashMap<String, HashMap<String, u32>>) -> usize {
    find_containers(input, MY_BAG).len()
}

pub fn find_containees(input: &HashMap<String, HashMap<String, u32>>, bag: &str) -> u32 {
    let container_keys: Vec<HashMap<String, u32>> = input.iter().clone().filter(|(x,_)| { *x == bag }).map(|(_,y)| y.clone()).collect();
    let mut total: u32 = 0;
    for x in container_keys {
        for (y, z) in x.into_iter() {
            if z > 0 {
               total += z + (z * find_containees(input, y.as_str()));
            }
        }
    }
    total
}

#[aoc(day7, part2)]
pub fn part2(input: &HashMap<String, HashMap<String, u32>>) -> u32 {
    find_containees(input, MY_BAG)
}

#[cfg(test)]
mod tests {
    use super::{input_generator, part1, part2};

    const SAMPLE_2: &str = "shiny gold bags contain 2 dark red bags.\ndark red bags contain 2 dark orange bags.\ndark orange bags contain 2 dark yellow bags.\ndark yellow bags contain 2 dark green bags.\ndark green bags contain 2 dark blue bags.\ndark blue bags contain 2 dark violet bags.\ndark violet bags contain no other bags.";

    const SAMPLE_1: &str = "light red bags contain 1 bright white bag, 2 muted yellow bags.\ndark orange bags contain 3 bright white bags, 4 muted yellow bags.\nbright white bags contain 1 shiny gold bag.\nmuted yellow bags contain 2 shiny gold bags, 9 faded blue bags.\nshiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.\ndark olive bags contain 3 faded blue bags, 4 dotted black bags.\nvibrant plum bags contain 5 faded blue bags, 6 dotted black bags.\nfaded blue bags contain no other bags.\ndotted black bags contain no other bags.";

    #[test]
    fn sample1() {
        assert_eq!(part1(&input_generator(SAMPLE_1)), 4);
   }

    #[test]
    fn sample2() {
        assert_eq!(part2(&input_generator(SAMPLE_1)), 32);
    }

    #[test]
    fn sample3() {
        assert_eq!(part2(&input_generator(SAMPLE_2)), 126);
   }
}
