use std::{str::FromStr, num::ParseIntError, collections::HashSet};

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct Point(i32, i32);

impl FromStr for Point {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (a, b) = s.split_once(',').unwrap();
        let x = a.parse()?;
        let y = b.parse()?;
        Ok(Point(x, y))
    }
}

fn list_of_points_in_wall(wall: &Vec<Point>) -> Vec<Point> {
    let mut wall_points = Vec::new();
    for ix in 0..wall.len() - 1 {
        let mut start = wall[ix].clone();
        let end = &wall[ix + 1];
        wall_points.push(start.clone());

        if start.0 != end.0 && start.1 != end.1 {
            panic!("diagonal wall!");
        }

        while start.0 != end.0 {
            start.0 += (end.0 - start.0) / (i32::abs(end.0 - start.0));
            wall_points.push(start.clone());
        }

        while start.1 != end.1 {
            start.1 += (end.1 - start.1) / (i32::abs(end.1 - start.1));
            wall_points.push(start.clone());
        }
    }
    wall_points
}

fn let_sand_fall(sand_source: &Point, obstacles: &HashSet<Point>, bottom: i32, hard_bottom: bool) -> Option<Point> {
    let mut curr = sand_source.clone();

    let blocked = |next: &Point| {
        obstacles.contains(&next) || (next.1 == bottom && hard_bottom)
    };

    'outer: 
    loop {
        // Have we fallen off the edge?
        if curr.1 >= bottom {
            return None
        }

        // check below, left, right
        for p in [Point(curr.0, curr.1 + 1), Point(curr.0 - 1, curr.1 + 1), Point(curr.0 + 1, curr.1 + 1)] {
            if !blocked(&p) {
                curr = p;
                continue 'outer;
            }
        }

        // must come to rest
        break;
    }

    Some(curr)
}

pub fn day14(input_lines: &str) -> (String, String) {
    let walls: Vec<Vec<Point>> = input_lines.lines().map(|l| {
        l.split(" -> ").map(|t| Point::from_str(t).unwrap()).collect()
    }).collect();
    let sand_source = Point(500, 0);

    // list of points that are in a wall
    let wall_points: HashSet<Point> = walls.iter().flat_map(|w| list_of_points_in_wall(w)).collect();

    // where is the bottommost wall?
    let bottom = wall_points.iter().map(|p| p.1).max().unwrap();

    // Part 1: no floor
    let mut obstacles_1 = wall_points.clone().into_iter().collect();
    loop {
        match let_sand_fall(&sand_source, &obstacles_1, bottom, false) {
            Some(p) => if !obstacles_1.insert(p) { panic!("double stacking!") },
            None => break,
        }
    }
    println!("Answer 1 is {}", obstacles_1.len() - wall_points.len());
    let answer1 = obstacles_1.len() - wall_points.len();

    // Part 2: floor is 2 below the lowest wall
    let new_bottom = bottom + 2;
    let mut obstacles_2 = wall_points.clone().into_iter().collect();

    loop {
        match let_sand_fall(&sand_source, &obstacles_2, new_bottom, true) {
            Some(Point(500, 0)) => { obstacles_2.insert(Point(500, 0)); break; },
            Some(p) => { 
                if !obstacles_2.insert(p) { panic!("double stacking!") }
            },
            None => panic!("ran out of bottom!"), 
        }
    }

    let answer2 = obstacles_2.len() - wall_points.len();
    (format!("{}", answer1), format!("{}", answer2))
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

    #[test]
    fn check_day14_part1_case1() {
        assert_eq!(day14(TEST_INPUT).0, "24".to_string())
    }

    #[test]
    fn check_day14_part2_case1() {
        assert_eq!(day14(TEST_INPUT).1, "93".to_string())
    }

    #[test]
    fn check_day14_both_case1() {
        assert_eq!(day14(TEST_INPUT), ("24".to_string(), "93".to_string()))
    }
}
