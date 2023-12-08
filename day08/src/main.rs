use std::collections::BTreeMap;
// use std::iter::zip;

fn part1(input: &str) -> u32 {
    // parse the input
    let (moves, map_str) = input.split_once("\n\n").expect("shoule be moves and map");
    let map = BTreeMap::from_iter(
        map_str
            .split("\n")
            .map(|line| line.split_once(" = ").expect("should be = on each line")),
    );

    // execute the moves
    let mut position = "AAA";
    let mut result = 0;
    for mv in moves.chars().cycle() {
        if position == "ZZZ" {
            return result;
        }
        let next_moves = map
            .get(position)
            .expect("all positions should be in map")
            .trim_start_matches("(")
            .trim_end_matches(")")
            .split_once(", ")
            .expect("shoule be , seperated");
        match mv {
            'L' => {
                position = next_moves.0;
            }
            'R' => {
                position = next_moves.1;
            }
            _ => panic!("should be L or R"),
        }
        result += 1;
    }
    panic!("should not reach here");
}

fn part2(input: &str) -> usize {
    // parse the input
    let (moves, map_str) = &input.split_once("\n\n").expect("shoule be moves and map");
    let map = BTreeMap::from_iter(
        map_str
            .split("\n")
            .map(|line| line.split_once(" = ").expect("should be = on each line")),
    );

    // execute the moves
    // can be brute forced or find lcm of the cycle lengths
    // we choose the latter for efficiency
    let mut positions = map
        .keys()
        .map(|k| *k)
        .filter(|k| k.chars().nth(2).expect("should be len of 3") == 'A')
        .collect::<Vec<&str>>();
    let mut result = 1;
    for position in &mut positions {
        let mut part_result = 0;
        for mv in moves.chars().cycle() {
            if position.chars().nth(2).expect("should be len of 3") == 'Z' {
                break;
            }
            let next_moves = map
                .get(position)
                .expect("all positions should be in map")
                .trim_start_matches("(")
                .trim_end_matches(")")
                .split_once(", ")
                .expect("shoule be , seperated");
            match mv {
                'L' => {
                    *position = next_moves.0;
                }
                'R' => {
                    *position = next_moves.1;
                }
                _ => panic!("should be L or R"),
            }
            part_result += 1;
        }
        result = num::integer::lcm(result, part_result);
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
    const TEST_INPUT: &str = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

    const TEST_INPUT2: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

    const TEST_INPUT_PART2: &str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&TEST_INPUT), 2);
    }

    #[test]
    fn test2_part1() {
        assert_eq!(part1(&TEST_INPUT2), 6);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT_PART2), 6);
    }
}
