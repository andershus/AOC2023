use std::collections;
use std::fs;

fn part1(input: &str) -> u32 {
    let mut result: u32 = 0;
    for line in input.trim_end_matches("\n").split("\n") {
        let first_digit = line
            .chars()
            .nth(line.find(|c: char| c.is_numeric()).unwrap())
            .unwrap() as u32
            - 48;
        let last_digit = line
            .chars()
            .nth(line.rfind(|c: char| c.is_numeric()).unwrap())
            .unwrap() as u32
            - 48;
        result += first_digit * 10 + last_digit;
    }
    result
}

fn part2(input: &str) -> u32 {
    let mut string_numbers = collections::HashMap::new();
    string_numbers.insert("one", 1);
    string_numbers.insert("two", 2);
    string_numbers.insert("three", 3);
    string_numbers.insert("four", 4);
    string_numbers.insert("five", 5);
    string_numbers.insert("six", 6);
    string_numbers.insert("seven", 7);
    string_numbers.insert("eight", 8);
    string_numbers.insert("nine", 9);
    string_numbers.insert("1", 1);
    string_numbers.insert("2", 2);
    string_numbers.insert("3", 3);
    string_numbers.insert("4", 4);
    string_numbers.insert("5", 5);
    string_numbers.insert("6", 6);
    string_numbers.insert("7", 7);
    string_numbers.insert("8", 8);
    string_numbers.insert("9", 9);
    let mut result: u32 = 0;
    for line in input.trim_end_matches("\n").split("\n") {
        let mut all_matches: Vec<_> = Vec::new();
        for (key, value) in string_numbers.iter() {
            let matches: Vec<_> = line.match_indices(key).map(|(i, _)| (i, value)).collect();
            all_matches.extend(matches);
        }
        let first_digit = all_matches
            .iter()
            .enumerate()
            .min_by_key(|&(_, (index, value))| index)
            .unwrap()
            .1
             .1;
        let last_digit = all_matches
            .iter()
            .enumerate()
            .max_by_key(|&(_, (index, value))| index)
            .unwrap()
            .1
             .1;
        result += *first_digit as u32 * 10 + *last_digit as u32;
    }
    result
}

fn main() {
    let contents =
        fs::read_to_string("./input.txt").expect("Should have been able to read the file");
    println!("{}", part1(&contents));
    println!("{}", part2(&contents));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        let test_input = "1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet";
        assert_eq!(part1(&test_input), 142);
    }

    #[test]
    fn test_part2() {
        let test_input = "two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen";
        assert_eq!(part2(&test_input), 281);
    }
}
