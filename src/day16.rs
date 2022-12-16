use std::collections::{HashMap, VecDeque};

use lazy_static::lazy_static;
use regex::Regex;

#[derive(Clone)]
struct Valve {
    flow_rate: i32,
    neighbours: Vec<String>,
}

// entry = destination -> distance
type PathSet = HashMap<String, i32>;
type PathSetMap = HashMap<String, PathSet>;
// entry = name -> Valve struct
type ValveSet = HashMap<String, Valve>;

fn bfs(valves: &ValveSet, start: &str, end: &str) -> i32 {
    let mut steps = 0;
    let mut queue = VecDeque::<&str>::new();

    queue.push_back(start);

    while !queue.is_empty() {
        for _ in 0..queue.len() {
            let step = queue.pop_front().unwrap();

            if step == end {
                return steps;
            }

            for next in valves.get(step).unwrap().neighbours.iter() {
                queue.push_back(next);
            }
        }

        steps += 1;
    }

    unreachable!()
}

// inspired by https://gist.github.com/liampwll/351fb848f05e8efd257ac87c7d09d1b0
fn recurse(valves: &ValveSet, paths: &PathSetMap, opened: &Vec<String>, from: &str, total_released: i32, time_left: i32, best_total: i32) -> i32 {
    let valve = valves.get(from).unwrap();
    let mut new_best = best_total;

    if total_released > best_total {
        new_best = total_released;
    }

    if time_left <= 0 {
        return new_best
    }

    if !opened.contains(&from.to_owned()) {
        let mut new_opened = opened.clone();
        new_opened.push(from.to_owned());
        let new_total = total_released + (valve.flow_rate * time_left);
        new_best = recurse(valves, paths, &new_opened, from, new_total, time_left - 1, new_best);
    } else {
        for (next, cost) in paths.get(from).unwrap().iter().filter(|(k, _)| !opened.contains(k)) {
            new_best = recurse(valves, paths, opened, next, total_released, time_left - cost, new_best);
        }
    }

    return new_best;
}

pub fn day16(input_lines: &str) -> (String, String) {
    let valves: ValveSet = input_lines.lines().map(|s| {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"Valve (.*) has flow rate=(\d+); tunnel(?:s?) lead(?:s?) to valve(?:s?) (.*)").unwrap();
        }
        
        let caps = RE.captures(s).unwrap();
        let name = caps.get(1).unwrap().as_str().to_owned();
        let flow_rate = caps.get(2).unwrap().as_str().parse().unwrap();
        let neighbours = caps.get(3).unwrap().as_str().split(", ").map(|s| s.to_owned()).collect();

        (name, Valve { flow_rate, neighbours })
    }).collect();

    let mut keys: Vec<String> = valves.keys().filter(|k| valves.get(*k).unwrap().flow_rate != 0).map(|s| s.to_owned()).collect();
    keys.push("AA".to_string());

    let mut paths: HashMap<String, PathSet> = HashMap::new();
    for k in &keys {
        paths.insert(k.to_string(), PathSet::new());
        for k2 in &keys {
            if k2 != k {
                paths.get_mut(k).unwrap().insert(k2.to_string(), bfs(&valves, k, k2));
            }
        }
    }

    let answer1 = recurse(&valves, &paths, vec!["AA".to_owned()].as_ref(), "AA", 0, 29, 0);

    let answer2 = 0;
    (format!("{}", answer1), format!("{}", answer2))
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";

    #[test]
    fn check_day16_part1_case1() {
        assert_eq!(day16(TEST_INPUT).0, "1651".to_string())
    }

    #[test]
    fn check_day16_part2_case1() {
        assert_eq!(day16(TEST_INPUT).1, "0".to_string())
    }

    #[test]
    fn check_day16_both_case1() {
        assert_eq!(day16(TEST_INPUT), ("0".to_string(), "0".to_string()))
    }
}
