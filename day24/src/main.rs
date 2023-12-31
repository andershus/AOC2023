// use std::collections::HashMap;
use itertools::Itertools;

fn part1(input: &str, test_min: usize, test_max: usize) -> usize {
    let hailstones: Vec<((isize, isize), (isize, isize))> = input
        .lines()
        .map(|line| {
            let (pos_str, dir_str) = line.split_once(" @ ").unwrap();
            (
                pos_str
                    .split(", ")
                    .map(|v| v.trim().parse::<isize>().unwrap())
                    .collect_tuple()
                    .unwrap(),
                dir_str
                    .split(", ")
                    .map(|v| v.trim().parse::<isize>().unwrap())
                    .collect_tuple()
                    .unwrap(),
            )
            // let mut pos = Vec::new();
            // let mut dir = Vec::new();
            // for (p, d) in pos_str.split(", ").zip(dir_str.split(", ")) {
            //     pos.push(p.trim().parse::<isize>().unwrap());
            //     dir.push(d.trim().parse::<isize>().unwrap());
            // }
            // ((pos[0], pos[1]), (dir[0], dir[1]))
        })
        .collect();
    dbg!(&hailstones);
    // let mut positions = Vec::new();
    for ((p1, d1), (p2, d2)) in hailstones
        .iter()
        .combinations(2)
        .map(|pair| (*pair.first().unwrap(), *pair.last().unwrap()))
    {
        let v1 = (p2.0 - p1.0, p2.1 - p1.1);
        // let v2: (d2.1, -d2.0);
        break;
    }
    0
}

fn part2(input: &str) -> usize {
    input.len()
}

fn main() {
    let binding =
        std::fs::read_to_string("./input.txt").expect("Should have been able to read the file");
    let contents = binding.trim_start_matches("\n").trim_end_matches("\n");
    println!(
        "result part1: {}",
        part1(&contents, 200000000000000, 400000000000000)
    );
    println!("result part2: {}", part2(&contents));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(
                "19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3",
                7,
                27
            ),
            94
        );
    }
}
