use std::cmp::{max, min};

use crate::helpers::split_into_lines;

pub fn day04(input_lines: &str) -> (String, String) {
    let pairs = split_into_lines(input_lines);
    let parsed: Vec<Vec<(i32, i32)>> = pairs.iter().map(|p| {
        let elves = p.split(',');
        elves.map(|e| {
            let range_ends: Vec<&str> = e.split('-').collect();
            (range_ends[0].trim().parse().unwrap(), range_ends[1].trim().parse().unwrap())
        }).collect()
    }).collect();

    let answer1 = parsed.iter().filter(|elf_ranges| {
        (elf_ranges[0].0 <= elf_ranges[1].0 && elf_ranges[0].1 >= elf_ranges[1].1) ||
        (elf_ranges[1].0 <= elf_ranges[0].0 && elf_ranges[1].1 >= elf_ranges[0].1)
    }).count();

    let answer2 = parsed.iter().filter(|elf_ranges| {
        max(elf_ranges[0].0, elf_ranges[1].0) <= min(elf_ranges[0].1, elf_ranges[1].1)
    }).count();
    (format!("{}", answer1), format!("{}", answer2))
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn check_day04_part1_case1() {
        assert_eq!(day04(TEST_INPUT).0, "2".to_string())
    }

    #[test]
    fn check_day04_part2_case1() {
        assert_eq!(day04(TEST_INPUT).1, "4".to_string())
    }

    #[test]
    fn check_day04_both_case1() {
        assert_eq!(day04(TEST_INPUT), ("2".to_string(), "4".to_string()))
    }
}
