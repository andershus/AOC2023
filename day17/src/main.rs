use std::collections:: HashMap;

fn collect_grid(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn part1(input: &str) -> usize {
    let grid = collect_grid(input);
    let mut shortest_paths: Vec<Vec<HashMap<(isize, isize), usize>>> = vec![vec![HashMap::new(); grid[0].len()]; grid.len()];
    let mut options = Vec::new();
    for start in [((0, 1), (0isize, 1isize), 1), ((1, 0), (1isize, 0isize), 1)] {
        options.push(start);
        shortest_paths[start.0.1][start.0.0].insert(start.1, grid[start.0 .1][start.0 .0]);
    }
    // dbg!(shortest_paths);
    while !options.is_empty() {
        let (pos, dir, straight) = options.remove(0);
        for new_dir in [(dir.1, -dir.0), dir, (-dir.1, dir.0)] {
            let new_straight = if new_dir == dir { straight + 1 } else { 1 };
            let new_x = pos.0.checked_add_signed(new_dir.0);
            let new_y = pos.1.checked_add_signed(new_dir.1);
            if new_x.is_none() || new_y.is_none() {
                continue;
            }
            let new_pos = (new_x.unwrap(), new_y.unwrap());
            if (new_straight <= 3) && (new_pos.0 < grid[0].len()) && (new_pos.1 < grid.len()) {
                // dbg!(pos, &shortest_paths);
                let new_shortest_path = shortest_paths[pos.1][pos.0].get(&dir).expect("should have been visited") + grid[new_pos.1][new_pos.0];
                let current_shortest_paths = shortest_paths[new_pos.1][new_pos.0].entry(new_dir).or_insert(usize::MAX);
                if new_shortest_path < *current_shortest_paths {
                    // println!("coming from {:?} with a shortest path {:?}, we can now reach {:?} in the distance {:?}", pos, shortest_paths[pos.1][pos.0], new_pos, new_shortest_path);
                    options.push((new_pos, new_dir, new_straight));
                    shortest_paths[new_pos.1][new_pos.0].insert(new_dir, new_shortest_path);
                    // dbg!((new_pos, &shortest_paths[new_pos.1][new_pos.0]));
                } else {
                    println!(
                        "THIS DISTANCE FROM {:?}  TO {:?} IS NOT SHORTER",
                        pos, new_pos
                    );
                }
            }
        }
    }
    dbg!(&shortest_paths);
    *shortest_paths[grid[0].len() - 1][grid.len() - 1].values().min().unwrap()
}

fn part2(input: &str) -> usize {
    input.len()
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

    const TEST_INPUT: &str = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";

//     #[test]
//     fn test_part1_smaller() {
//         assert_eq!(
//             part1(&"
// 991111
// 111991
// 119911
// 119911
// 199991
// 111111".trim_start_matches("\n")),
//             14
//         );
//     }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&TEST_INPUT), 102);
    }

    // #[test]
    // fn test_part2() {
    //     assert_eq!(part2(&TEST_INPUT), 51);
    // }
}

// path (>4, >5, v6, >11, >15, >20, ^23)
