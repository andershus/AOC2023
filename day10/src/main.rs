use std::collections::{HashMap, HashSet};

fn part1(input: &str, starts: Vec<((usize, usize), (isize, isize), usize)>) -> usize {
    let mut start_pos = (0, 0);
    let grid: Vec<Vec<char>> = input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            line.chars()
                .enumerate()
                .map(|(j, c)| {
                    if c == 'S' {
                        start_pos = (j, i);
                    }
                    c
                })
                .collect()
        })
        .collect();
    dbg!(start_pos);
    let mut seen = HashMap::new();
    // should find these programatically
    let mut queue = starts;
    loop {
        let (pos, dir, dist) = queue.remove(0);
        seen.insert(pos, dist);
        let next_dir = match grid[pos.1][pos.0] {
            '|' | '-' => dir,
            'L' | 'J' if dir.1 == 0 => (0, -1),
            'L' | 'F' if dir.0 == 0 => (1, 0),
            '7' | 'F' if dir.1 == 0 => (0, 1),
            '7' | 'J' if dir.0 == 0 => (-1, 0),
            _ => panic!("not implemented for {} at {:?}", grid[pos.1][pos.0], pos),
        };
        let next_pos = (
            pos.0.checked_add_signed(next_dir.0).unwrap(),
            pos.1.checked_add_signed(next_dir.1).unwrap(),
        );
        // dbg!(&seen, &next);
        if seen.contains_key(&next_pos) {
            break;
        }
        queue.push((next_pos, next_dir, dist + 1));
    }
    *seen.iter().map(|(_, v)| v).max().unwrap()
}

fn display_grid_vec(
    positions: &Vec<(usize, usize)>,
    positions_char: char,
    grid: &Vec<Vec<char>>,
    display_grid: bool,
) -> () {
    // let mut print_line = String::new();
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if positions.contains(&(x, y)) {
                print!("{positions_char}");
            } else {
                if display_grid {
                    print!("{}", grid[y][x]);
                } else {
                    print!(" ");
                }
            }
        }
        println!();
    }
}

fn display_grid(
    positions: &HashSet<(usize, usize)>,
    positions_char: char,
    grid: &Vec<Vec<char>>,
    display_grid: bool,
) -> () {
    // let mut print_line = String::new();
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if positions.contains(&(x, y)) {
                print!("{positions_char}");
            } else {
                if display_grid {
                    print!("{}", grid[y][x]);
                } else {
                    print!(" ");
                }
            }
        }
        println!();
    }
}

fn part2(input: &str, start_symbol: char) -> usize {
    let mut start_pos = (0, 0);
    let mut grid: Vec<Vec<char>> = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| {
                    if c == 'S' {
                        start_pos = (x, y);
                        start_symbol
                    } else {
                        c
                    }
                })
                .collect()
        })
        .collect();
    // should find these programatically
    let start_dir: (isize, isize) = match start_symbol {
        '-' | 'F' | 'L' => (1, 0),
        '|' | '7' => (0, 1),
        'J' => (-1, 0),
        _ => panic!("Not implemented for {}", start_symbol),
    };
    let mut seen = HashSet::new();
    let mut queue = vec![(start_pos, start_dir)];
    loop {
        let (pos, dir) = queue.remove(0);
        seen.insert(pos);
        let next_dir = match grid[pos.1][pos.0] {
            '|' | '-' => dir,
            'L' | 'J' if dir.1 == 0 => (0, -1),
            'L' | 'F' if dir.0 == 0 => (1, 0),
            '7' | 'F' if dir.1 == 0 => (0, 1),
            '7' | 'J' if dir.0 == 0 => (-1, 0),
            _ => panic!("not implemented for {} at {:?}", grid[pos.1][pos.0], pos),
        };
        let next_pos = (
            pos.0.checked_add_signed(next_dir.0).unwrap(),
            pos.1.checked_add_signed(next_dir.1).unwrap(),
        );
        // dbg!(&seen, &next);
        if seen.contains(&next_pos) {
            break;
        }
        queue.push((next_pos, next_dir));
    }
    let mut empty_pos = Vec::new();
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if !seen.contains(&(x, y)) {
                grid[y][x] = '.';
                empty_pos.push((x, y));
            }
        }
    }
    let corners = ['F', 'J', 'L', '7'];
    let mut results = Vec::new();
    for pos in empty_pos {
        let mut hits = 0;
        let mut new_pos = pos;
        let mut last_corner = '.';
        loop {
            let symbol = grid[new_pos.1][new_pos.0];
            if symbol == '|' {
                hits += 1;
            } else if corners.contains(&symbol) {
                if last_corner != '.' {
                    hits += match (symbol, last_corner) {
                        ('F', 'J') | ('L', '7') => 1,
                        _ => 0,
                    };
                    last_corner = '.';
                } else {
                    last_corner = symbol;
                }
            }
            if new_pos.0 > 0 {
                new_pos.0 -= 1;
            } else {
                break;
            }
        }
        if hits % 2 == 1 {
            results.push(pos);
        }
    }
    results.len()
}

fn main() {
    let binding =
        std::fs::read_to_string("./input.txt").expect("Should have been able to read the file");
    let contents = binding.trim_start_matches("\n").trim_end_matches("\n");
    println!(
        "result part1: {}",
        part1(
            &contents,
            vec![
                ((90, 97), (0isize, -1isize), 1),
                ((90, 99), (0isize, 1isize), 1)
            ]
        )
    );
    println!(
        "result part2: {}",
        part2(
            &contents, '|',
            // ((90, 97), (0isize, -1isize))
        )
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(
                ".....
.S-7.
.|.|.
.L-J.
.....",
                vec![((1, 2), (0isize, 1isize), 1), ((2, 1), (1isize, 0isize), 1)]
            ),
            4
        );
    }

    #[test]
    fn test_part12() {
        assert_eq!(
            part1(
                "-L|F7
7S-7|
L|7||
-L-J|
L|-JF",
                vec![((1, 2), (0isize, 1isize), 1), ((2, 1), (1isize, 0isize), 1)]
            ),
            4
        );
    }

    #[test]
    fn test_part13() {
        assert_eq!(
            part1(
                "..F7.
.FJ|.
SJ.L7
|F--J
LJ...",
                vec![((1, 2), (1isize, 0isize), 1), ((0, 3), (0isize, 1isize), 1)]
            ),
            8
        );
    }

    #[test]
    fn test_part14() {
        assert_eq!(
            part1(
                "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ",
                vec![((1, 2), (1isize, 0isize), 1), ((0, 3), (0isize, 1isize), 1)]
            ),
            8
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(
                "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........",
                'F',
            ),
            4
        );
    }

    #[test]
    fn test_part21() {
        assert_eq!(
            part2(
                "..........
.S------7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
..........",
                'F',
            ),
            4
        );
    }

    #[test]
    fn test_part22() {
        assert_eq!(
            part2(
                ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...",
                'F',
            ),
            8
        );
    }

    #[test]
    fn test_part23() {
        assert_eq!(
            part2(
                "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L",
                'J',
            ),
            10
        );
    }
}
