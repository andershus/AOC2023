use itertools::Itertools;
use std::collections::BTreeMap;

fn part1(input: &str) -> usize {
    let mut galaxy: Vec<Vec<char>> = Vec::new();
    for line_str in input.lines() {
        let line: Vec<char> = line_str.chars().collect();
        if line.iter().all(|pos| *pos == '.') {
            galaxy.push(line.clone());
            galaxy.push(line.clone());
        } else {
            galaxy.push(line);
        }
    }
    let mut transposed = (0..galaxy[0].len()).map(|_| vec![]).collect::<Vec<_>>();

    for original_row in galaxy {
        for (item, transposed_row) in original_row.into_iter().zip(&mut transposed) {
            transposed_row.push(item);
        }
    }
    let mut galaxy: Vec<Vec<char>> = Vec::new();
    for original_line in transposed.iter() {
        let line: Vec<char> = original_line.clone();
        if line.iter().all(|pos| *pos == '.') {
            galaxy.push(line.clone());
            galaxy.push(line.clone());
        } else {
            galaxy.push(line);
        }
    }
    let mut galaxy_locations = Vec::new();
    for (i, line) in galaxy.iter().enumerate() {
        for (j, c) in line.iter().enumerate() {
            if *c == '#' {
                galaxy_locations.push((i, j));
            }
        }
    }
    let mut result = 0;
    for pair in galaxy_locations.iter().combinations(2) {
        let first = pair.first().expect("combinations contain two elements");
        let last = pair.last().expect("combinations contain two elements");
        result += first.0.abs_diff(last.0) + first.1.abs_diff(last.1);
    }
    result
}

fn part2(input: &str, scale: u32) -> usize {
    let original_galaxy: Vec<Vec<char>> =
        input.lines().map(|line| line.chars().collect()).collect();
    let mut galaxy_locations = BTreeMap::new();
    let mut i_off = 0;
    for (i, line) in original_galaxy.iter().enumerate() {
        if line.iter().all(|pos| *pos == '.') {
            i_off += scale - 1;
        }
        for (j, c) in line.iter().enumerate() {
            if *c == '#' {
                galaxy_locations.insert((i, j), (i + i_off as usize, j));
            }
        }
    }

    let mut transposed = (0..original_galaxy[0].len())
        .map(|_| vec![])
        .collect::<Vec<_>>();
    for original_row in original_galaxy {
        for (item, transposed_row) in original_row.into_iter().zip(&mut transposed) {
            transposed_row.push(item);
        }
    }

    let mut j_off = 0;
    for (j, line) in transposed.iter().enumerate() {
        if line.iter().all(|pos| *pos == '.') {
            j_off += scale - 1;
        }
        for (i, c) in line.iter().enumerate() {
            if *c == '#' {
                let location = galaxy_locations
                    .get_mut(&(i, j))
                    .expect("should be a galaxy in original map"); //, (i+i_off as usize,j));
                *location = (location.0, location.1 + j_off as usize);
            }
        }
    }
    let mut result = 0;
    for pair in galaxy_locations.values().into_iter().combinations(2) {
        let first = pair.first().expect("combinations contain two elements");
        let last = pair.last().expect("combinations contain two elements");
        result += first.0.abs_diff(last.0) + first.1.abs_diff(last.1);
    }
    result
}

fn main() {
    let binding =
        std::fs::read_to_string("./input.txt").expect("Should have been able to read the file");
    let contents = binding.trim_start_matches("\n").trim_end_matches("\n");
    println!("result part1: {}", part1(&contents));
    println!("result part2: {}", part2(&contents, 1000000));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&TEST_INPUT), 374);
    }

    #[test]
    fn test_part2_10() {
        assert_eq!(part2(&TEST_INPUT, 10), 1030);
    }

    #[test]
    fn test_part2_100() {
        assert_eq!(part2(&TEST_INPUT, 100), 8410);
    }
}
