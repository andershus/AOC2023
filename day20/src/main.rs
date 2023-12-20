use std::collections::{HashMap, VecDeque};

fn collect_input(input: &str) -> (HashMap<&str, (char, Vec<&str>, usize)>, Vec<Vec<usize>>) {
    let mut mapping = HashMap::new();
    let module_count = input.lines().count() - 1;
    let mut initial_state = vec![vec![0; module_count]; module_count];
    // for flip-flops we use all 0s to denote off and all 1s to denote on
    // for conjunction we use 0 to denote last signal was high and 1 to denote last signal was low
    // we initialize conjunctions to have recieved a high signal from all inputs it is missing
    for (i, line) in input.lines().enumerate() {
        let (module_w_type, output_str) = line.split_once(" -> ").unwrap();
        let (module_type, module) = module_w_type
            .chars()
            .next()
            .map(|c| (c, &module_w_type[c.len_utf8()..]))
            .unwrap();
        let outputs = output_str.split(", ").collect::<Vec<_>>();
        mapping.insert(module, (module_type, outputs, if i > 0 { i - 1 } else { 0 }));
    }
    for (i, line) in input.lines().enumerate() {
        let (module_w_type, output_str) = line.split_once(" -> ").unwrap();
        let (_, module) = module_w_type
            .chars()
            .next()
            .map(|c| (c, &module_w_type[c.len_utf8()..]))
            .unwrap();
        for output in output_str.split(", ").collect::<Vec<_>>() {
            match mapping.get(&output) {
                Some((module_type, _, index)) => {
                    if module_type == &'&' {
                        // for conjunction we use 0 to denote last signal was low and 1 to denote last signal was high
                        initial_state[*index][i-1] = 1;
                    }
                }
                // not every module sends a signal to another module
                None => (),
            }
        }
    }
    (mapping, initial_state)
}

fn part1(input: &str) -> usize {
    let (mapping, initial_state) = collect_input(input);
    let module_count = input.lines().count() - 1;
    let off = vec![0; module_count];
    let on = vec![1; module_count];
    // dbg!((&mapping, &initial_state));
    let mut state_to_signals = vec![(initial_state.clone(), 0)];
    loop {
        let mut low_signals = 1;
        let mut high_signals = 0;
        let mut state = state_to_signals.last().unwrap().0.clone();
        let signals = &mut mapping.get(&"roadcaster").unwrap().1.iter().map(|&s| (s, 0, "roadcaster")).collect::<VecDeque<_>>();
        signals.push_back(("inv", 1, "c"));
        signals.push_back(("a", 0, "inv"));
        signals.push_back(("b", 0, "a"));
        signals.push_back(("c", 0, "b"));
        signals.push_back(("inv", 0, "c"));
        while !signals.is_empty() {
            let (destination, strength, source) = signals.pop_front().unwrap();
            println!("processing {:?}", (destination, strength, source));
            if strength == 0 {
                low_signals += 1
            } else {
                high_signals += 1
            }
            match mapping.get(destination) {
                Some((module_type, outputs, index)) => {
                match module_type {
                    // flip-flop
                    '%' => {
                        println!("{} is flip-flop in state {:?}", destination, state[*index]);
                        if strength == 0 {
                            if state[*index] == off {
                                state[*index] = on.clone();
                                // for output in outputs {
                                //     signals.push_back((output, 1, destination));
                                // }
                            } else {
                                state[*index] = off.clone();
                                // for output in outputs {
                                //     signals.push_back((output, 1, destination));
                                // }
                            }
                        }
                    },
                    // conjunction
                    '&' => {
                        state[*index][mapping.get(&source).unwrap().2] = 1 - strength;
                        if state[*index] == off {
                            // for output in outputs {
                            //     signals.push_back((output, 0, destination));
                            // }
                        } else {
                            // for output in outputs {
                            //     signals.push_back((output, 1, destination));
                            // }
                        }
                    },
                    _ => panic!("Invalid input"),
                    }
                }
                None => (),
            }
            println!("next signals {:?}", signals);
            println!("state {:?}\n", state);
        }
        if &state == &initial_state {
            println!("found solution");
            break;
        } else {
            println!("pushing state {:?} having processed {:?} + {:?} signals", state, low_signals, high_signals);
            state_to_signals.push((state, low_signals + high_signals));
            low_signals = 1;
            high_signals = 0;
        }
        break;
    }
    0
}

fn part2(input: &str) -> usize {
    input.len()
}

fn main() {
    let binding =
        std::fs::read_to_string("./input.txt").expect("Should have been able to read the file");
    // remove trailing newline that should exist in every file
    let contents = binding.trim_end_matches("\n");
    println!("result part1: {:?}", part1(&contents));
    println!("result part2: {:?}", part2(&contents));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";

    const TEST_INPUT2: &str = "broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&TEST_INPUT), 32000000);
    }

    // #[test]
    // fn test_part12() {
    //     assert_eq!(part1(&TEST_INPUT2), 11687500);
    // }

    // #[test]
    // fn test_part2() {
    //     assert_eq!(part2(&TEST_INPUT), 167409079868000);
    // }
}
