#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Vec<Vec<char>> {
    input.split("\n").map(|x| x.chars().collect()).collect::<Vec<Vec<char>>>()
}

#[aoc(day4, part1)]
pub fn part1(input: &[Vec<char>]) -> u32 {
    let mut total: u32 = 0;
    for (i, line) in input.iter().enumerate() {
        for (j, letter) in line.iter().enumerate() {
            if *letter == 'X' {
                if j > 2 {
                    if input[i][j-1] == 'M' && input[i][j-2] == 'A' && input[i][j-3] == 'S' {
                        // println!("{} {}", i, j);
                        total+=1;
                    }
                    if i > 2 {
                        if input[i-1][j-1] == 'M' && input[i-2][j-2] == 'A' && input[i-3][j-3] == 'S' {
                            // println!("{} {}", i, j);
                            total+=1;
                        }
                    }
                    if i + 3 < input.len() {
                        if input[i+1][j-1] == 'M' && input[i+2][j-2] == 'A' && input[i+3][j-3] == 'S' {
                            // println!("{} {}", i, j);
                            total+=1;
                        }
                    }
                }
                if j + 3 < line.len() {
                    if input[i][j+1] == 'M' && input[i][j+2] == 'A' && input[i][j+3] == 'S' {
                        // println!("{} {}", i, j);
                        total+=1;
                    }
                    if i > 2 {
                        if input[i-1][j+1] == 'M' && input[i-2][j+2] == 'A' && input[i-3][j+3] == 'S' {
                            // println!("{} {}", i, j);
                            total+=1;
                        }
                    }
                    if i + 3 < input.len() {
                        if input[i+1][j+1] == 'M' && input[i+2][j+2] == 'A' && input[i+3][j+3] == 'S' {
                            // println!("{} {}", i, j);
                            total+=1;
                        }
                    }
                }
                if i > 2 {
                    if input[i-1][j] == 'M' && input[i-2][j] == 'A' && input[i-3][j] == 'S' {
                        // println!("{} {}", i, j);
                        total+=1;
                    }
                }
                if i + 3 < input.len() {
                    if input[i+1][j] == 'M' && input[i+2][j] == 'A' && input[i+3][j] == 'S' {
                        // println!("{} {}", i, j);
                        total+=1;
                    }
                }
            }
        }
    }
    total
}


// 2961 is too high
// 2670 is too high

#[aoc(day4, part2)]
pub fn part2(input: &[Vec<char>]) -> u32 {
    let mut total: u32 = 0;
    for (i, line) in input.iter().enumerate() {
        for (j, letter) in line.iter().enumerate() {
            if *letter == 'A' {
                if j > 0 && j + 1 < line.len() {
                    if i > 0 && i + 1 < input.len() {
                        if ( input[i-1][j-1] == 'M' && input[i+1][j+1] == 'S' ) || ( input[i-1][j-1] == 'S' &&  input[i+1][j+1] == 'M' ) {
                            if ( input[i-1][j+1] == 'M' && input[i+1][j-1] == 'S' ) || ( input[i-1][j+1] == 'S' &&  input[i+1][j-1] == 'M' ) {
                                println!("{} {}", i, j);
                                total+=1;
                            }
                        }
                    }
                }
            }
        }
    }
    total
}


#[cfg(test)]
mod tests {
    use super::{input_generator, part1, part2};

    const SAMPLES: &[(&str, u32, u32)] = &[
        (r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"#, 18, 9)];

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
