// use std::collections::BTreeMap;

// fn part1_too_slow(input: &str) -> u32 {
//     let mut mappings: Vec<BTreeMap<u32, u32>> = Vec::new();

//     let mut lines = input.lines();
//     let mut seeds: Vec<u32> = lines
//         .next()
//         .expect("non-empty input")
//         .split_once(": ")
//         .expect("should have :")
//         .1
//         .split_whitespace()
//         .map(|x| x.parse::<u32>().expect("should be a number"))
//         .collect();
//     // println!("{:?}", seeds);
//     lines.next().expect("should be empty line");

//     let binding = lines.collect::<Vec<&str>>().join("\n");
//     for map_description in binding.split("\n\n") {
//         let mut map: BTreeMap<u32, u32> = BTreeMap::new();
//         for map_line in map_description
//             .split_once("\n")
//             .expect("should have header")
//             .1
//             .split("\n")
//         {
//             let map_info = map_line
//                 .split_whitespace()
//                 .map(|x| x.parse::<u32>().expect("should be a number"))
//                 .collect::<Vec<u32>>();
//             // println!("{:?}", map_info);
//             for i in 0..map_info[2] {
//                 map.insert(map_info[1] + i, map_info[0] + i);
//             }
//         }
//         mappings.push(map);
//     }
//     for map in &mappings {
//         for seed in &mut seeds {
//             match map.get(seed) {
//                 Some(value) => *seed = *value,
//                 None => (),
//             }
//         }
//         // println!("{:?}", seeds);
//     }
//     *seeds.iter().min().expect("should have a min")
// }

// fn part1(input: &str) -> usize {
//     let mut lines = input.lines();
//     let mut seeds: Vec<usize> = lines
//         .next()
//         .expect("non-empty input")
//         .split_once(": ")
//         .expect("should have :")
//         .1
//         .split_whitespace()
//         .map(|x| x.parse::<usize>().expect("should be a number"))
//         .collect();
//     // println!("{:?}", seeds);
//     lines.next().expect("should be empty line");

//     let binding = lines.collect::<Vec<&str>>().join("\n");
//     for map_description in binding.split("\n\n") {
//         // let mut new_locations = seeds.clone();
//         for seed in &mut seeds {
//             // println!("processing seed {:?}", seed);
//             let mut movable = true;
//             for map_line in map_description
//                 .split_once("\n")
//                 .expect("should have header")
//                 .1
//                 .split("\n")
//             {
//                 let map_info = map_line
//                     .split_whitespace()
//                     .map(|x| x.parse::<usize>().expect("should be a number"))
//                     .collect::<Vec<usize>>();
//                 // println!("{:?}", map_info);
//                 if movable && (*seed >= map_info[1] && *seed < map_info[1] + map_info[2]) {
//                     // println!(
//                     //     "seed {:?} is in range and will be moved to {}",
//                     //     seed,
//                     //     map_info[0] + (*seed - map_info[1])
//                     // );
//                     *seed = map_info[0] + (*seed - map_info[1]);
//                     movable = false;
//                 }
//             }
//         }
//         // println!("seeds {:?}", seeds);
//     }
//     *seeds.iter().min().expect("should have a min")
// }

// fn part2_too_slow(input: &str) -> usize {
//     let mut seeds: Vec<usize> = Vec::new();
//     let mut lines = input.lines();
//     let seed_line: Vec<usize> = lines
//         .next()
//         .expect("non-empty input")
//         .split_once(": ")
//         .expect("should have :")
//         .1
//         .split_whitespace()
//         .map(|x| x.parse::<usize>().expect("should be a number"))
//         .collect();
//     for (seed_start, len) in seed_line.chunks(2).map(|x| (x[0], x[1])) {
//         seeds.extend(seed_start..seed_start + len);
//     }
//     // println!("{:?}", seeds);
//     lines.next().expect("should be empty line");

