fn collect_grid(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
}

fn transpose(grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    (0..grid[0].len())
        .map(|i| {
            grid.iter()
                .map(|inner| inner[i].clone())
                .collect::<Vec<_>>()
        })
        .collect()
}

fn roll_west(grid: &Vec<Vec<char>>, flip: bool) -> Vec<Vec<char>> {
    grid.iter()
        .map(|line| {
            line.into_iter()
                .collect::<String>()
                .split("#")
                .map(|section| {
                    let mut collect = section.chars().collect::<Vec<_>>();
                    if flip {
                        collect.sort();
                    } else {
                        collect.sort_by(|a, b| b.cmp(a));
                    }
                    collect.iter().collect::<String>()
                })
                .collect::<Vec<_>>()
                .join("#")
                .chars()
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn roll_north(grid: &Vec<Vec<char>>, flip: bool) -> Vec<Vec<char>> {
    transpose(&roll_west(&transpose(&grid), flip))
}

fn count_load(grid: &Vec<Vec<char>>) -> usize {
    grid.iter()
        .rev()
        .enumerate()
        .map(|(i, line)| {
            line.iter()
                .map(|c| match *c {
                    'O' => i + 1,
                    _ => 0,
                })
                .sum::<usize>()
        })
        .sum::<usize>()
}

fn part1(input: &str) -> usize {
    count_load(&roll_north(&collect_grid(&input), false))
}

// #[cached] // could not get caching working so we detect the cycle manually
fn cycle(grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
    roll_west(
        &roll_north(&roll_west(&roll_north(&grid, false), false), true),
        true,
    )
}

fn detect_cycle(vec: &Vec<usize>, max_size: usize) -> (usize, usize) {
    let mut cycle_size = max_size;
    for _ in 1..max_size {
        for i in 0..vec.len() - cycle_size as usize {
            let mut cycle = true;
            for j in 0..cycle_size {
                if vec[i + j] != vec[i + (cycle_size + j) as usize] {
                    cycle = false;
                    break;
                }
            }
            if cycle {
                return (i, cycle_size);
            }
        }
        cycle_size -= 1;
    }
    panic!("no cycle found");
}

fn part2(input: &str, cycle_detection_range: u32, max_cycle_len: usize) -> usize {
    let mut results = Vec::new();
    let mut cycles = Vec::new();
    let mut grid = collect_grid(&input);
    for _ in 0..cycle_detection_range {
        results.push(count_load(&grid));
        cycles.push(grid.clone());
        grid = cycle(grid);
    }
    let (index, cycle_size) = detect_cycle(&results, max_cycle_len);
    let cycle_pos = (1000000000 - index) % cycle_size;
    results[index + cycle_pos]
}

fn main() {
    let binding =
        std::fs::read_to_string("./input.txt").expect("Should have been able to read the file");
    let contents = binding.trim_start_matches("\n").trim_end_matches("\n");
    println!("result part1: {}", part1(&contents));
    println!("result part2: {}", part2(&contents, 250, 100));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

    const TEST_STEP2: &str = "OOOO.#.O..
OO..#....#
OO..O##..O
O..#.OO...
........#.
..#....#.#
..O..#.O.O
..O.......
#....###..
#....#....";

    #[test]
    fn test_part1_roll() {
        let input_grid = TEST_INPUT
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();
        let output_grid = TEST_STEP2
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();
        assert_eq!(roll_north(&input_grid, false), output_grid);
    }

    #[test]
    fn test_part1_load() {
        let output_grid = TEST_STEP2
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();
        assert_eq!(count_load(&output_grid), 136);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&TEST_INPUT), 136);
    }

    #[test]
    fn test_part2_cycle() {
        let grid = TEST_INPUT
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();
        let first_cycle = collect_grid(
            ".....#....
....#...O#
...OO##...
.OO#......
.....OOO#.
.O#...O#.#
....O#....
......OOOO
#...O###..
#..OO#....",
        );
        assert_eq!(cycle(grid), first_cycle);
    }

    #[test]
    fn test_part2_cycle2() {
        let grid = collect_grid(TEST_INPUT);
        let first_cycle = ".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#..OO###..
#.OOO#...O"
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();
        assert_eq!(cycle(cycle(grid)), first_cycle);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&TEST_INPUT, 20, 10), 64);
    }
}
