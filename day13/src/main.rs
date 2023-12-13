fn find_reflection(grid: &Vec<Vec<char>>) -> usize {
    let mut reflection_lines = 0;
    for i in 0..grid.len() - 1 {
        let mut reflection = true;
        for j in 0..i + 1 {
            if i + 1 + j >= grid.len() {
                // the rest of the grid is missing and assumed to match
                break;
            }
            if grid[i - j] != grid[i + 1 + j] {
                reflection = false;
                break;
            }
        }
        if reflection {
            reflection_lines += i + 1;
        }
    }
    reflection_lines
}

fn part1(input: &str) -> usize {
    let mut result = 0;
    let grids = input
        .split("\n\n")
        .map(|grid| {
            grid.lines()
                .map(|line| line.chars().collect::<Vec<char>>())
                .collect::<Vec<Vec<char>>>()
        })
        .collect::<Vec<_>>();
    for grid in grids.iter() {
        result += 100 * find_reflection(&grid);
        // find vertical lines, not clear if a grid can have both vertical and horizontal reflections
        let transposed = (0..grid[0].len())
            .map(|j| (0..grid.len()).map(|i| grid[i][j]).collect::<Vec<_>>())
            .collect::<Vec<_>>();
        result += find_reflection(&transposed);
    }
    result
}

fn find_smudge_reflection(grid: &Vec<Vec<u8>>) -> usize {
    let mut reflection_lines = 0;
    // find horizontal lines
    for i in 0..grid.len() - 1 {
        let mut smudge_index = 0;
        for j in 0..i + 1 {
            if i + 1 + j >= grid.len() {
                // the rest of the grid is missing and assumed to match
                break;
            }
            smudge_index += grid[i - j]
                .iter()
                .zip(grid[i + 1 + j].iter())
                .map(|(a, b)| a ^ b)
                .sum::<u8>();
        }
        // this can also be used for part1 with smudge_index == 0
        if smudge_index == 1 {
            reflection_lines += i + 1;
        }
    }
    reflection_lines
}

fn part2(input: &str) -> usize {
    let mut result = 0;
    let grids = input
        .split("\n\n")
        .map(|grid| {
            grid.lines()
                .map(|line| {
                    line.chars()
                        .map(|c| match c {
                            '.' => 0,
                            '#' => 1,
                            _ => panic!(),
                        })
                        .collect::<Vec<u8>>()
                })
                .collect::<Vec<Vec<u8>>>()
        })
        .collect::<Vec<_>>();
    for grid in grids.iter() {
        result += 100 * find_smudge_reflection(&grid);
        // find vertical lines, not clear if a grid can have both vertical and horizontal reflections
        let transposed = (0..grid[0].len())
            .map(|j| (0..grid.len()).map(|i| grid[i][j]).collect::<Vec<_>>())
            .collect::<Vec<_>>();
        result += find_smudge_reflection(&transposed);
    }
    result
}

fn main() {
    let binding =
        std::fs::read_to_string("./input.txt").expect("Should have been able to read the file");
    let contents = binding.trim_start_matches("\n").trim_end_matches("\n");
    println!("result part1: {}", part1(&contents));
    println!("result part2: {}", part2(&contents));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&TEST_INPUT), 405);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&TEST_INPUT), 400);
    }
}
