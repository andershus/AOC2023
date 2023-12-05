use std::collections::BTreeMap;

fn part1_too_slow(input: &str) -> u32 {
    let mut mappings: Vec<BTreeMap<u32, u32>> = Vec::new();

    let mut lines = input.lines();
    let mut seeds: Vec<u32> = lines
        .next()
        .expect("non-empty input")
        .split_once(": ")
        .expect("should have :")
        .1
        .split_whitespace()
        .map(|x| x.parse::<u32>().expect("should be a number"))
        .collect();
    lines.next().expect("should be empty line");

    let binding = lines.collect::<Vec<&str>>().join("\n");
    for map_description in binding.split("\n\n") {
        let mut map: BTreeMap<u32, u32> = BTreeMap::new();
        for map_line in map_description
            .split_once("\n")
            .expect("should have header")
            .1
            .split("\n")
        {
            let map_info = map_line
                .split_whitespace()
                .map(|x| x.parse::<u32>().expect("should be a number"))
                .collect::<Vec<u32>>();
            for i in 0..map_info[2] {
                map.insert(map_info[1] + i, map_info[0] + i);
            }
        }
        mappings.push(map);
    }
    for map in &mappings {
        for seed in &mut seeds {
            match map.get(seed) {
                Some(value) => *seed = *value,
                None => (),
            }
        }
    }
    *seeds.iter().min().expect("should have a min")
}

fn part1(input: &str) -> usize {
    let mut lines = input.lines();
    let mut seeds: Vec<usize> = lines
        .next()
        .expect("non-empty input")
        .split_once(": ")
        .expect("should have :")
        .1
        .split_whitespace()
        .map(|x| x.parse::<usize>().expect("should be a number"))
        .collect();
    lines.next().expect("should be empty line");

    let binding = lines.collect::<Vec<&str>>().join("\n");
    for map_description in binding.split("\n\n") {
        for seed in &mut seeds {
            let mut movable = true;
            for map_line in map_description
                .split_once("\n")
                .expect("should have header")
                .1
                .split("\n")
            {
                let map_info = map_line
                    .split_whitespace()
                    .map(|x| x.parse::<usize>().expect("should be a number"))
                    .collect::<Vec<usize>>();
                if movable && (*seed >= map_info[1] && *seed < map_info[1] + map_info[2]) {
                    *seed = map_info[0] + (*seed - map_info[1]);
                    movable = false;
                }
            }
        }
    }
    *seeds.iter().min().expect("should have a min")
}

fn part2_too_slow(input: &str) -> usize {
    let mut seeds: Vec<usize> = Vec::new();
    let mut lines = input.lines();
    let seed_line: Vec<usize> = lines
        .next()
        .expect("non-empty input")
        .split_once(": ")
        .expect("should have :")
        .1
        .split_whitespace()
        .map(|x| x.parse::<usize>().expect("should be a number"))
        .collect();
    for (seed_start, len) in seed_line.chunks(2).map(|x| (x[0], x[1])) {
        seeds.extend(seed_start..seed_start + len);
    }
    lines.next().expect("should be empty line");

    let binding = lines.collect::<Vec<&str>>().join("\n");
    for map_description in binding.split("\n\n") {
        for seed in &mut seeds {
            let mut movable = true;
            for map_line in map_description
                .split_once("\n")
                .expect("should have header")
                .1
                .split("\n")
            {
                let map_info = map_line
                    .split_whitespace()
                    .map(|x| x.parse::<usize>().expect("should be a number"))
                    .collect::<Vec<usize>>();
                if movable && (*seed >= map_info[1] && *seed < map_info[1] + map_info[2]) {
                    *seed = map_info[0] + (*seed - map_info[1]);
                    movable = false;
                }
            }
        }
    }
    *seeds.iter().min().expect("should have a min")
}

fn part2(input: &str) -> usize {
    let mut seeds = Vec::new();
    let mut lines = input.lines();
    let seed_line: Vec<usize> = lines
        .next()
        .expect("non-empty input")
        .split_once(": ")
        .expect("should have :")
        .1
        .split_whitespace()
        .map(|x| x.parse::<usize>().expect("should be a number"))
        .collect();
    for (seed_start, len) in seed_line.chunks(2).map(|x| (x[0], x[1])) {
        seeds.push((seed_start, seed_start + len - 1));
    }
    seeds.sort_by_key(|&(a, _)| a);
    lines.next().expect("should be empty line");

    let binding = lines.collect::<Vec<&str>>().join("\n");
    for map_description in binding.split("\n\n") {
        let mut move_ranges = Vec::new();
        let (map_header, map_lines) = map_description
            .split_once("\n")
            .expect("should have header");
        for map_line in map_lines.split("\n") {
            let map_info = map_line
                .split_whitespace()
                .map(|x| x.parse::<usize>().expect("should be a number"))
                .collect::<Vec<usize>>();
            move_ranges.push(((map_info[1], map_info[1] + map_info[2] - 1), map_info[0]));
            move_ranges.sort_by_key(|&((_, _), start_map)| start_map);
        }

        let mut new_ranges = Vec::new();
        for seed in &seeds {
            let mut unmoved = true;
            let mut missed = Vec::new();
            for move_range in &move_ranges {
                let overlap = (move_range.0 .0.max(seed.0), move_range.0 .1.min(seed.1));
                if overlap.0 <= overlap.1 {
                    unmoved = false;
                    let new_range = (
                        move_range.1 + overlap.0 - move_range.0 .0,
                        move_range.1 + overlap.1 - move_range.0 .0,
                    );
                    new_ranges.push(new_range);
                    if seed.0 < overlap.0 {
                        let remaining = (seed.0, overlap.0 - 1);
                        missed.push(remaining);
                    }
                    if overlap.1 < seed.1 {
                        let remaining = (overlap.1 + 1, seed.1);
                        missed.push(remaining);
                    }
                }
            }
            for seed in &missed {
                let mut dunmoved = true;
                for move_range in &move_ranges {
                    let overlap = (move_range.0 .0.max(seed.0), move_range.0 .1.min(seed.1));
                    if overlap.0 <= overlap.1 {
                        dunmoved = false;
                    }
                }
                if dunmoved {
                    new_ranges.push(*seed);
                }
            }
            if unmoved {
                new_ranges.push(*seed);
            }
        }
        seeds = new_ranges;
        seeds.sort_by_key(|&(a, _)| a);
    }
    seeds.first().expect("non-empty").0
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
    const TEST_INPUT: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn test_part1_slow() {
        assert_eq!(part1_too_slow(TEST_INPUT), 35);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 35);
    }

    #[test]
    fn test_part2_slow() {
        assert_eq!(part2_too_slow(TEST_INPUT), 46);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 46);
    }
}
