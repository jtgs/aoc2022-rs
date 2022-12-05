use std::{num::ParseIntError, str::FromStr};
use itertools::Itertools;

use crate::helpers::split_into_lines;

struct Step {
    qty: i32,
    from: usize,
    to: usize
}

impl std::str::FromStr for Step {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let x = s.replace("from", "to");

        let (q, f, t) = x
            .strip_prefix("move ")
            .and_then(|s| Some(s.split(" to ")))
            .and_then(|s| Some(s.collect_tuple()))
            .unwrap()
            .unwrap();

        let qty = q.trim().parse::<i32>()?;
        let from = f.trim().parse::<usize>()?;
        let to = t.trim().parse::<usize>()?;

        Ok(Step { qty, from, to })
    }
}

pub fn day05(input_lines: &str) -> (String, String) {
    let lines = split_into_lines(input_lines);
    let mut parts = lines.split(|l| l.is_empty()).take(2);
    let boxes: Vec<String> = parts.next().unwrap().iter().rev().map(|s| s.to_owned()).collect();
    let num_stacks: i32 = boxes[0].split_whitespace().last().unwrap().parse().unwrap();
    let mut stacks: Vec<Vec<char>> = vec![];
    for _ in 0..num_stacks {
        stacks.push(vec![]);
    }

    for i in 1..boxes.len() {
        let row_chars:Vec<char> = boxes[i].chars().collect();
        for j in 0..stacks.len() {
            let char = row_chars[(4 * j) + 1];
            if char != ' ' {
                stacks[j].push(char);
            }
        }
    }

    // Parse out the steps
    let steps_raw = parts.next().unwrap();
    let steps = steps_raw.iter().map(|s| Step::from_str(&*s).unwrap());

    // Finally we can solve the puzzle
    for (_, step) in steps.enumerate() {
        for _ in 0..step.qty {
            let item = stacks[step.from - 1].pop().unwrap();
            stacks[step.to - 1].push(item);
        }
    };

    let answer1 = stacks.iter().fold("".to_string(), |acc, x| format!("{}{}", acc, x.last().unwrap()));
    let answer2 = 0;
    (format!("{}", answer1), format!("{}", answer2))
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
    1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
";

    #[test]
    fn check_day05_part1_case1() {
        assert_eq!(day05(TEST_INPUT).0, "CMZ".to_string())
    }

    #[test]
    fn check_day05_part2_case1() {
        assert_eq!(day05(TEST_INPUT).1, "0".to_string())
    }

    #[test]
    fn check_day05_both_case1() {
        assert_eq!(day05(TEST_INPUT), ("0".to_string(), "0".to_string()))
    }
}
