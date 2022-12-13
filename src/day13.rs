use itertools::Itertools;
use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Packet(serde_json::Value);

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (&self.0, &other.0) {
            // both numbers -> left should be smaller
            (Value::Number(a), Value::Number(b)) => a.as_u64().unwrap().cmp(&b.as_u64().unwrap()),
            // both lists -> for first non-equal value, left should be smaller; else left should be shorter
            (Value::Array(a), Value::Array(b)) => {
                std::iter::zip(a, b)
                    .map(|(a, b)| (Packet(a.clone()).cmp(&Packet(b.clone()))))
                    .find(|x| *x != std::cmp::Ordering::Equal)
                    .unwrap_or_else(|| a.len().cmp(&b.len()))
            },
            // mixed -> convert number to list and re-compare
            (Value::Array(_), Value::Number(_)) => {
                self.cmp(&Packet(Value::Array(vec![other.0.clone()])))
            },
            (Value::Number(_), Value::Array(_)) => {
                Packet(Value::Array(vec![self.0.clone()])).cmp(other)
            },
            _ => unreachable!(),
        }
    }
}

pub fn day13(input_lines: &str) -> (String, String) {
    let lines: Vec<&str> = input_lines.lines().collect();
    let pairs = lines.split(|l| l.is_empty());
    let mut correctly_ordered_pairs = Vec::new();

    for (i, pair) in pairs.enumerate() {
        let first = Packet(serde_json::from_str(pair[0]).unwrap());
        let second = Packet(serde_json::from_str(pair[1]).unwrap());

        if first < second {
            correctly_ordered_pairs.push(i + 1);
        }
    }
    let answer1: usize = correctly_ordered_pairs.iter().sum();

    let mut part2_lines = lines.clone();
    let mut dividers = vec!["[[2]]", "[[6]]"];
    let divider_packets: Vec<Packet> = dividers.iter().map(|p| Packet(serde_json::from_str(p).unwrap())).collect();
    part2_lines.append(&mut dividers);
    let lines = part2_lines
        .iter()
        .filter(|l| !l.is_empty())
        .map(|l| Packet(serde_json::from_str(l).unwrap()))
        .sorted();

    let answer2: usize = lines
        .enumerate()
        .filter_map(|(i, x)| {
            if divider_packets.contains(&x) {
                Some(i + 1)
            } else {
                None
            }
        })
        .product();
    (format!("{}", answer1), format!("{}", answer2))
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

    #[test]
    fn check_day13_part1_case1() {
        assert_eq!(day13(TEST_INPUT).0, "13".to_string())
    }

    #[test]
    fn check_day13_part2_case1() {
        assert_eq!(day13(TEST_INPUT).1, "140".to_string())
    }

    #[test]
    fn check_day13_both_case1() {
        assert_eq!(day13(TEST_INPUT), ("13".to_string(), "140".to_string()))
    }
}
