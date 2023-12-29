use std::collections::HashSet;

fn collect_input(input: &str) -> (Vec<Vec<char>>, (usize, usize)) {
    let mut start_pos = (0, 0);
    let grid = input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            line.chars()
                .enumerate()
                .map(|(j, c)| {
                    if c == 'S' {
                        start_pos = (i, j)
                    }
                    c
                })
                .collect()
        })
        .collect();
    (grid, start_pos)
}

fn part1(input: &str, steps: usize) -> usize {
    let (grid, start_pos) = collect_input(&input);
    let mut current_pos = HashSet::from([start_pos]);
    for _ in 0..steps {
        let mut next_pos = HashSet::new();
        for pos in current_pos {
            for new_pos in [
                (pos.0 - 1, pos.1),
                (pos.0, pos.1 - 1),
                (pos.0, pos.1 + 1),
                (pos.0 + 1, pos.1),
            ] {
                if (0..grid[0].len()).contains(&new_pos.0)
                    && (0..grid.len()).contains(&new_pos.1)
                    && grid[new_pos.1][new_pos.0] != '#'
                {
                    next_pos.insert(new_pos);
                }
            }
        }
        current_pos = next_pos;
    }
    current_pos.len()
}

fn collect_input2(input: &str) -> (Vec<Vec<char>>, (isize, isize)) {
    let mut start_pos = (0isize, 0isize);
    let grid = input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            line.chars()
                .enumerate()
                .map(|(j, c)| {
                    if c == 'S' {
                        start_pos = (i as isize, j as isize)
                    }
                    c
                })
                .collect()
        })
        .collect();
    (grid, start_pos)
}

fn part2(input: &str, steps: usize) -> usize {
    let (grid, start_pos) = collect_input2(&input);
    let mut current_pos = HashSet::from([start_pos]);
    for i in 0..steps {
        if i * 100 / steps % 10 == 0 {
            println!("finished {}%", i * 100 / steps);
        }
        let mut next_pos = HashSet::new();
        for pos in current_pos {
            for new_pos in [
                (pos.0 - 1, pos.1),
                (pos.0, pos.1 - 1),
                (pos.0, pos.1 + 1),
                (pos.0 + 1, pos.1),
            ] {
                let wrapped_y = (((new_pos.1 % grid.len() as isize) + grid.len() as isize)
                    % grid.len() as isize) as usize;
                let wrapped_x = (((new_pos.0 % grid[0].len() as isize) + grid[0].len() as isize)
                    % grid[0].len() as isize) as usize;
                if grid[wrapped_y][wrapped_x] != '#' {
                    next_pos.insert(new_pos);
                }
            }
        }
        current_pos = next_pos;
    }
    current_pos.len()
}

fn main() {
    let binding =
        std::fs::read_to_string("./input.txt").expect("Should have been able to read the file");
    // remove trailing newline that should exist in every file
    let contents = binding.trim_end_matches("\n");
    println!("result part1: {:?}", part1(&contents, 64));
    println!("result part2: {:?}", part2(&contents, 26501365));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";

    // #[test]
    // fn test_part1() {
    //     assert_eq!(part1(&TEST_INPUT, 6), 16);
    // }

    // #[test]
    // fn test_part2() {
    //     assert_eq!(part2(&TEST_INPUT, 10), 50);
    // }

    // #[test]
    // fn test_part2_1() {
    //     assert_eq!(part2(&TEST_INPUT, 50), 1594);
    // }

    #[test]
    fn test_part2_2() {
        assert_eq!(part2(&TEST_INPUT, 100), 6536);
    }

    // #[test]
    // fn test_part2_3() {
    //     assert_eq!(part2(&TEST_INPUT, 500), 167004);
    // }

    // #[test]
    // fn test_part2_4() {
    //     assert_eq!(part2(&TEST_INPUT, 1000), 668697);
    // }

    // #[test]
    // fn test_part2_5() {
    //     assert_eq!(part2(&TEST_INPUT, 500), 16733044);
    // }
}
