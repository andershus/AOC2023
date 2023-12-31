use std::collections::{HashMap, HashSet};

fn display_grid(positions: &HashSet<(usize, usize)>, grid: &Vec<Vec<char>>) -> () {
    // let mut print_line = String::new();
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if positions.contains(&(x, y)) {
                print!("{}", 'O');
            } else {
                print!(" ");
                // print!("{}", grid[y][x]);
            }
        }
        println!();
        // println!("{}", print_line);
        // print_line.clear();
    }
    println!();
}

fn part1(input: &str) -> usize {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let start_pos = (1usize, 0usize);
    let first_step = (1usize, 1usize);
    let mut seen = HashMap::from([(start_pos, 0), (first_step, 1)]);
    let mut queue = vec![(first_step, start_pos)];
    while !queue.is_empty() {
        let (current, previous_step) = queue.pop().unwrap();
        let next_step_count = seen.get(&current).unwrap() + 1;
        if current == (grid.len() - 2, grid[0].len() - 1) {
            continue;
        }
        for dir in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
            let next_step = (
                current.0.checked_add_signed(dir.0).unwrap(),
                current.1.checked_add_signed(dir.1).unwrap(),
            );
            if next_step == previous_step {
                continue;
            }
            match grid[next_step.1][next_step.0] {
                '#' => (),
                '.' => {
                    queue.push((next_step, current));
                    seen.insert(next_step, next_step_count);
                }
                'v' => {
                    if next_step.1 < current.1 {
                        continue;
                    }
                    let downhill = (
                        next_step.0.checked_add_signed(0).unwrap(),
                        next_step.1.checked_add_signed(1).unwrap(),
                    );
                    queue.push((downhill, next_step));
                    seen.insert(next_step, next_step_count);
                    seen.insert(downhill, next_step_count + 1);
                }
                '>' => {
                    if next_step.0 < current.0 {
                        continue;
                    }
                    let downhill = (
                        next_step.0.checked_add_signed(1).unwrap(),
                        next_step.1.checked_add_signed(0).unwrap(),
                    );
                    queue.push((downhill, next_step));
                    seen.insert(next_step, next_step_count);
                    seen.insert(downhill, next_step_count + 1);
                }
                '<' => {
                    if next_step.0 > current.0 {
                        continue;
                    }
                    let downhill = (
                        next_step.0.checked_add_signed(-1).unwrap(),
                        next_step.1.checked_add_signed(0).unwrap(),
                    );
                    queue.push((downhill, next_step));
                    seen.insert(next_step, next_step_count);
                    seen.insert(downhill, next_step_count + 1);
                }
                '^' => {
                    if next_step.1 > current.1 {
                        continue;
                    }
                    let downhill = (
                        next_step.0.checked_add_signed(0).unwrap(),
                        next_step.1.checked_add_signed(-1).unwrap(),
                    );
                    queue.push((downhill, next_step));
                    seen.insert(next_step, next_step_count);
                    seen.insert(downhill, next_step_count + 1);
                }
                _ => panic!(
                    "unexpected match {} at {:?}",
                    grid[next_step.1][next_step.0], next_step
                ),
            }
        }
    }
    *seen.iter().map(|(_, v)| v).max().unwrap()
}

fn part2(input: &str) -> usize {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let start_pos = (1usize, 0usize);
    let first_step = (1usize, 1usize);
    let mut max_dist = 0;
    let mut paths_unexplored = vec![(vec![(first_step, 1)], HashSet::from([start_pos]))];
    while !paths_unexplored.is_empty() {
        let (mut queue, mut seen) = paths_unexplored.remove(0);
        while !queue.is_empty() {
            let (current, step_count) = queue.pop().unwrap();
            seen.insert(current);
            if current == (grid.len() - 2, grid[0].len() - 1) {
                display_grid(&seen, &grid);
                if step_count > max_dist {
                    dbg!(step_count, paths_unexplored.len());
                    max_dist = step_count;
                }
                break;
            }
            let mut potentials = Vec::new();
            for dir in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
                let next_step = (
                    current.0.checked_add_signed(dir.0).unwrap(),
                    current.1.checked_add_signed(dir.1).unwrap(),
                );
                if seen.contains(&next_step) {
                    continue;
                }
                match grid[next_step.1][next_step.0] {
                    '#' => (),
                    _ => {
                        potentials.push(next_step);
                    }
                }
            }
            if potentials.is_empty() {
                continue;
            }
            queue.push((potentials.remove(0), step_count + 1));
            if potentials.len() > 0 {
                for step in potentials {
                    paths_unexplored.push((vec![(step, step_count + 1)], seen.clone()))
                }
            }
        }
    }
    max_dist
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

    const TEST_INPUT: &str = "#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#";

    // #[test]
    // fn test_part1() {
    //     assert_eq!(part1(&TEST_INPUT), 94);
    // }

    #[test]
    fn test_part1() {
        assert_eq!(part2(&TEST_INPUT), 154);
    }

    //too low: 4526
}
