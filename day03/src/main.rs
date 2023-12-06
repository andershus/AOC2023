fn part1(input: &str) -> u32 {
    let mut result = 0;
    let chars: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let num_lines = chars.len() as i32;
    for (line_num, line) in input.lines().enumerate() {
        let line_len = line.len() as i32;
        let mut potential_num = "".to_string();
        let mut is_surrounded = false;
        for (line_pos, c) in line.chars().enumerate() {
            if c.is_numeric() {
                potential_num.push(c);
                for i in -1..2 {
                    let i_pos = line_num as i32 + i;
                    for j in -1..2 {
                        let j_pos = line_pos as i32 + j;
                        if (i_pos >= 0) && (i_pos < num_lines) && (j_pos >= 0) && (j_pos < line_len)
                        {
                            let surrounding_char = chars[i_pos as usize][j_pos as usize];
                            if (surrounding_char != '.') && (!surrounding_char.is_numeric()) {
                                is_surrounded = true;
                            }
                        }
                    }
                }
            } else {
                if is_surrounded {
                    result += potential_num.parse::<u32>().expect("should be num");
                }
                // reset for next number
                potential_num = "".to_string();
                is_surrounded = false;
            }
        }
        // Also need to add surrounded numbers at end of line
        if is_surrounded {
            result += potential_num.parse::<u32>().expect("should be num");
        }
    }
    result
}

fn part2(input: &str) -> u32 {
    let mut result = 0;
    let chars: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let num_lines = chars.len() as i32;
    let line_len = input.lines().next().expect("len > 0").len() as i32;
    let mut positions: Vec<Vec<(u8, u32)>> = vec![vec![(0, 1); line_len as usize]; chars.len()];
    for (line_num, line) in input.lines().enumerate() {
        let mut potential_num = "".to_string();
        let mut is_surrounded = false;
        let mut gear_i_pos = 0;
        let mut gear_j_pos = 0;
        for (line_pos, c) in line.chars().enumerate() {
            if c.is_numeric() {
                potential_num.push(c);
                let mut surrounding_char = '.';
                for i in -1..2 {
                    let i_pos = line_num as i32 + i;
                    for j in -1..2 {
                        let j_pos = line_pos as i32 + j;
                        if (i_pos >= 0) && (i_pos < num_lines) && (j_pos >= 0) && (j_pos < line_len)
                        {
                            surrounding_char = chars[i_pos as usize][j_pos as usize];
                        }
                        if surrounding_char == '*' {
                            gear_i_pos = i_pos as usize;
                            gear_j_pos = j_pos as usize;
                            is_surrounded = true;
                        }
                    }
                }
            } else {
                if is_surrounded {
                    positions[gear_i_pos][gear_j_pos].0 += 1;
                    positions[gear_i_pos][gear_j_pos].1 *=
                        potential_num.parse::<u32>().expect("should be num")
                }
                // reset for next number
                potential_num = "".to_string();
                is_surrounded = false;
            }
        }
        // Also need to add surrounded numbers at end of line
        if is_surrounded {
            positions[gear_i_pos][gear_j_pos].0 += 1;
            positions[gear_i_pos][gear_j_pos].1 *=
                potential_num.parse::<u32>().expect("should be num")
        }
    }
    for row in positions.iter() {
        for pos in row.iter() {
            if pos.0 == 2 {
                result += pos.1;
            }
        }
    }
    result
}

fn main() {
    let binding =
        std::fs::read_to_string("./input.txt").expect("Should have been able to read the file");
    let contents = binding.trim_start_matches("\n").trim_end_matches("\n");
    println!("{}", part1(&contents));
    println!("{}", part2(&contents));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        let test_input = "
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."
            .trim_start_matches("\n")
            .trim_end_matches("\n");
        assert_eq!(part1(&test_input), 4361);
    }

    #[test]
    fn test_part2() {
        let test_input = "
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."
            .trim_start_matches("\n");
        assert_eq!(part2(&test_input), 467835);
    }
}
