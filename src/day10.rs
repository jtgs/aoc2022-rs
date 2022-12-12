use std::{str::FromStr, num::ParseIntError};

enum Operation {
    Addx(i32),
    Noop,
}

impl FromStr for Operation {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // It's easier if there is only one delimiter
        let a: &str;
        let mut b = "";
        if s.contains(' ') {
            (a, b) = s.split_once(' ').unwrap();
        } else {
            a = s;
        }

        let op = match a {
            "addx" => Operation::Addx(b.parse().unwrap()),
            "noop" => Operation::Noop,
            _ => unreachable!()
        };

        Ok(op)
    }
}

pub fn day10(input_lines: &str) -> (String, String) {
    let operations: Vec<Operation> = input_lines.lines().map(|s| Operation::from_str(s).unwrap()).collect();
    let interesting_cycles = [20, 60, 100, 140, 180, 220];

    let mut pc = 1;
    let mut x = 1;
    let mut signal_strengths = 0;
    let mut crt = "\n".to_string();

    for op in operations {
        if interesting_cycles.contains(&pc) {
            signal_strengths += x * pc;
        }

        let crt_pos = (pc - 1) % 40;  
        if x == crt_pos || x - 1 == crt_pos || x + 1 == crt_pos {
            crt = format!("{}{}", crt, "#");
        } else {
            crt = format!("{}{}", crt, ".");
        }
        if pc % 40 == 0 {
            crt = format!("{}{}", crt, "\n");
        }

        pc += 1;

        match op {
            Operation::Noop => (),
            Operation::Addx(v) => {
                if interesting_cycles.contains(&pc) {
                    signal_strengths += x * pc;
                }

                let crt_pos = (pc - 1) % 40;                
                if x == crt_pos || x - 1 == crt_pos || x + 1 == crt_pos {
                    crt = format!("{}{}", crt, "#");
                } else {
                    crt = format!("{}{}", crt, ".");
                }
                if pc % 40 == 0 {
                    crt = format!("{}{}", crt, "\n");
                }

                x += v;
                pc += 1;
            }
        }
    }

    let answer1 = signal_strengths;
    let answer2 = crt;
    (format!("{}", answer1), answer2)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

    #[test]
    fn check_day10_part1_case_tiny() {
        let input = "noop
addx 3
addx -5";
        assert_eq!(day10(input).0, "0".to_string())
    }

    #[test]
    fn check_day10_part1_case1() {
        assert_eq!(day10(TEST_INPUT).0, "13140".to_string())
    }

    const PART_2_ANSWER: &str = "
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
";
    #[test]
    fn check_day10_part2_case1() {
        assert_eq!(day10(TEST_INPUT).1, PART_2_ANSWER.to_string())
    }

    #[test]
    fn check_day10_both_case1() {
        assert_eq!(day10(TEST_INPUT), ("13140".to_string(), PART_2_ANSWER.to_string()))
    }
}
