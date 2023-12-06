use std::collections::BTreeMap;

fn part1(input: &str) -> u32 {
    let mut result = 0;
    for line in input.lines() {
        let (winning_numbers_str, our_numbers_str) = line
            .split_once(":")
            .expect("should have :")
            .1
            .split_once("|")
            .expect("should have |");
        let winning_numbers: Vec<u32> = winning_numbers_str
            .split_whitespace()
            .map(|x| x.parse::<u32>().expect("should be a number"))
            .collect();
        let mut game_result: u32 = 0;
        for number in our_numbers_str
            .split_whitespace()
            .map(|x| x.parse::<u32>().expect("should be a number"))
            .collect::<Vec<u32>>()
        {
            if winning_numbers.contains(&number) {
                if game_result == 0 {
                    game_result = 1;
                } else {
                    game_result *= 2;
                }
            }
        }
        result += game_result;
    }
    result
}

fn part2(input: &str) -> u32 {
    let mut result = 0;
    let num_games = input.lines().count() as u32;
    let mut games: BTreeMap<u32, u32> = BTreeMap::new();
    for game_num in 1..=num_games {
        games.insert(game_num, 1);
    }
    for (game_num, line) in input.lines().enumerate() {
        let (winning_numbers_str, our_numbers_str) = line
            .split_once(":")
            .expect("should have :")
            .1
            .split_once("|")
            .expect("should have |");
        let winning_numbers: Vec<u32> = winning_numbers_str
            .split_whitespace()
            .map(|x| x.parse::<u32>().expect("should be a number"))
            .collect();
        let current_game_tickets = games.pop_first().expect("should have a game").1;

        let mut game_result: u32 = 0;
        for number in our_numbers_str
            .split_whitespace()
            .map(|x| x.parse::<u32>().expect("should be a number"))
            .collect::<Vec<u32>>()
        {
            if winning_numbers.contains(&number) {
                game_result += 1;
                let tickets_won = games
                    .get_mut(&(game_num as u32 + 1 + &game_result))
                    .expect("should have a game");
                *tickets_won += current_game_tickets;
            }
        }
        result += current_game_tickets;
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
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
            .trim_start_matches("\n")
            .trim_end_matches("\n");
        assert_eq!(part1(&test_input), 13);
    }

    #[test]
    fn test_part2() {
        let test_input = "
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
            .trim_start_matches("\n");
        assert_eq!(part2(&test_input), 30);
    }
}
