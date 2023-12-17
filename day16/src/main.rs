use std::collections::HashSet;

fn collect_input(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

fn lazorz(
    grid: &Vec<Vec<char>>,
    pos: (isize, isize),
    dir: (isize, isize),
    mut visited: HashSet<((usize, usize), (isize, isize))>,
) -> HashSet<((usize, usize), (isize, isize))> {
    if (pos.0 < 0)
        | (pos.1 < 0)
        | (grid.len() <= pos.0 as usize)
        | (grid[0].len() <= pos.1 as usize)
    {
        // outside of grid
        return visited;
    }
    if visited.contains(&((pos.0 as usize, pos.1 as usize), dir)) {
        // already visited from this direction so we stop to aviod infinite loop;
        return visited;
    }
    visited.insert(((pos.0 as usize, pos.1 as usize), dir));
    match grid[pos.1 as usize][pos.0 as usize] {
        '.' => {
            visited.extend(&lazorz(
                &grid,
                (pos.0 + dir.0, pos.1 + dir.1),
                dir,
                visited.clone(),
            ));
        }
        '|' => {
            if dir.0 != 0 {
                visited.extend(&lazorz(&grid, (pos.0, pos.1 - 1), (0, -1), visited.clone()));
                visited.extend(&lazorz(&grid, (pos.0, pos.1 + 1), (0, 1), visited.clone()));
            } else {
                visited.extend(&lazorz(
                    &grid,
                    (pos.0 + dir.0, pos.1 + dir.1),
                    dir,
                    visited.clone(),
                ));
            }
        }
        '-' => {
            if dir.1 != 0 {
                visited.extend(&lazorz(&grid, (pos.0 - 1, pos.1), (-1, 0), visited.clone()));
                visited.extend(&lazorz(&grid, (pos.0 + 1, pos.1), (1, 0), visited.clone()));
            } else {
                visited.extend(&lazorz(
                    &grid,
                    (pos.0 + dir.0, pos.1 + dir.1),
                    dir,
                    visited.clone(),
                ));
            }
        }
        '/' => {
            let new_dir = (-1 * dir.1, -1 * dir.0);
            visited.extend(&lazorz(
                &grid,
                (pos.0 + new_dir.0, pos.1 + new_dir.1),
                new_dir,
                visited.clone(),
            ));
        }
        '\\' => {
            let new_dir = (dir.1, dir.0);
            visited.extend(&lazorz(
                &grid,
                (pos.0 + new_dir.0, pos.1 + new_dir.1),
                new_dir,
                visited.clone(),
            ));
        }
        _ => {
            panic!(
                "not implemented for {:?}",
                grid[pos.1 as usize][pos.0 as usize]
            );
        }
    }
    visited
}

fn part1(input: &str) -> usize {
    let mut energized: HashSet<(usize, usize)> = HashSet::new();
    for visited in lazorz(&collect_input(input), (0, 0), (1, 0), HashSet::new()) {
        energized.insert(visited.0);
    }
    energized.len()
}

fn part2(input: &str) -> usize {
    let grid = collect_input(input);
    let mut energy = Vec::new();
    for y in 0..grid.len() {
        for x_speed in [-1, 1] {
            let mut energized: HashSet<(usize, usize)> = HashSet::new();
            let entry = if x_speed == 1 { 0 } else { grid.len() - 1 };
            for visited in lazorz(
                &grid,
                (entry as isize, y as isize),
                (x_speed, 0),
                HashSet::new(),
            ) {
                energized.insert(visited.0);
            }
            energy.push(energized.len())
        }
    }
    for x in 0..grid[0].len() {
        for y_speed in [-1, 1] {
            let mut energized: HashSet<(usize, usize)> = HashSet::new();
            let entry = if y_speed == 1 { 0 } else { grid[0].len() - 1 };
            for visited in lazorz(
                &grid,
                (x as isize, entry as isize),
                (0, y_speed),
                HashSet::new(),
            ) {
                energized.insert(visited.0);
            }
            energy.push(energized.len())
        }
    }
    *energy.iter().max().unwrap()
}

fn main() {
    let binding =
        std::fs::read_to_string("./input.txt").expect("Should have been able to read the file");
    // remove trailing newline that should exist in every file
    let contents = binding.trim_end_matches("\n");
    println!("result part1: {}", part1(&contents));
    println!("result part2: {}", part2(&contents));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&TEST_INPUT), 46);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&TEST_INPUT), 51);
    }
}
