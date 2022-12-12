use std::{str::FromStr, num::ParseIntError};

use lazy_static::lazy_static;
use regex::Regex;

#[derive(Clone)]
enum Operand {
    Old,
    Num(i64)
}

#[derive(Clone)]
struct Monkey {
    items: Vec<i64>,
    operation: String,
    operand: Operand,
    test_divisor: i64,
    next_if_true: i32,
    next_if_false: i32,
    items_inspected: i64,
}

impl FromStr for Monkey {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"Monkey (\d):\n  Starting items: (.*)\n  Operation: new = old (.) (.*)\n  Test: divisible by (\d+)\n    If true: throw to monkey (\d)\n    If false: throw to monkey (\d)").unwrap();
        }
        
        let caps = RE.captures(s).unwrap();
        let items: Vec<i64> = caps.get(2).unwrap().as_str().split(", ").map(|x| x.parse().unwrap()).collect();
        let operation = caps.get(3).unwrap().as_str().to_string();
        let operand = match caps.get(4).unwrap().as_str() {
            "old" => Operand::Old,
            other => Operand::Num(other.parse().unwrap()),
        };
        let test_divisor = caps.get(5).unwrap().as_str().parse()?;
        let next_if_true = caps.get(6).unwrap().as_str().parse()?;
        let next_if_false = caps.get(7).unwrap().as_str().parse()?;

        Ok(Monkey { items, operation, operand, test_divisor, next_if_true, next_if_false, items_inspected: 0 })
    }
}

pub fn day11(input_lines: &str) -> (String, String) {
    let lines = input_lines.lines().collect::<Vec<&str>>();
    let blocks = lines.split(|l| l.is_empty());
    let monkeys: Vec<Monkey> = blocks.map(|b| Monkey::from_str(b.join("\n").as_str()).unwrap()).collect();

    let mut monkeys_1 = monkeys.clone();

    for _ in 1..=20 {
        for m in 0..monkeys_1.len() {
            let monkey = monkeys_1[m].clone();
            for _ in 0..monkey.items.len() {
                let item  = monkeys_1[m].items.pop().unwrap();
                let real_operand = match monkey.operand {
                    Operand::Old => item,
                    Operand::Num(x) => x,
                };

                let mut new_worry = match monkey.operation.as_str() {
                    "+" => item + real_operand,
                    "-" => item - real_operand,
                    "*" => item * real_operand,
                    _ => unreachable!(),
                };

                new_worry /= 3;

                if new_worry % monkey.test_divisor == 0 {
                    monkeys_1[monkey.next_if_true as usize].items.push(new_worry);
                } else {
                    monkeys_1[monkey.next_if_false as usize].items.push(new_worry);
                }

                monkeys_1[m].items_inspected += 1;
            }
        }
    }

    monkeys_1.sort_by(|m, n| m.items_inspected.cmp(&n.items_inspected));
    let answer1 = monkeys_1.iter().rev().take(2).fold(1, |acc, m| acc * m.items_inspected);

    let mut monkeys_2 = monkeys;
    let divisor = monkeys_2.iter().fold(1, |acc, m| acc * m.test_divisor);
    for _ in 1..=10000 {
        for m in 0..monkeys_2.len() {
            let monkey = monkeys_2[m].clone();
            for _ in 0..monkey.items.len() {
                let item  = monkeys_2[m].items.pop().unwrap();
                let real_operand = match monkey.operand {
                    Operand::Old => item,
                    Operand::Num(x) => x,
                };

                let new_worry = match monkey.operation.as_str() {
                    "+" => item + real_operand,
                    "-" => item - real_operand,
                    "*" => item * real_operand,
                    _ => unreachable!(),
                } % divisor;

                if new_worry % monkey.test_divisor == 0 {
                    monkeys_2[monkey.next_if_true as usize].items.push(new_worry);
                } else {
                    monkeys_2[monkey.next_if_false as usize].items.push(new_worry);
                }

                monkeys_2[m].items_inspected += 1;
            }
        }
    }

    monkeys_2.sort_by(|m, n| m.items_inspected.cmp(&n.items_inspected));
    let answer2 = monkeys_2.iter().rev().take(2).fold(1, |acc, m| acc * m.items_inspected);
    (format!("{}", answer1), format!("{}", answer2))
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    #[test]
    fn check_day11_part1_case1() {
        assert_eq!(day11(TEST_INPUT).0, "10605".to_string())
    }

    #[test]
    fn check_day11_part2_case1() {
        assert_eq!(day11(TEST_INPUT).1, "2713310158".to_string())
    }

    #[test]
    fn check_day11_both_case1() {
        assert_eq!(day11(TEST_INPUT), ("10605".to_string(), "2713310158".to_string()))
    }
}
