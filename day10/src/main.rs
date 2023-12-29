use std::collections::HashMap;

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
        let next_pos = (pos.0.checked_add_signed(next_dir.0).unwrap(), pos.1.checked_add_signed(next_dir.1).unwrap());
        // dbg!(&seen, &next);
        if seen.contains_key(&next_pos) {
            break;
        }
        queue.push((next_pos, next_dir, dist + 1));
    }
    *seen.iter().map(|(_,v)| v).max().unwrap()

}

fn part2(input: &str) -> usize {
    input.len()
}

fn main() {
    let binding =
        std::fs::read_to_string("./input.txt").expect("Should have been able to read the file");
    let contents = binding.trim_start_matches("\n").trim_end_matches("\n");
    println!("result part1: {}", part1(&contents, vec![((90, 97), (0isize, -1isize), 1), ((90, 99), (0isize, 1isize), 1)]));
    println!("result part2: {}", part2(&contents));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(".....
.S-7.
.|.|.
.L-J.
.....", vec![((1, 2), (0isize, 1isize), 1), ((2, 1), (1isize, 0isize), 1)]), 4);
    }

    #[test]
    fn test_part12() {
        assert_eq!(part1("-L|F7
7S-7|
L|7||
-L-J|
L|-JF", vec![((1, 2), (0isize, 1isize), 1), ((2, 1), (1isize, 0isize), 1)]), 4);
    }

    #[test]
    fn test_part13() {
        assert_eq!(part1("..F7.
.FJ|.
SJ.L7
|F--J
LJ...", vec![((1, 2), (1isize, 0isize), 1), ((0, 3), (0isize, 1isize), 1)]), 8);
    }

    #[test]
    fn test_part14() {
        assert_eq!(part1("7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ", vec![((1, 2), (1isize, 0isize), 1), ((0, 3), (0isize, 1isize), 1)]), 8);
    }


    #[test]
    fn test_part2() {
        assert_eq!(part2("...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
..........."), 4);
    }
}
