// use std::collections::BTreeMap;

fn part1(input: &str) -> u32 {
    let mut result = 0;
    for line in input.lines() {
        let mut all_zeroes = false;
        let mut sequences = Vec::<Vec<i32>>::new();
        // skip the case when starting input is all zeroes
        let mut current: Vec<i32> = line
            .split_whitespace()
            .map(|num| {
                num.parse::<i32>()
                    .expect("should be a space seperated list of numbers")
            })
            .collect();
        sequences.push(current.clone());
        while !all_zeroes {
            let next: Vec<i32> = current
                .iter()
                .zip(current.iter().skip(1))
                .map(|(a, b)| b - a)
                .collect();
            sequences.push(next.clone());
            all_zeroes = next.iter().all(|n| *n == 0 as i32);
            current = next;
        }
        let mut first = true;
        let mut current = Vec::new();
        let mut previous = Vec::new();
        for sequence in &mut sequences.iter().rev() {
            if first {
                previous = sequence.clone();
                previous.push(0 as i32);
                first = false;
            } else {
                current = sequence.clone();
                current.push(
                    current.last().expect("current not empty")
                        + previous.last().expect("previous not empty"),
                );
                previous = current.clone();
            }
        }
        result += current.last().expect("current not empty");
    }
    result
        .try_into()
        .expect("all differences should be positive")
}

fn part2(input: &str) -> i32 {
    let mut result = 0;
    for line in input.lines() {
        let mut all_zeroes = false;
        let mut sequences = Vec::<Vec<i32>>::new();
        // skip the case when starting input is all zeroes
        // change from part 1: reverse each sequence
        let mut current: Vec<i32> = line
            .split_whitespace()
            .map(|num| {
                num.parse::<i32>()
                    .expect("should be a space seperated list of numbers")
            })
            .rev()
            .collect();
        sequences.push(current.clone());
        while !all_zeroes {
            // change from part 1: chang operation
            let next: Vec<i32> = current
                .iter()
                .zip(current.iter().skip(1))
                .map(|(a, b)| a - b)
                .collect();
            sequences.push(next.clone());
            all_zeroes = next.iter().all(|n| *n == 0 as i32);
            current = next;
        }
        let mut first = true;
        let mut current = Vec::new();
        let mut previous = Vec::new();
        for sequence in &mut sequences.iter().rev() {
            if first {
                previous = sequence.clone();
                previous.push(0 as i32);
                first = false;
            } else {
                current = sequence.clone();
                // change from part 1: change operation
                current.push(
                    current.last().expect("current not empty")
                        - previous.last().expect("previous not empty"),
                );
                previous = current.clone();
            }
        }
        result += current.last().expect("current not empty");
    }
    result
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

    const TEST_INPUT: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&TEST_INPUT), 114);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 2);
    }
}
