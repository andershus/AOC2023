// use std::collections::BTreeMap;

fn part1(input: &str) -> u32 {
    let chars: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    println!("{:?}", chars);
    0
}

fn part2(input: &str) -> i32 {
    let mut result = 0;
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

    const TEST_INPUT: &str = ".....
.S-7.
.|.|.
.L-J.
.....";

    const TEST_INPUT_2: &str = "-L|F7
7S-7|
L|7||
-L-J|
L|-JF";

    const TEST_INPUT3: &str = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";

    const TEST_INPUT_4: &str = "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";


    #[test]
    fn test_part1() {
        assert_eq!(part1(&TEST_INPUT), 4);
    }

    // #[test]
    // fn test_part12() {
    //     assert_eq!(part1(&TEST_INPUT), 4);
    // }

    // #[test]
    // fn test_part13() {
    //     assert_eq!(part1(&TEST_INPUT), 8);
    // }

    // #[test]
    // fn test_part14() {
    //     assert_eq!(part1(&TEST_INPUT), 8);
    // }

    // #[test]
    // fn test_part2() {
    //     assert_eq!(part2(TEST_INPUT), 2);
    // }
}
