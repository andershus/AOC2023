// use std::collections::HashMap;
use itertools::Itertools;

fn part1(input: &str, test_min: usize, test_max: usize) -> usize {
    let hailstones: Vec<((isize, isize), (isize, isize))> = input
        .lines()
        .map(|line| {
            line.split(" @ ")
                .map(|part| {
                    let (x, y, _) = part
                        .split(", ")
                        .map(|v| v.trim().parse::<isize>().unwrap())
                        .collect_tuple()
                        .unwrap();
                    (x, y)
                })
                .collect_tuple()
                .unwrap()
        })
        .collect();
    let mut result = 0;
    for ((p, d1), (q, d2)) in hailstones
        .iter()
        .combinations(2)
        .map(|pair| (*pair.first().unwrap(), *pair.last().unwrap()))
    {
        let d2_len = ((d2.0 * d2.0 + d2.1 * d2.1) as f64).sqrt();
        let s = (d2.0 as f64 / d2_len, d2.1 as f64 / d2_len);
        let sperp = (s.1, -s.0);
        let d1_len = ((d1.0 * d1.0 + d1.1 * d1.1) as f64).sqrt();
        let r = (d1.0 as f64 / d1_len, d1.1 as f64 / d1_len);
        let r_dot_sperp = r.0 * sperp.0 + r.1 * sperp.1;
        if r_dot_sperp == 0.0 {
            // paths are parallell
            continue;
        }
        let qp = (q.0 - p.0, q.1 - p.1);
        let rperp = (r.1, -r.0);
        let qp_dot_sperp = qp.0 as f64 * sperp.0 + qp.1 as f64 * sperp.1;
        let qp_dot_rperp = qp.0 as f64 * rperp.0 + qp.1 as f64 * rperp.1;
        let t = qp_dot_sperp / r_dot_sperp;
        let u = qp_dot_rperp * r_dot_sperp;
        if t < 0.0 || u < 0.0 {
            // meet in the past
            continue;
        }
        let v = (p.0 as f64 + t * r.0, p.1 as f64 + t * r.1);
        if ((test_min as f64) <= v.0)
            && (v.0 <= (test_max as f64))
            && ((test_min as f64) <= v.1)
            && (v.1 <= (test_max as f64))
        {
            result += 1;
        }
    }
    result
}

fn part2(input: &str) -> usize {
    let hailstones: Vec<((isize, isize, isize), (isize, isize, isize))> = input
        .lines()
        .map(|line| {
            line.split(" @ ")
                .map(|part| {
                    part.split(", ")
                        .map(|v| v.trim().parse::<isize>().unwrap())
                        .collect_tuple()
                        .unwrap()
                })
                .collect_tuple()
                .unwrap()
        })
        .collect();
    dbg!(hailstones);
    let start_pos = (24, 13, 10);
    // do something to start_pos
    start_pos.0 + start_pos.1 + start_pos.2
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
            2
        );
    }
    #[test]
    fn test_part2() {
        assert_eq!(
            part2(
                "19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3"
            ),
            47
        );
    }
}
