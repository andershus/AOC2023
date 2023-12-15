use std::collections::HashMap;

fn collect_input(input: &str) -> Vec<&str> {
    input.split(",").collect::<Vec<&str>>()
}

fn compute_hash(input: &str) -> usize {
    input.chars().fold(0, |mut acc, c| {
        acc += c as usize;
        (acc * 17) % 256
    })
}

fn part1(input: &str) -> usize {
    collect_input(input)
        .iter()
        .map(|part| compute_hash(&part))
        .sum()
}

fn collect_input2(input: &str) -> Vec<(String, char, usize)> {
    input
        .split(",")
        .map(|part| {
            part.chars().fold(("".to_string(), '0', 0), |mut acc, c| {
                match c {
                    '=' | '-' => acc.1 = c,
                    _ if c.is_digit(10) => acc.2 = c.to_digit(10).unwrap() as usize,
                    _ if c.is_alphabetic() => acc.0 += &c.to_string(),
                    _ => panic!("Invalid input"),
                };
                acc
            })
        })
        .collect::<Vec<_>>()
}

fn calculate_focusing_power(
    boxes_map: Vec<HashMap<String, usize>>,
    boxes_ordered: Vec<Vec<String>>,
) -> usize {
    boxes_map
        .iter()
        .zip(boxes_ordered.iter())
        .enumerate()
        .map(|(i, (map, vec))| {
            vec.iter()
                .enumerate()
                .map(|(j, label)| (i + 1) * (j + 1) * map.get(label).unwrap())
                .sum::<usize>()
        })
        .sum::<usize>()
}

fn part2(input: &str) -> usize {
    let mut boxes_map: Vec<HashMap<String, usize>> = vec![HashMap::new(); 256];
    let mut boxes_ordered: Vec<Vec<String>> = vec![Vec::new(); 256];
    for vec in collect_input2(&input) {
        let box_num = compute_hash(&vec.0);
        match vec.1 {
            '=' => {
                if !boxes_map[box_num].contains_key(&vec.0) {
                    boxes_ordered[box_num].push(vec.0.clone());
                }
                boxes_map[box_num].insert(vec.0, vec.2)
            }
            '-' => {
                let mut new_boxes_ordered = boxes_ordered.clone();
                let pos = boxes_ordered[box_num].iter().position(|x| x == &vec.0);
                if pos.is_some() {
                    new_boxes_ordered[box_num].remove(pos.unwrap());
                };
                boxes_ordered = new_boxes_ordered;
                boxes_map[box_num].remove(&vec.0)
            }
            _ => panic!("Invalid input"),
        };
    }
    calculate_focusing_power(boxes_map, boxes_ordered)
}

fn main() {
    let binding =
        std::fs::read_to_string("./input.txt").expect("Should have been able to read the file");
    // remove trailing newline that should exist in every file
    let contents = binding.trim_end_matches("\n");
    println!("result part1: {}", part1(&contents));
    println!("result part2: {}", part2(&contents));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn test_hash() {
        let input = "HASH";
        assert_eq!(compute_hash(input), 52);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&TEST_INPUT), 1320);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&TEST_INPUT), 145);
    }
}
