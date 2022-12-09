use std::{str::FromStr, num::ParseIntError, collections::HashSet};

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

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct Point {
    x: i32, 
    y: i32,
}

fn move_rope(rope: &mut [Point], direction: &Direction) {
    // Move the head
    match direction {
        Direction::Up => rope[0].y -= 1,
        Direction::Down => rope[0].y += 1,
        Direction::Left => rope[0].x -= 1,
        Direction::Right => rope[0].x += 1,
    }

    // Move the remaining knots
    for i in 1..rope.len() {
        if rope[i-1].x - rope[i].x > 1 {
            if rope[i-1].y < rope[i].y {
                rope[i].y -= 1;
            } else if rope[i-1].y > rope[i].y {
                rope[i].y += 1;
            }
            rope[i].x += 1;
        } else if rope[i-1].x - rope[i].x < -1 {
            if rope[i-1].y < rope[i].y {
                rope[i].y -= 1;
            } else if rope[i-1].y > rope[i].y {
                rope[i].y += 1;
            }
            rope[i].x -= 1;
        } else if rope[i-1].y - rope[i].y > 1 {
            if rope[i-1].x < rope[i].x {
                rope[i].x -= 1;
            } else if rope[i-1].x > rope[i].x {
                rope[i].x += 1;
            }
            rope[i].y += 1;
        } else if rope[i-1].y - rope[i].y < -1 {
            if rope[i-1].x < rope[i].x {
                rope[i].x -= 1;
            } else if rope[i-1].x > rope[i].x {
                rope[i].x += 1;
            }
            rope[i].y -= 1;
        }
    }
}

pub fn day09(input_lines: &str) -> (String, String) {
    let instructions: Vec<Instruction> = input_lines.lines().map(|s| Instruction::from_str(s).unwrap()).collect();

    let mut cells_visited_a: HashSet<Point> = HashSet::new();
    let mut cells_visited_b: HashSet<Point> = HashSet::new();

    let mut rope_a = [Point {x: 0, y:0}; 2];
    cells_visited_a.insert(*rope_a.last().unwrap());

    let mut rope_b = [Point {x: 0, y:0}; 10];
    cells_visited_b.insert(*rope_b.last().unwrap());

    for instruction in instructions.iter() {
        for _ in 0..instruction.dist {
            move_rope(&mut rope_a, &instruction.dir);
            move_rope(&mut rope_b, &instruction.dir);

            // Record the position of the tail
            cells_visited_a.insert(*rope_a.last().unwrap());
            cells_visited_b.insert(*rope_b.last().unwrap());
        }
    }

    let answer1 = cells_visited_a.len();
    let answer2 = cells_visited_b.len();
    (format!("{}", answer1), format!("{}", answer2))
}

#[cfg(test)]
mod tests {
    use crate::helpers::load_input;

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
        assert_eq!(day09(TEST_INPUT).1, "1".to_string());

        let test_input_2 = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";
        assert_eq!(day09(test_input_2).1, "36".to_string());
    }

    #[test]
    fn check_day09_both_case1() {
        assert_eq!(day09(TEST_INPUT), ("13".to_string(), "1".to_string()))
    }

    #[test]
    fn check_day09_puzzle() {
        let input = load_input(9);
        assert_eq!(day09(&input), ("5930".to_string(), "2443".to_string()))
    }
}
