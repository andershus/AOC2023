use std::collections::BTreeMap;
use std::iter::zip;

fn part1(input: &str) -> usize {
    let mut result = 1;
    let races = BTreeMap::from_iter(input.lines().map(|line| {
        let (header, races_str) = line.split_once(": ").expect("should have :");
        let races_part = races_str
            .split_whitespace()
            .map(|x| x.parse::<u32>().expect("should be a number"))
            .collect::<Vec<u32>>();
        (header, races_part)
    }));
    for (time, distance) in zip(
        races.get("Time").expect("should have time"),
        races.get("Distance").expect("should have distance"),
    ) {
        let mut combinations = 0;
        for i in 0..*time + 1 {
            if i * (time - i) > *distance {
                combinations += 1;
            }
        }
        result *= combinations;
    }
    result
}

fn part2(input: &str) -> usize {
    let mut result = 1;
    let mut lines = input.lines();
    // this should be doable in a single loop, but I'm not sure how to do it
    let time = lines
        .next()
        .expect("non-empty input")
        .split_once(": ")
        .expect("should have :")
        .1
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join("")
        .parse::<usize>()
        .expect("should be a number");
    let distance = lines
        .next()
        .expect("non-empty input")
        .split_once(": ")
        .expect("should have :")
        .1
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join("")
        .parse::<usize>()
        .expect("should be a number");
    let mut combinations = 0;
    for i in 0..time + 1 {
        if i * (time - i) > distance {
            combinations += 1;
        }
    }
    result *= combinations;
    result
}

fn main() {
    let binding =
        std::fs::read_to_string("./input.txt").expect("Should have been able to read the file");
    let contents = binding.trim_start_matches("\n").trim_end_matches("\n");
    println!("{}", part1(&contents));
    println!("{}", part2(&contents));
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 288);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 71503);
    }
}
