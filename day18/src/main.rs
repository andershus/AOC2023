use itertools::Itertools;
use std::collections::HashSet;

fn collect_input_part1(input: &str) -> Vec<((isize, isize), usize)> {
    let mut result = Vec::new();
    for line in input.lines() {
        let parts = line.split_whitespace().collect::<Vec<_>>();
        result.push((
            match parts[0] {
                "R" => (1, 0),
                "D" => (0, -1),
                "L" => (-1, 0),
                "U" => (0, 1),
                _ => panic!("Invalid input"),
            },
            parts[1].parse::<usize>().unwrap(),
        ))
    }
    result
}

fn get_interior(
    trench: &Vec<(isize, isize)>,
    pos: &(isize, isize),
    dir: &(isize, isize),
    max_len: usize,
) -> Vec<(isize, isize)> {
    let mut result = Vec::new();
    let mut interior = false;
    let mut new_pos = *pos;
    for _ in 0..max_len {
        new_pos.0 += dir.0;
        new_pos.1 += dir.1;
        if trench.contains(&new_pos) {
            interior = true;
            break;
        }
        result.push(new_pos);
    }
    if interior {
        result
    } else {
        Vec::new()
    }
}

fn display_trench(positions: &HashSet<(isize, isize)>) -> () {
    let mut result = String::new();
    for y in (-9..1).rev() {
        for x in 0..7 {
            if positions.contains(&(x, y)) {
                result.push('#');
            } else {
                result.push('.');
            }
        }
        println!("{}", result);
        result.clear();
    }
}

fn part1(input: &str) -> usize {
    let mut pos = (0, 0);
    let mut trench = vec![pos];
    for line in collect_input_part1(input).iter() {
        let dir = line.0;
        for _ in 1..line.1 + 1 {
            pos.0 += dir.0;
            pos.1 += dir.1;
            trench.push(pos);
        }
    }
    let mut area = 0;
    for (p1, p2) in trench //[(1, 6), (3, 1), (7, 2), (4, 4), (8, 5), (1, 6)]
        .iter()
        .tuple_windows()
    {
        area += p1.0 * p2.1 - p1.1 * p2.0;
    }
    dbg!((area as f64 / 2.0).abs());
    dbg!((trench.len() - 1) as f64 / 2.0);

    // let mut trench_left = Vec::new();
    // let mut trench_right = Vec::new();
    // for line in collect_input_part1(input).iter() {
    //     let dir = line.0;
    //     trench_left.extend(get_interior(&trench, &pos, &(-dir.0, dir.1), 100));
    //     trench_right.extend(get_interior(&trench, &pos, &(dir.1, -dir.0), 100));
    //     for _ in 1..line.1 + 1 {
    //         pos.0 += dir.0;
    //         pos.1 += dir.1;
    //         trench_left.extend(get_interior(&trench, &pos, &(-dir.0, dir.1), 100));
    //         trench_right.extend(get_interior(&trench, &pos, &(dir.1, -dir.0), 100));
    //     }
    // }
    // let trench_hash = HashSet::<(isize, isize)>::from_iter(trench);
    // dbg!(&trench_hash.len());
    // dbg!(&trench_left);
    // dbg!(&trench_left.len());
    // let trench_left_hash = HashSet::from_iter(trench_left);
    // let trench_right_hash = HashSet::from_iter(trench_right);
    // dbg!(&trench_right_hash.len());
    // display_trench(&trench_hash);
    // display_trench(&trench_right_hash);
    // println!("{:?}",&trench_right);
    // (
    //     trench_hash.union(&trench_left_hash).count(),
    //     trench_hash.union(&trench_right_hash).count(),
    // )
    0
}

fn collect_input(input: &str) -> Vec<((isize, isize), usize, &str)> {
    let mut result = Vec::new();
    for line in input.lines() {
        let parts = line.split_whitespace().collect::<Vec<_>>();
        result.push((
            match parts[0] {
                "R" => (1, 0),
                "D" => (0, -1),
                "L" => (-1, 0),
                "U" => (0, 1),
                _ => panic!("Invalid input"),
            },
            parts[1].parse::<usize>().unwrap(),
            parts[2].trim_start_matches('(').trim_end_matches(')'),
        ))
    }
    result
}

fn part2(input: &str) -> usize {
    input.len()
}

fn main() {
    let binding =
        std::fs::read_to_string("./input.txt").expect("Should have been able to read the file");
    // remove trailing newline that should exist in every file
    let contents = binding.trim_end_matches("\n");
    println!("result part1: {:?}", part1(&contents));
    println!("result part2: {}", part2(&contents));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";

    // #[test]
    // fn test_interior() {
    //     assert_eq!(
    //         get_interior(&vec![(0, 0), (5, 0)], (0, 0), (1, 0), 20),
    //         vec![(1, 0), (2, 0), (3, 0), (4, 0)]
    //     );
    // }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&TEST_INPUT), 62);
    }

    // too low: 7316, 15077, 36521

    // #[test]
    // fn test_part2() {
    //     assert_eq!(part2(&TEST_INPUT), 51);
    // }
}
