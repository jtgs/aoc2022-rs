use std::collections::HashSet;

fn priority_for_char(c: &char) -> i32 {
    match *c {
        'a'..='z' => *c as i32 - 96,
        'A'..='Z' => *c as i32 - 38,
        _ => panic!("not alphabetic!")
    }
}

pub fn day03(input_lines: &str) -> (String, String) {
    let rucksacks: Vec<&str> = input_lines.lines().collect();

    let answer1: i32 = rucksacks.iter().map(|r| {
        let mut chars: Vec<char> = r.chars().collect();
        let compartment_size = r.len() / 2;
        let second_half = chars.split_off(compartment_size);
        let a = HashSet::<char>::from_iter(chars);
        let b = HashSet::<char>::from_iter(second_half);
        let mut i = a.intersection(&b);

        priority_for_char(i.next().unwrap())
    }).sum();

    let answer2: i32 = rucksacks.chunks(3).map(|c| {
        let a = HashSet::<char>::from_iter(c[0].chars());
        let b = HashSet::<char>::from_iter(c[1].chars());
        let c = HashSet::<char>::from_iter(c[2].chars());

        let i = a.intersection(&b).map(|c| c.to_owned()).collect::<HashSet<char>>();
        let mut j = i.intersection(&c);

        priority_for_char(j.next().unwrap())
    }).sum();

    (format!("{}", answer1), format!("{}", answer2))
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn check_day03_part1_case1() {
        assert_eq!(day03(TEST_INPUT).0, "157".to_string())
    }

    #[test]
    fn check_day03_part2_case1() {
        assert_eq!(day03(TEST_INPUT).1, "70".to_string())
    }

    #[test]
    fn check_day03_both_case1() {
        assert_eq!(day03(TEST_INPUT), ("157".to_string(), "70".to_string()))
    }
}
