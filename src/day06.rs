use std::collections::HashSet;

fn find_first_unique_set(input: &str, len: usize) -> usize {
    for i in len-1..input.len() { // start at char #4
        let start_ix = i - (len - 1);
        let select_chars = input[start_ix..=i].to_owned();
        let charset: HashSet<char> = select_chars.chars().collect();
        if charset.len() == len {
            return i + 1;
        }
    }
    unreachable!()
}

pub fn day06(input_line: &str) -> (String, String) {
    let answer1 = find_first_unique_set(input_line, 4);
    let answer2 = find_first_unique_set(input_line, 14);
    (format!("{}", answer1), format!("{}", answer2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_day06_part1_case1() {
        assert_eq!(day06("mjqjpqmgbljsphdztnvjfqwrcgsmlb").0, "7".to_string());
        assert_eq!(day06("bvwbjplbgvbhsrlpgdmjqwftvncz").0, "5".to_string());
        assert_eq!(day06("nppdvjthqldpwncqszvftbrmjlhg").0, "6".to_string());
        assert_eq!(day06("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg").0, "10".to_string());
        assert_eq!(day06("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw").0, "11".to_string());
    }

    #[test]
    fn check_day06_part2_case1() {
        assert_eq!(day06("mjqjpqmgbljsphdztnvjfqwrcgsmlb").1, "19".to_string());
        assert_eq!(day06("bvwbjplbgvbhsrlpgdmjqwftvncz").1, "23".to_string());
        assert_eq!(day06("nppdvjthqldpwncqszvftbrmjlhg").1, "23".to_string());
        assert_eq!(day06("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg").1, "29".to_string());
        assert_eq!(day06("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw").1, "26".to_string());
    }

    // #[test]
    // fn check_day06_both_case1() {
    //     assert_eq!(day06(""), ("0".to_string(), "0".to_string()))
    // }
}
