use crate::helpers::split_into_lines;

pub fn day02(input_lines: &str) -> (String, String) {
    let lines = split_into_lines(input_lines);
    // A, X = Rock
    // B, Y = Paper
    // C, Z = Scissors
    let answer1: i32 = lines.iter().map(|p| match p.as_str() {
        "A X" => 4, // 1 + 3 (Rock + Draw)
        "A Y" => 8, // 2 + 6 (Paper + Win)
        "A Z" => 3, // 3 + 0 (Scissors + Lose)
        "B X" => 1, // 1 + 0
        "B Y" => 5, // 2 + 3
        "B Z" => 9, // 3 + 6
        "C X" => 7, // 1 + 6
        "C Y" => 2, // 2 + 0
        "C Z" => 6, // 3 + 3
        _ => panic!("Invalid combination!")
    }).sum();

    // X = Lose
    // Y = Draw
    // Z = Win
    let answer2: i32 = lines.iter().map(|p| match p.as_str() {
        "A X" => 3, // 3 + 0
        "A Y" => 4, // 1 + 3
        "A Z" => 8, // 2 + 6
        "B X" => 1, // 1 + 0
        "B Y" => 5, // 2 + 3
        "B Z" => 9, // 3 + 6
        "C X" => 2, // 2 + 0
        "C Y" => 6, // 3 + 3
        "C Z" => 7, // 1 + 6
        _ => panic!("Invalid combination!")
    }).sum();

    (format!("{}", answer1), format!("{}", answer2))
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "A Y
B X
C Z";

    #[test]
    fn check_day02_part1_case1() {
        assert_eq!(day02(TEST_INPUT).0, "15".to_string())
    }

    #[test]
    fn check_day02_part2_case1() {
        assert_eq!(day02(TEST_INPUT).1, "12".to_string())
    }

    #[test]
    fn check_day02_both_case1() {
        assert_eq!(day02(TEST_INPUT), ("15".to_string(), "12".to_string()))
    }
}
