#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> (Vec<Vec<u32>>, Vec<Vec<u32>>) {
    let ordering_pages: Vec<&str> = input.split("\n\n").collect();
    let ordering_rules = ordering_pages[0].split("\n").map(|n| n.split("|").map(|m| m.parse::<u32>().unwrap() ).collect::<Vec<u32>>()).collect::<Vec<Vec<u32>>>();
    let page_updates = ordering_pages[1].split("\n").map(|n| n.split(",").map(|m| m.parse::<u32>().unwrap()).collect::<Vec<u32>>()).collect::<Vec<Vec<u32>>>();
    (ordering_rules, page_updates)
}

pub fn check_order_first(update: &[u32], rules: &[Vec<u32>]) -> bool {
    if update.len() == 1 {
        return true
    }
    let first = update[0];
    let rest = &update[1..];
    for page in rest {
        if rules.contains(&vec![*page,first]) {
            return false
        }
    }
    true
}

pub fn check_order(update: &[u32], rules: &[Vec<u32>]) -> bool {
    if update.len() == 1 {
        return true
    }
    let first = update[0];
    let rest = &update[1..];
    for page in rest {
        if rules.contains(&vec![*page,first]) {
            return false
        }
    }
    check_order(rest, rules)
}

pub fn correct_order(update: &[u32], rules: &[Vec<u32>]) -> Vec<u32> {
    if update.len() == 1 {
        return update.to_vec()
    }
    for (i, page) in update.iter().enumerate() {
        let mut rest = update.to_vec();
        rest.remove(i);

        if check_order_first(&[vec![*page],rest.clone()].iter().flatten().copied().collect::<Vec<u32>>(), rules) {
            return [vec![*page],correct_order(&rest, rules)].iter().flatten().copied().collect::<Vec<u32>>()
        }
    }
    panic!("No solution found for {:?}", update);
}

// pub fn correct_order(update: &[u32], rules: &[Vec<u32>]) -> Vec<u32> {
//     update.iter().copied().permutations(update.len()).par_bridge().find_any(|option| check_order(option, rules)).unwrap().to_vec()
// }

#[aoc(day5, part1)]
pub fn part1((ordering_rules, page_updates): &(Vec<Vec<u32>>, Vec<Vec<u32>>)) -> u32 {
    let mut total: u32 = 0;
    // println!("{:?} {:?}", ordering_rules, page_updates);
    for page_update in page_updates {
        // println!("{:?}", page_update);
        if check_order(page_update, ordering_rules) {
            total += page_update[page_update.len()/2]
        }
    }
    total
}

#[aoc(day5, part2)]
pub fn part2((ordering_rules, page_updates): &(Vec<Vec<u32>>, Vec<Vec<u32>>)) -> u32 {
    let mut total: u32 = 0;
    // println!("{:?} {:?}", ordering_rules, page_updates);
    for page_update in page_updates {
        if ! check_order(page_update, ordering_rules) {
            let new_page_update = correct_order(page_update, ordering_rules);
            // println!("{:?}", new_page_update);
            total += new_page_update[new_page_update.len()/2]
        }
    }
    total
}

#[cfg(test)]
mod tests {
    use super::{input_generator, part1, part2};

    const SAMPLES: &[(&str, u32, u32)] = &[
        (r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"#, 143, 123)];

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
