use std::collections::BTreeMap;
use std::iter::zip;

const STRENGHTS: [char; 13] = [
    'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
];
const STRENGHTS_2: [char; 13] = [
    'A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J',
];

fn part1(input: &str) -> u32 {
    let lines = input
        .split("\n")
        .map(|line| {
            line.split_once(" ")
                .expect("should be two values on each line")
        })
        .collect::<Vec<(&str, &str)>>();
    let mut hands: Vec<&str> = Vec::new();
    let mut bets = BTreeMap::new();
    for line in lines {
        hands.push(line.0);
        bets.insert(line.0, line.1.parse::<u32>().expect("should be a number"));
    }

    // categorize the hands
    let mut categorized_rounds: BTreeMap<u32, Vec<&str>> = BTreeMap::from([
        (1, Vec::new()),
        (2, Vec::new()),
        (3, Vec::new()),
        (4, Vec::new()),
        (5, Vec::new()),
        (6, Vec::new()),
        (7, Vec::new()),
    ]);
    for hand in &hands {
        let mut cat_hand = BTreeMap::new();
        for card in hand.chars() {
            *cat_hand.entry(card).or_insert(0) += 1;
        }
        let mut sorted_cat = Vec::new();
        for (_, count) in cat_hand {
            sorted_cat.push(count);
        }
        sorted_cat.sort_by(|a, b| b.cmp(a));
        // println!("hand {}, sorted cat {:?}", hand, sorted_cat);
        // this can be done with one single sort
        if sorted_cat == [5] {
            // 5 of a kind
            categorized_rounds.get_mut(&1).unwrap().push(hand);
        } else if sorted_cat == [4, 1] {
            // 4 of a kind
            categorized_rounds.get_mut(&2).unwrap().push(hand);
        } else if sorted_cat == [3, 2] {
            // full house
            categorized_rounds.get_mut(&3).unwrap().push(hand);
        } else if sorted_cat == [3, 1, 1] {
            // 3 of a kind
            categorized_rounds.get_mut(&4).unwrap().push(hand);
        } else if sorted_cat == [2, 2, 1] {
            // 2 pair
            categorized_rounds.get_mut(&5).unwrap().push(hand);
        } else if sorted_cat == [2, 1, 1, 1] {
            // pair
            categorized_rounds.get_mut(&6).unwrap().push(hand);
        } else {
            // high card
            categorized_rounds.get_mut(&7).unwrap().push(hand);
        }
    }

    let mut sorted_hands: Vec<&str> = Vec::new();
    for types in categorized_rounds.values() {
        if types.len() > 0 {
            let mut sorted_types = types.clone();
            sorted_types.sort_by(|a, b| {
                for (a_c, b_c) in zip(a.chars(), b.chars()) {
                    if a_c == b_c {
                        continue;
                    }
                    let a_pos = STRENGHTS.iter().position(|&r| r == a_c).unwrap();
                    let b_pos = STRENGHTS.iter().position(|&r| r == b_c).unwrap();
                    return a_pos.cmp(&b_pos);
                }
                // should never happen as the hands are not empty
                a.cmp(&b)
            });
            sorted_hands.extend(sorted_types);
        }
    }

    let mut result = 0;
    let num_rounds = sorted_hands.len();
    for (i, hand) in sorted_hands.iter().enumerate() {
        result += (num_rounds - i) as u32 * *bets.get(hand).expect("should have a bet");
    }
    result
}

fn part2(input: &str) -> u32 {
    let lines = input
        .split("\n")
        .map(|line| {
            line.split_once(" ")
                .expect("should be two values on each line")
        })
        .collect::<Vec<(&str, &str)>>();
    let mut hands: Vec<&str> = Vec::new();
    let mut bets = BTreeMap::new();
    for line in lines {
        hands.push(line.0);
        bets.insert(line.0, line.1.parse::<u32>().expect("should be a number"));
    }

    // categorize the hands
    let mut categorized_rounds: BTreeMap<u32, Vec<&str>> = BTreeMap::from([
        (1, Vec::new()),
        (2, Vec::new()),
        (3, Vec::new()),
        (4, Vec::new()),
        (5, Vec::new()),
        (6, Vec::new()),
        (7, Vec::new()),
    ]);
    for hand in &hands {
        let mut cat_hand = BTreeMap::new();
        for card in hand.chars() {
            *cat_hand.entry(card).or_insert(0) += 1;
        }
        let mut sorted_cat = Vec::new();
        let mut jack_count = 0;
        for (card, count) in cat_hand {
            println!("card {}, count {}", card, count);
            if card == 'J' {
                jack_count = count;
                continue;
            }
            sorted_cat.push(count);
        }
        sorted_cat.sort_by(|a, b| b.cmp(a));
        println!(
            "hand {}, sorted cat {:?}, jacks {:?}",
            hand, sorted_cat, jack_count
        );
        if sorted_cat.len() == 0 {
            // only jacks
            categorized_rounds.get_mut(&1).unwrap().push(hand);
            continue;
        }
        sorted_cat[0] += jack_count;
        // this can be done with one single sort
        if sorted_cat == [5] {
            // 5 of a kind
            categorized_rounds.get_mut(&1).unwrap().push(hand);
        } else if sorted_cat == [4, 1] {
            // 4 of a kind
            categorized_rounds.get_mut(&2).unwrap().push(hand);
        } else if sorted_cat == [3, 2] {
            // full house
            categorized_rounds.get_mut(&3).unwrap().push(hand);
        } else if sorted_cat == [3, 1, 1] {
            // 3 of a kind
            categorized_rounds.get_mut(&4).unwrap().push(hand);
        } else if sorted_cat == [2, 2, 1] {
            // 2 pair
            categorized_rounds.get_mut(&5).unwrap().push(hand);
        } else if sorted_cat == [2, 1, 1, 1] {
            // pair
            categorized_rounds.get_mut(&6).unwrap().push(hand);
        } else {
            // high card
            categorized_rounds.get_mut(&7).unwrap().push(hand);
        }
    }

    let mut sorted_hands: Vec<&str> = Vec::new();
    for types in categorized_rounds.values() {
        if types.len() > 0 {
            let mut sorted_types = types.clone();
            sorted_types.sort_by(|a, b| {
                for (a_c, b_c) in zip(a.chars(), b.chars()) {
                    if a_c == b_c {
                        continue;
                    }
                    let a_pos = STRENGHTS_2.iter().position(|&r| r == a_c).unwrap();
                    let b_pos = STRENGHTS_2.iter().position(|&r| r == b_c).unwrap();
                    return a_pos.cmp(&b_pos);
                }
                // should never happen as the hands are not empty
                a.cmp(&b)
            });
            sorted_hands.extend(sorted_types);
        }
    }

    let mut result = 0;
    let num_rounds = sorted_hands.len();
    for (i, hand) in sorted_hands.iter().enumerate() {
        // println!("calculating: {} * {} from hand {}", num_rounds - i, *bets.get(hand).expect("should have a bet"), hand);
        result += (num_rounds - i) as u32 * *bets.get(hand).expect("should have a bet");
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
    const TEST_INPUT: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&TEST_INPUT), 6440);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 5905);
    }
}
