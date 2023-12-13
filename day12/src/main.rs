use itertools::Itertools;

fn part1(input: &str) -> usize {
    let mut result = 0;
    for (springs_str, groups_str) in input.lines().map(|line| line.split_once(" ").unwrap()) {
        let groups = groups_str.split(",").map(|c| c.parse::<u32>().unwrap()).collect::<Vec<u32>>();
        for solution in std::iter::repeat(['.', '#'].iter()).take(springs_str.chars().filter(|&c| c == '?').count()).multi_cartesian_product().collect_vec() {
            let mut springs = springs_str.to_string();
            for c in &solution {
                springs = springs.replacen("?", &c.to_string(), 1);
            }
            let spring_groups = springs.split(".").filter(|&g| !g.is_empty()).map(|g| g.len() as u32).collect::<Vec<_>>();
            if spring_groups == groups {
                result += 1;
            }
        }
    }
    result
}

fn part2(input: &str) -> usize {
    let mut result = 0;
    for (springs_str, groups_str) in input.lines().map(|line| line.split_once(" ").unwrap()) {
        // println!("processing: {} {}", springs_str, groups_str);
        let unfolded_springs_str = vec![springs_str; 5].join("?");
        let unfolded_groups_str = vec![groups_str; 5].join(",");
        println!("processing: {:?} {}", unfolded_springs_str, unfolded_groups_str);
        let groups = groups_str.split(",").map(|c| c.parse::<u32>().unwrap()).collect::<Vec<u32>>();
        // let remaining_broken = groups.iter().sum::<u32>() - springs_str.chars().filter(|&c| c == '#').count() as u32;
        // let remaining_working = springs_str.chars().filter(|&c| c == '?').count() as u32 - remaining_broken;
        // println!("remaining working {:?}, remaining broken {:?}", remaining_working, remaining_broken);
        // should be a combinatorial way to generate this
        // let binding =  ".".repeat(remaining_working as usize) + &"#".repeat(remaining_broken as usize);
        let items = ['.', '#'];
        let multi_prod = std::iter::repeat(items.iter()).take(unfolded_springs_str.chars().filter(|&c| c == '?').count()).multi_cartesian_product().collect_vec();
        // println!("{:?}", multi_prod);
        // let mut processed_solutions = Vec::new();
        for solution in multi_prod {
            // if processed_solutions.contains(&solution) {
            //     continue;
            // }
            // println!("{:?}, {:?}", solution, springs_str);
            let mut springs = springs_str.to_string();
            for c in &solution {
                springs = springs.replacen("?", &c.to_string(), 1);
            }
            // println!("{:?}, {:?}", solution, springs);
            let spring_groups = springs.split(".").filter(|&g| !g.is_empty()).map(|g| g.len() as u32).collect::<Vec<_>>();
            // println!("{:?}", spring_groups);
            if spring_groups == groups {
                // println!("found match {:?}", springs);
                result += 1;
            }
            // processed_solutions.push(solution);
        }
        // break;
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

    const TEST_INPUT: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

    // #[test]
    // fn test_part1() {
    //     assert_eq!(part1(&TEST_INPUT), 21);
    // }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&TEST_INPUT), 525152);
    }

}
