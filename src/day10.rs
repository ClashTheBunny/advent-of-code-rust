use cached::proc_macro::cached;
use itertools::Itertools;
use std::thread;
use std::collections::HashSet;

#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> Vec<i64> {
    input.lines().map(|x| { x.parse::<i64>().unwrap() }).collect()
}

#[cached]
fn check_plugs(plug_list: Vec<i64>) -> Option<i64> {
    let mut three_sum = 0;
    let mut two_sum = 0;
    let mut one_sum = 0;
    for wind in plug_list.windows(2) {
        match wind[1] - wind[0] {
            1 => one_sum += 1,
            2 => two_sum +=1,
            3 => three_sum +=1,
            _ => return None,
        }
    }
    // if two_sum > 0 {
    //     println!("{:?}", plug_list);
    //     // panic!("Doesn't seem to be possible, but I don't understand why, let's see if it ever happens");
    // }
    Some((if one_sum > 0 { one_sum } else { 1 }) *
          (if two_sum > 0 { two_sum } else { 1 } ) *
            (if three_sum > 0 { three_sum } else { 1 } ))
}

#[aoc(day10, part1)]
pub fn part1(input: &[i64]) -> i64 {
    let mut sort_input = input.to_vec();
    sort_input.push(0);
    sort_input.sort_unstable();
    let max = sort_input.pop().unwrap();
    sort_input.push(max);
    sort_input.push(max + 3);
    check_plugs(sort_input.to_vec()).unwrap()
}

#[cached]
pub fn run_part2(plug_list: Vec<i64>) -> HashSet<Vec<i64>>{
    let mut correct = HashSet::new();
    let mut incorrect = HashSet::new();
    let input_len = plug_list.len();
    let first_plug = plug_list[0];
    let last_plug = plug_list[input_len - 1];
    if input_len > 1 && check_plugs(plug_list.to_vec()).is_some() {
        correct.insert(plug_list.to_vec());
        for combination in plug_list.iter().cloned().combinations(input_len - 1) {
           let mut copy = combination.to_vec();
           copy.sort_unstable();
           if copy[0] != first_plug {
               continue
           }
           if copy[copy.len() -1] != last_plug {
               continue
           }
           if correct.contains(&copy) || incorrect.contains(&copy) {
               continue
           }
           if check_plugs(copy.to_vec()).is_some() {
               correct.insert(copy.to_vec());
               for combo in run_part2(copy) {
                   correct.insert(combo);
               }
           } else {
               incorrect.insert(copy.to_vec());
           };
        }
    }
    correct
}

const STACK_SIZE: usize = 4 * 1024 * 1024;


// #[aoc(day10, part2)]
pub fn part2(input: &[i64]) -> i64{
    let mut cloned_input = input.to_vec();
    cloned_input.push(0);
    cloned_input.sort_unstable();
    let mut max = cloned_input.pop().unwrap() as i32;
    cloned_input.push(max as i64);
    max += 3;
    cloned_input.push(max as i64);

    // println!("{:?}", cloned_input);
    for (thing1, thing2) in cloned_input.iter().enumerate().group_by(|(i,x)| { if *i > 0 { cloned_input[i+0] - cloned_input[i-1] > 2 } else {false} }).into_iter() {
        // println!("{:?} {:?}", thing1, thing2.collect::<Vec<_>>());
    }

    // Spawn thread with explicit stack size
    let child = thread::Builder::new()
        .stack_size(STACK_SIZE)
        .spawn(move || { run_part2(cloned_input) })
        .unwrap();

    // Wait for thread to join
    let result = child.join().unwrap();
    // for item in result.iter() {
    //     println!("{:?}", item);
    // }
    // println!("{:?}", RUN_PART2.lock().unwrap());
    result.len() as i64

}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::{input_generator, part1, part2};

    const SAMPLE_1: &str = "16\n10\n15\n5\n1\n11\n7\n19\n6\n12\n4";

    const SAMPLE_2: &str = "28\n33\n18\n42\n31\n14\n46\n20\n48\n47\n24\n23\n49\n45\n19\n38\n39\n11\n1\n32\n25\n35\n8\n17\n7\n9\n4\n2\n34\n10\n3";

    #[test]
    fn sample1() {
        assert_eq!(part1(&input_generator(SAMPLE_1)), 7 * 5);
    }

    #[test]
    fn sample2() {
        assert_eq!(part1(&input_generator(SAMPLE_2)), 22 * 10 );
   }

    #[test]
    fn sample3() {
        assert_eq!(part2(&input_generator(SAMPLE_1)), 8);
    }

   //  #[test]
   //  fn sample4() {
   //      assert_eq!(part2(&input_generator(SAMPLE_2)), 19208);
   // }
}
