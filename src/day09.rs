use std::{str::FromStr, num::ParseIntError, collections::HashSet};

use crate::helpers::split_into_lines;

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Instruction {
    dir: Direction,
    dist: u8,
}

impl FromStr for Instruction {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // It's easier if there is only one delimiter
        let (a, b) = s.split_once(' ').unwrap();
        let dir = match a {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => unreachable!()
        };
        let dist: u8 = b.parse()?;

        Ok(Instruction { dir, dist })
    }
}

pub fn day09(input_lines: &str) -> (String, String) {
    let rows = split_into_lines(input_lines);
    let instructions: Vec<Instruction> = rows.iter().map(|s| Instruction::from_str(s).unwrap()).collect();

    let mut cells_visited: HashSet<(i32, i32)> = HashSet::new();

    let mut head_x = 0;
    let mut head_y = 0;
    let mut tail_x = 0;
    let mut tail_y = 0;
    cells_visited.insert((tail_x, tail_y));

    for instruction in instructions.iter() {
        for _ in 0..instruction.dist {
            // Move the head
            match &instruction.dir {
                Direction::Up => head_y -= 1,
                Direction::Down => head_y += 1,
                Direction::Left => head_x -= 1,
                Direction::Right => head_x += 1,
            }

            let old_tail = (tail_x, tail_y);
            // Move the tail?
            if head_x - tail_x > 1 {
                if head_y != tail_y {
                    tail_y = head_y;
                }
                tail_x += 1;
            } else if head_x - tail_x < -1 {
                if head_y != tail_y {
                    tail_y = head_y;
                }
                tail_x -= 1;
            } else if head_y - tail_y > 1 {
                if head_x != tail_x {
                    tail_x = head_x;
                }
                tail_y += 1;
            } else if head_y - tail_y < -1 {
                if head_x != tail_x {
                    tail_x = head_x;
                }
                tail_y -= 1;
            }

            if (tail_x, tail_y) != old_tail {
                cells_visited.insert((tail_x, tail_y));
            }
        }
    }

    let answer1 = cells_visited.len();
    let answer2 = 0;
    (format!("{}", answer1), format!("{}", answer2))
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    #[test]
    fn check_day09_part1_case1() {
        assert_eq!(day09(TEST_INPUT).0, "13".to_string())
    }

    #[test]
    fn check_day09_part2_case1() {
        assert_eq!(day09("").1, "0".to_string())
    }

    #[test]
    fn check_day09_both_case1() {
        assert_eq!(day09(""), ("0".to_string(), "0".to_string()))
    }
}
