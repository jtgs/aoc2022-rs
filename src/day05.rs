use std::{num::ParseIntError, str::FromStr};
use itertools::Itertools;

use crate::helpers::split_into_lines;

struct Step {
    qty: usize,
    from: usize,
    to: usize
}

impl std::str::FromStr for Step {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // It's easier if there is only one delimiter
        let x = s.replace("from", "to");

        let (q, f, t) = x
            .strip_prefix("move ") // Remove "move" from the front
            .and_then(|s| Some(s.split(" to "))) // Split into just the numbers
            .and_then(|s| Some(s.collect_tuple()))
            .unwrap()
            .unwrap();

        let qty = q.trim().parse::<usize>()?;
        let from = f.trim().parse::<usize>()?;
        let to = t.trim().parse::<usize>()?;

        Ok(Step { qty, from, to })
    }
}

pub fn day05(input_lines: &str) -> (String, String) {
    let lines = split_into_lines(input_lines);
    // The two halves of the input are split by a blank line
    let mut parts = lines.split(|l| l.is_empty()).take(2);
    // First `part` is the diagram of boxes - it is most useful as a Vec<String>, order reversed (so legend is the first item)
    let boxes: Vec<String> = parts.next().unwrap().iter().rev().map(|s| s.to_owned()).collect();
    // The legend (first item in `boxes`) tells us how many stacks we need - then create them
    let num_stacks: i32 = boxes[0].split_whitespace().last().unwrap().parse().unwrap();
    let mut stacks: Vec<Vec<char>> = vec![];
    for _ in 0..num_stacks {
        stacks.push(vec![]);
    }

    // Parse the stacks - look at the (4j + 1)th character in each row for the box in stack `j`, if any
    // Our stacks are ordered such that the 'bottom' is the start of the Vec
    for i in 1..boxes.len() {
        let row_chars:Vec<char> = boxes[i].chars().collect();
        for j in 0..stacks.len() {
            let char = row_chars[(4 * j) + 1];
            if char != ' ' {
                stacks[j].push(char);
            }
        }
    }

    // Parse out the steps - use the FromStr implementation above
    let steps_raw = parts.next().unwrap();
    let steps = steps_raw.iter().map(|s| Step::from_str(&*s).unwrap());

    // Finally we can solve the puzzle
    let mut stacks1 = stacks.clone();
    let mut stacks2 = stacks.clone();

    for (_, step) in steps.enumerate() {
        // Part 1: take boxes off one at a time and push them onto the `to` stack
        for _ in 0..step.qty {
            let item = stacks1[step.from - 1].pop().unwrap();
            stacks1[step.to - 1].push(item);
        }

        // Part 2: take a slice off the `from` stack and append it to the `to` stack
        let stacks2_copy = stacks2.clone();
        let (remainder, slice) = stacks2_copy[step.from - 1].split_at(stacks2_copy[step.from - 1].len() - step.qty);
        stacks2[step.from - 1] = remainder.to_vec();
        stacks2[step.to - 1].extend_from_slice(slice);
    };
    let answer1 = stacks1.iter().fold("".to_string(), |acc, x| format!("{}{}", acc, x.last().unwrap()));
    let answer2 = stacks2.iter().fold("".to_string(), |acc, x| format!("{}{}", acc, x.last().unwrap()));
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
        assert_eq!(day05(TEST_INPUT).1, "MCD".to_string())
    }

    #[test]
    fn check_day05_both_case1() {
        assert_eq!(day05(TEST_INPUT), ("CMZ".to_string(), "MCD".to_string()))
    }
}
