#[aoc(day1, part1)]
pub fn part1(input: &str) -> Option<i32> {
    let mut my_ints = input.lines().map(|x|
        x.parse::<i32>().unwrap()
    ).collect::<Vec<i32>>();
    my_ints.sort();
    for x in &my_ints {
        if 2020 - x > 0 {
            if let Some(_) = my_ints.iter().position(|&y| y == 2020 - x ) {
                return Some(x * (2020 - x))
            }
        }
    }
    None
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> Option<i32> {
    let mut my_ints = input.lines().map(|x|
        x.parse::<i32>().unwrap()
    ).collect::<Vec<i32>>();
    my_ints.sort();
    for x in &my_ints {
        if 2020 - x > 0 {
            for y in &my_ints {
                if let Some(_) = my_ints.iter().position(|&z| z == 2020 - x - y ) {
                    return Some(y * x * (2020 - x - y))
                }
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    fn sample1() {
        assert_eq!(part1("1721\n979\n366\n299\n675\n1456"), Some(514579));
    }
    #[test]
    fn sample2() {
        assert_eq!(part2("1721\n979\n366\n299\n675\n1456"), Some(241861950));
    }
}
