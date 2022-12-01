pub fn day01(input_lines: &str) -> (String, String) {
    // Split by double line break to get the elves' entries
    let elves = input_lines.split("\r\n\r\n");
    // Sum up each elf's individual total
    let elf_totals: Vec<i32> = elves.map(|e| e.split("\r\n").map(|x| x.trim().parse::<i32>().unwrap()).sum()).collect();

    // Part 1: find the elf with the biggest calorie count
    let answer1 = elf_totals.iter().max().unwrap();
    // Part 2: find the total carried by the top three elves
    let mut sorted = elf_totals.clone();
    sorted.sort();
    let answer2: i32 = sorted.iter().rev().take(3).sum();
    (format!("{}", answer1), format!("{}", answer2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_day01_part1_case1() {
        assert_eq!(day01("").0, "0".to_string())
    }

    #[test]
    fn check_day01_part2_case1() {
        assert_eq!(day01("").1, "0".to_string())
    }

    #[test]
    fn check_day01_both_case1() {
        assert_eq!(day01(""), ("0".to_string(), "0".to_string()))
    }
}
