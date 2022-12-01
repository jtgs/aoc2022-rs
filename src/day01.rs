#[cfg(windows)]
const LINE_ENDING: &'static str = "\r\n";
#[cfg(not(windows))]
const LINE_ENDING: &'static str = "\n";

pub fn day01(input_lines: &str) -> (String, String) {
    // Split by double line break to get the elves' entries
    let double_line_break = format!("{}{}", LINE_ENDING, LINE_ENDING);
    let elves = input_lines.split(&double_line_break);
    // Sum up each elf's individual total
    let elf_totals: Vec<i32> = elves.map(|e| e.split(LINE_ENDING).map(|x| x.trim().parse::<i32>().unwrap()).sum()).collect();

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
        #[cfg(windows)]
        let input = "1000\r\n2000\r\n3000\r\n\r\n4000\r\n\r\n5000\r\n6000\r\n\r\n7000\r\n8000\r\n9000\r\n\r\n10000";
        #[cfg(not(windows))]
        let input = "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000";

        assert_eq!(day01(input).0, "24000".to_string())
    }

    #[test]
    fn check_day01_part2_case1() {
        #[cfg(windows)]
        let input = "1000\r\n2000\r\n3000\r\n\r\n4000\r\n\r\n5000\r\n6000\r\n\r\n7000\r\n8000\r\n9000\r\n\r\n10000";
        #[cfg(not(windows))]
        let input = "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000";

        assert_eq!(day01(input).1, "45000".to_string())
    }

    #[test]
    fn check_day01_both_case1() {
        #[cfg(windows)]
        let input = "1000\r\n2000\r\n3000\r\n\r\n4000\r\n\r\n5000\r\n6000\r\n\r\n7000\r\n8000\r\n9000\r\n\r\n10000";
        #[cfg(not(windows))]
        let input = "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000";

        assert_eq!(day01(input), ("24000".to_string(), "45000".to_string()))
    }
}
