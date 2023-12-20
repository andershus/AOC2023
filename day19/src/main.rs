use std::collections::HashMap;

fn parse_rules(rules_str: &str) -> HashMap<&str, Vec<&str>> {
    rules_str
        .lines()
        .map(|line| {
            let (id, rules) = line.trim_end_matches("}").split_once("{").unwrap();
            (id, rules.split(",").collect::<Vec<_>>())
        })
        .collect::<HashMap<&str, Vec<&str>>>()
}

fn parse_parts(parts_str: &str) -> Vec<HashMap<char, usize>> {
    let mut result = Vec::new();
    let parsed = parts_str
        .lines()
        .map(|line| {
            line.trim_start_matches("{")
                .trim_end_matches("}")
                .split(",")
                .map(|property| property.split_once("=").unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    for pairs in parsed {
        result.push(HashMap::from_iter(pairs.iter().map(|&(id, value)| {
            (id.chars().next().unwrap(), value.parse::<usize>().unwrap())
        })));
    }
    result
}

fn collect_input(input: &str) -> (HashMap<&str, Vec<&str>>, Vec<HashMap<char, usize>>) {
    let (rules_str, parts_str) = input.split_once("\n\n").unwrap();
    (parse_rules(rules_str), parse_parts(parts_str))
}

fn part1(input: &str) -> usize {
    let (rules, parts) = collect_input(input);
    let mut accepted: Vec<HashMap<char, usize>> = Vec::new();
    dbg!(&rules);
    for part in &parts {
        dbg!(&part);
        let mut step = rules.get(&"in").unwrap();
        loop {
            let mut next_step = step;
            for rule in step.iter() {
                dbg!(&rule);
                match rule.split_once(":") {
                    Some((condition, potential_step)) => {
                        let mut cond_iter = condition.chars();
                        let (property, aligator, bound) = (
                            cond_iter.next().unwrap(),
                            cond_iter.next().unwrap(),
                            cond_iter.as_str(),
                        );
                        if match aligator {
                            '<' => {
                                if part.get(&property).unwrap() < &bound.parse::<usize>().unwrap() {
                                    true
                                } else {
                                    false
                                }
                            }
                            '>' => {
                                if part.get(&property).unwrap() > &bound.parse::<usize>().unwrap() {
                                    true
                                } else {
                                    false
                                }
                            }
                            _ => panic!("Invalid input"),
                        } {
                            match potential_step {
                                "R" => {
                                    println!("{:?} REJECTED BY {:?}", &part, rule);
                                    break;
                                }
                                "A" => {
                                    accepted.push(part.clone());
                                    println!("{:?} ACCEPTED", part);
                                    break;
                                }
                                _ => {
                                    println!("following {:?}", potential_step);
                                    next_step = &rules.get(potential_step).unwrap();
                                    break;
                                }
                            }
                        }
                    }
                    None => {
                        println!("end of ruleset {:?}", step);
                        match rule {
                            &"R" => {
                                println!("{:?} REJECTED BY {:?}", &part, rule);
                            }
                            &"A" => {
                                accepted.push(part.clone());
                                println!("{:?} ACCEPTED", part);
                            }
                            _ => {
                                println!("following {:?}", rule);
                                next_step = &rules.get(rule).unwrap();
                            }
                        }
                    }
                }
            }
            if next_step == step {
                break;
            } else {
                step = next_step;
            }
        }
    }
    accepted
        .iter()
        .map(|part| part.values().sum::<usize>())
        .sum()
}

fn new_bounds(
    bounds: &HashMap<char, Vec<usize>>,
    property: char,
    index: usize,
    value: usize,
) -> (HashMap<char, Vec<usize>>, HashMap<char, Vec<usize>>) {
    dbg!(&bounds);
    let mut new_bounds = bounds.clone();
    let mut leftover_bounds = bounds.clone();
    let bound = &bounds.get(&property).unwrap();
    if index == 0 {
        new_bounds.insert(property, vec![bound[index].max(value) + 1, bound[1]]);
        leftover_bounds.insert(property, vec![bound[0], bound[index].max(value)]);
    } else if index == 1 {
        new_bounds.insert(property, vec![bound[0], bound[index].min(value) - 1]);
        leftover_bounds.insert(property, vec![bound[index].min(value), bound[1]]);
    }
    dbg!(&new_bounds, &leftover_bounds);
    (new_bounds, leftover_bounds)
}

fn process_ruleset(
    rules: &HashMap<&str, Vec<&str>>,
    current_rule: &str,
    bounds: &mut HashMap<char, Vec<usize>>,
) -> Vec<HashMap<char, Vec<usize>>> {
    let mut result = Vec::new();
    dbg!(&current_rule);
    for rule in rules.get(current_rule).unwrap().iter() {
        dbg!(&rule, &bounds);
        match rule.split_once(":") {
            Some((condition, next_rule)) => {
                let mut cond_iter = condition.chars();
                let (property, aligator, bound) = (
                    cond_iter.next().unwrap(),
                    cond_iter.next().unwrap(),
                    cond_iter.as_str(),
                );
                let mut new_boundings;
                match aligator {
                    '<' => {
                        new_boundings = new_bounds(&bounds, property, 1, bound.parse::<usize>().unwrap());
                        }
                    '>' => {
                        new_boundings = new_bounds(&bounds, property, 0, bound.parse::<usize>().unwrap());
                    }
                    _ => panic!("Invalid input"),
                }
                match next_rule {
                    "R" => {
                        println!("REJECTED BY {:?}", rule);
                    }
                    "A" => {
                        println!("ACCEPTED EARLY {:?} by {:?}", bounds, rule);
                        result.push(new_boundings.1);
                    }
                    _ => {
                        println!("following {:?}", next_rule);
                        result.extend(process_ruleset(rules, next_rule, &mut new_boundings.0));
                    }
                }
            }
            None => {
                println!("end of ruleset");
                match rule {
                    &"R" => {
                        println!("REJECTED BY {:?}", current_rule);
                    }
                    &"A" => {
                        println!("ACCEPTED {:?} by {:?}", bounds, current_rule);
                        result.push(bounds.clone());
                    }
                    _ => {
                        println!("following {:?}", rule);
                        result.extend(process_ruleset(rules, rule, bounds));
                    }
                }
            }
        }
    }
    result
}

fn part2(input: &str) -> usize {
    let (rules, _) = collect_input(input);
    let mut bounds = HashMap::from([
        ('x', vec![1, 4000]),
        ('m', vec![1, 4000]),
        ('a', vec![1, 4000]),
        ('s', vec![1, 4000]),
    ]);
    let mut accepted: Vec<HashMap<char, Vec<usize>>> = Vec::new();
    accepted.extend(process_ruleset(&rules, "in", &mut bounds));
    dbg!(&accepted);
    accepted.iter().map(|slice| slice.values().map(|v| v[1] - v[0] + 1).product::<usize>()).sum()
}

fn main() {
    let binding =
        std::fs::read_to_string("./input.txt").expect("Should have been able to read the file");
    // remove trailing newline that should exist in every file
    let contents = binding.trim_end_matches("\n");
    println!("result part1: {:?}", part1(&contents));
    println!("result part2: {}", part2(&contents));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";

    // #[test]
    // fn test_part1() {
    //     assert_eq!(part1(&TEST_INPUT), 19114);
    // }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&TEST_INPUT), 167409079868000);
    }
}
