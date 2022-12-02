use crate::helpers::split_into_lines;

// pub fn day02_alt(input_lines: &str) -> (String, String) {
//     let lines = split_into_lines(input_lines);
//     // A, X = Rock
//     // B, Y = Paper
//     // C, Z = Scissors
//     let answer1: i32 = lines.iter().map(|row| match row.as_str() {
//         "A X" => 4, // 1 + 3 (Rock + Draw)
//         "A Y" => 8, // 2 + 6 (Paper + Win)
//         "A Z" => 3, // 3 + 0 (Scissors + Lose)
//         "B X" => 1, // 1 + 0
//         "B Y" => 5, // 2 + 3
//         "B Z" => 9, // 3 + 6
//         "C X" => 7, // 1 + 6
//         "C Y" => 2, // 2 + 0
//         "C Z" => 6, // 3 + 3
//         _ => panic!("Invalid combination!")
//     }).sum();

//     // X = Lose
//     // Y = Draw
//     // Z = Win
//     let answer2: i32 = lines.iter().map(|row| match row.as_str() {
//         "A X" => 3, // 3 + 0
//         "A Y" => 4, // 1 + 3
//         "A Z" => 8, // 2 + 6
//         "B X" => 1, // 1 + 0
//         "B Y" => 5, // 2 + 3
//         "B Z" => 9, // 3 + 6
//         "C X" => 2, // 2 + 0
//         "C Y" => 6, // 3 + 3
//         "C Z" => 7, // 1 + 6
//         _ => panic!("Invalid combination!")
//     }).sum();

//     (format!("{}", answer1), format!("{}", answer2))
// }

enum RPS {
    Rock,
    Paper,
    Scissors
}

enum Result {
    Win,
    Lose,
    Draw
}

fn score_for_type(input: &RPS) -> i32 {
    match input {
        RPS::Rock => 1,
        RPS::Paper => 2,
        RPS::Scissors => 3
    }
}

fn play_rps(tuple: &(RPS, RPS)) -> Result {
    match tuple.1 {
        RPS::Rock => match tuple.0 {
            RPS::Rock => Result::Draw,
            RPS::Paper => Result::Lose,
            RPS::Scissors => Result::Win
        },
        RPS::Paper => match tuple.0 {
            RPS::Rock => Result::Win,
            RPS::Paper => Result::Draw,
            RPS::Scissors => Result::Lose
        },
        RPS::Scissors => match tuple.0 {
            RPS::Rock => Result::Lose,
            RPS::Paper => Result::Win,
            RPS::Scissors => Result::Draw
        },
    }
}

fn points_for_result(result: &Result) -> i32 {
    match result {
        Result::Win => 6,
        Result::Lose => 0,
        Result::Draw => 3
    }
}

fn what_to_play(tuple: &(RPS, Result)) -> RPS {
    match tuple.0 {
        RPS::Rock => match tuple.1 {
            Result::Win => RPS::Paper,
            Result::Lose => RPS::Scissors,
            Result::Draw => RPS::Rock
        },
        RPS::Paper => match tuple.1 {
            Result::Win => RPS::Scissors,
            Result::Lose => RPS::Rock,
            Result::Draw => RPS::Paper
        },
        RPS::Scissors => match tuple.1 {
            Result::Win => RPS::Rock,
            Result::Lose => RPS::Paper,
            Result::Draw => RPS::Scissors
        },
    }
}

pub fn day02(input_lines: &str) -> (String, String) {
    let lines = split_into_lines(input_lines);

    let answer1: i32 = lines
        .iter()
        .map(|row| {
            let theirs = match row.as_bytes()[0] as char {
                'A' => RPS::Rock,
                'B' => RPS::Paper,
                'C' => RPS::Scissors,
                _ => panic!("Invalid character!")
            };
            let mine = match row.as_bytes()[2] as char {
                'X' => RPS::Rock,
                'Y' => RPS::Paper,
                'Z' => RPS::Scissors,
                _ => panic!("Invalid character!")
            };
            (theirs, mine)
            })
        .map(|tuple| score_for_type(&tuple.1) + points_for_result(&play_rps(&tuple)))
        .sum();

    let answer2: i32 = lines
        .iter()
        .map(|row| {
            let theirs = match row.as_bytes()[0] as char {
                'A' => RPS::Rock,
                'B' => RPS::Paper,
                'C' => RPS::Scissors,
                _ => panic!("Invalid character!")
            };
            let mine = match row.as_bytes()[2] as char {
                'X' => Result::Lose,
                'Y' => Result::Draw,
                'Z' => Result::Win,
                _ => panic!("Invalid character!")
            };
            (theirs, mine)
            })
        .map(|tuple| {
            score_for_type(&what_to_play(&tuple)) + points_for_result(&tuple.1)
        })
        .sum();
    
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
        assert_eq!(day02(TEST_INPUT).0, "15".to_string());
        assert_eq!(day02_alt(TEST_INPUT).0, "15".to_string());
    }

    #[test]
    fn check_day02_part2_case1() {
        assert_eq!(day02(TEST_INPUT).1, "12".to_string());
        assert_eq!(day02_alt(TEST_INPUT).1, "12".to_string());
    }

    #[test]
    fn check_day02_both_case1() {
        assert_eq!(day02(TEST_INPUT), ("15".to_string(), "12".to_string()));
        assert_eq!(day02_alt(TEST_INPUT), ("15".to_string(), "12".to_string()));
    }
}