//     let binding = lines.collect::<Vec<&str>>().join("\n");
//     for map_description in binding.split("\n\n") {
//         // let mut new_locations = seeds.clone();
//         for seed in &mut seeds {
//             // println!("processing seed {:?}", seed);
//             let mut movable = true;
//             for map_line in map_description
//                 .split_once("\n")
//                 .expect("should have header")
//                 .1
//                 .split("\n")
//             {
//                 let map_info = map_line
//                     .split_whitespace()
//                     .map(|x| x.parse::<usize>().expect("should be a number"))
//                     .collect::<Vec<usize>>();
//                 // println!("{:?}", map_info);
//                 if movable && (*seed >= map_info[1] && *seed < map_info[1] + map_info[2]) {
//                     // println!(
//                     //     "seed {:?} is in range and will be moved to {}",
//                     //     seed,
//                     //     map_info[0] + (*seed - map_info[1])
//                     // );
//                     *seed = map_info[0] + (*seed - map_info[1]);
//                     movable = false;
//                 }
//             }
//         }
//         // println!("seeds {:?}", seeds);
//     }
//     *seeds.iter().min().expect("should have a min")
// }

fn part2(input: &str) -> usize {
    let mut seeds: Vec<(usize, usize)> = Vec::new();
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
    // println!("{:?}", seeds);
    lines.next().expect("should be empty line");

    let binding = lines.collect::<Vec<&str>>().join("\n");
    for map_description in binding.split("\n\n") {
        // let mut new_locations = seeds.clone();
        let mut move_ranges = Vec::new();
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
            // println!("{:?}", map_info);
            move_ranges.push(((map_info[1], map_info[1] + map_info[2] - 1), map_info[0]));
        }
        // println!("move_ranges {:?}", move_ranges);
        let mut new_ranges = Vec::new();
        for seed in &mut seeds {
            println!("processing seeds {:?}", seed);
            for move_range in &move_ranges {
                println!("processing move_range {:?}", move_range);
                let overlap = (move_range.0.0.max(seed.0), move_range.0.1.min(seed.1));
                println!("overlap : {:?}", overlap);
                let mut intersections = [move_range.0.0, move_range.0.1, seed.0, seed.1];
                intersections.sort();
                println!("intersections : {:?}", intersections);
                if overlap.0 <= overlap.1 {
                    println!("seeds in {:?} will be moved", overlap);
                    new_ranges.push((move_range.1, move_range.1 + overlap.1 - overlap.0));
                }
                if move_range.0.0 <= seed.0 {
                    println!("seeds before {:?} will be moved", move_range.0.0);
                    new_ranges.push((move_range.0.0, seed.0 - 1));
                }
                if move_range.0.1 <= seed.1 {
                    println!("seeds after {:?} will be moved", move_range.0.1);
                    new_ranges.push((move_range.0.1, seed.1));
                }
                println!("new_ranges {:?}", new_ranges);
            }
        }
        println!("seeds {:?}", seeds);
        break;
    }
    *seeds
        .iter()
        .map(|(start, _)| start)
        .min()
        .expect("should have a min")
}

fn main() {
    let binding =
        std::fs::read_to_string("./input.txt").expect("Should have been able to read the file");
    let contents = binding.trim_start_matches("\n").trim_end_matches("\n");
    // println!("{}", part1(&contents));
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

    // #[test]
    // fn test_part1_slow() {
    //     assert_eq!(part1_too_slow(TEST_INPUT), 35);
    // }

    // #[test]
    // fn test_part1() {
    //     assert_eq!(part1(TEST_INPUT), 35);
    // }

    // #[test]
    // fn test_part2_slow() {
    //     assert_eq!(part2_too_slow(TEST_INPUT), 46);
    // }

    #[test]
    fn test_part2() {
        assert_eq!(part2("seeds: 79 14 55 13

seed-to-soil map:
50 60 11
60 50 11"
        ), 46);
    }
}
