use crate::helpers::split_into_lines;

pub fn day01(input_lines: &str) -> (String, String) {
    // Split by double line break to get the elves' entries
    let lines = split_into_lines(input_lines);
    let elves = lines.split(|l| l.is_empty());
    // Sum up each elf's individual total
    let mut elf_totals: Vec<i32> = elves.map(|e| e.iter().map(|x| x.trim().parse::<i32>().unwrap()).sum()).collect();
    elf_totals.sort();
    elf_totals.reverse();
    let top_three = &elf_totals[0..3];

    // Part 1: find the elf with the biggest calorie count
    let answer1 = top_three[0];
    // Part 2: find the total carried by the top three elves
    let answer2: i32 = top_three.iter().sum();
    (format!("{}", answer1), format!("{}", answer2))
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn check_day01_part1_case1() {
        assert_eq!(day01(TEST_INPUT).0, "24000".to_string())
    }

    #[test]
    fn check_day01_part2_case1() {
        assert_eq!(day01(TEST_INPUT).1, "45000".to_string())
    }

    #[test]
    fn check_day01_both_case1() {
        assert_eq!(day01(TEST_INPUT), ("24000".to_string(), "45000".to_string()))
    }
}
