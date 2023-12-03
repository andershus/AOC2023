use std::collections::HashMap;

fn part1(input: &str) -> u32 {
    let mut result = 0;
    let conf = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);
    for line in input.lines() {
        let (prefix, games) = &line.trim().split_once(": ").expect("line should contain :");
        let mut id = prefix
            .split_once(" ")
            .expect("game <id>")
            .1
            .parse::<u32>()
            .expect("id is number");
        for game in games.split("; ") {
            for draw in game.split(", ") {
                let (balls, colour) = draw.split_once(" ").expect("<num> <colour>");
                if balls.parse::<u8>().expect("num balls is number")
                    > *conf.get(&colour).expect("colour missing")
                {
                    id = 0;
                }
            }
        }
        result += id;
    }
    result
}

fn part2(input: &str) -> u32 {
    let mut result = 0;
    for line in input.lines() {
        let mut conf = HashMap::from([("red", 0), ("green", 0), ("blue", 0)]);
        let (_, games) = &line.trim().split_once(": ").expect("line should contain :");
        for game in games.split("; ") {
            for draw in game.split(", ") {
                let (balls_str, colour) = draw.split_once(" ").expect("<num> <colour>");
                let ball = balls_str.parse::<u32>().expect("num balls is number");
                if ball > *conf.get(&colour).expect("colour missing") {
                    conf.insert(&colour, ball);
                }
            }
        }
        result += conf.clone().into_values().product::<u32>();
    }
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
    #[test]
    fn test_part1() {
        let test_input = "
        Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
            .trim_start_matches("\n")
            .trim_end_matches("\n");
        assert_eq!(part1(&test_input), 8);
    }

    #[test]
    fn test_part2() {
        let test_input = "
        Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
            .trim_start_matches("\n");
        assert_eq!(part2(&test_input), 2286);
    }
}
