use std::{str::FromStr, num::ParseIntError, collections::HashSet};
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
struct Point(i32, i32);

#[derive(Debug)]
struct Sensor {
    location: Point,
    closest: Point,
    distance: i32,
}

fn manhattan_distance(a: &Point, b: &Point) -> i32 {
    i32::abs(a.0 - b.0) + i32::abs(a.1 - b.1)
}

fn is_within_range(sensor: &Sensor, point: &Point) -> bool {
    let separation = manhattan_distance(&sensor.location, point);
    separation <= sensor.distance
}

impl FromStr for Sensor {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"Sensor at x=(\S+), y=(\S+): closest beacon is at x=(\S+), y=(\S+)").unwrap();
        }

        let caps = RE.captures(s).unwrap();
        let s_x = caps.get(1).unwrap().as_str().parse()?;
        let s_y = caps.get(2).unwrap().as_str().parse()?;
        let b_x = caps.get(3).unwrap().as_str().parse()?;
        let b_y = caps.get(4).unwrap().as_str().parse()?;

        let location = Point(s_x, s_y);
        let closest = Point(b_x, b_y);
        let distance = manhattan_distance(&location, &closest);

        Ok(Sensor {
            location,
            closest,
            distance,
        })
    }
}
 
fn day15_inner(input_lines: &str, target_y: i32) -> (String, String) {
    let sensors: Vec<Sensor> = input_lines.lines().map(|l| Sensor::from_str(l).unwrap()).collect();

    // Part 1: how many squares where a beacon cannot be present, in row target_y
    let minimum_x = sensors.iter().map(|s| s.location.0 - s.distance).min().unwrap();
    let maximum_x = sensors.iter().map(|s| s.location.0 + s.distance).max().unwrap();

    let mut impossible_points_count = 0;
    for x in minimum_x..=maximum_x {
        let mut in_range = false;
        for sensor in &sensors {
            if is_within_range(&sensor, &Point(x, target_y)) {
                in_range = true;
                break;
            }
        }
        if in_range {
            impossible_points_count += 1;
        }
    }

    // Remove the spots that are actually beacons
    impossible_points_count -= sensors.iter().map(|s| s.closest).filter(|p| p.1 == target_y).collect::<HashSet<Point>>().len();

    let answer1 = impossible_points_count;
    
    let answer2 = 0;
    (format!("{}", answer1), format!("{}", answer2))
}

pub fn day15(input_lines: &str) -> (String, String) {
    let (answer1, answer2) = day15_inner(input_lines, 2000000);
    (answer1, answer2)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

    #[test]
    fn check_day15_part1_case1() {
        assert_eq!(day15_inner(TEST_INPUT, 10).0, "26".to_string())
    }

    #[test]
    fn check_day15_part2_case1() {
        assert_eq!(day15_inner(TEST_INPUT, 10).1, "56000011".to_string())
    }

    #[test]
    fn check_day15_both_case1() {
        assert_eq!(day15_inner(TEST_INPUT, 10), ("26".to_string(), "56000011".to_string()))
    }
}
