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

fn let_sand_fall(sand_source: &Point, wall_points: &Vec<Point>, sand_locs: &HashSet<Point>, bottom: i32) -> Option<Point> {
    let mut curr = sand_source.clone();
    let mut obstacles: HashSet<&Point> = wall_points.iter().collect();
    sand_locs.iter().for_each(|l| {obstacles.insert(l);});

    loop {
        // Have we fallen off the edge?
        if curr.1 >= bottom {
            return None
        }

        // check below
        if !obstacles.contains(&Point(curr.0, curr.1 + 1)) {
            curr.1 += 1;
            continue;
        }

        // check left
        if !obstacles.contains(&Point(curr.0 - 1, curr.1 + 1)) {
            curr.1 += 1;
            curr.0 -= 1;
            continue;
        }

        // check right
        if !obstacles.contains(&Point(curr.0 + 1, curr.1 + 1)) {
            curr.1 += 1;
            curr.0 += 1;
            continue;
        }

        // must come to rest
        break;
    }
    // println!("{:?}", curr);
    Some(curr)
}

fn draw_it(wall_points: &Vec<Point>, sand_locs: &HashSet<Point>, left: i32, right: i32, height: i32) {
    for y in 0..=height {
        let mut line: Vec<&str> = Vec::new();

        for x in left..=right {
            if wall_points.contains(&Point(x, y)) {
                line.push("#");
            } else if sand_locs.contains(&Point(x, y)) {
                line.push("O");
            } else {
                line.push(".");
            }
        }

        println!("{}", line.join(""));
    }
}

pub fn day14(input_lines: &str) -> (String, String) {
    let walls: Vec<Vec<Point>> = input_lines.lines().map(|l| {
        l.split(" -> ").map(|t| Point::from_str(t).unwrap()).collect()
    }).collect();
    let sand_source = Point(500, 0);

    // list of points that are in a wall
    let wall_points: Vec<Point> = walls.iter().flat_map(|w| list_of_points_in_wall(w)).collect();

    // list of points that have sand in them
    let mut sand_locs = HashSet::new();

    // where is the bottommost wall?
    let bottom = wall_points.iter().map(|p| p.1).max().unwrap();

    // Part 1: no floor
    loop {
        match let_sand_fall(&sand_source, &wall_points, &sand_locs, bottom) {
            Some(p) => if !sand_locs.insert(p) { panic!("double stacking!") },
            None => break,
        }
    }
    println!("Answer 1 is {}", sand_locs.len());
    let answer1 = sand_locs.len();

    // Part 2: floor is 2 below the lowest wall
    let new_bottom = bottom + 2;
    let left = wall_points.iter().map(|p| p.0).min().unwrap();
    let right = wall_points.iter().map(|p| p.0).max().unwrap();
    let mut new_wall = list_of_points_in_wall(&vec!(Point(left - 1000, new_bottom), Point(right + 1000, new_bottom)));
    let mut new_wall_points = wall_points.clone();
    new_wall_points.append(&mut new_wall);
    let mut sand_locs_2 = HashSet::new();

    loop {
        match let_sand_fall(&sand_source, &new_wall_points, &sand_locs_2, new_bottom) {
            Some(Point(500, 0)) => { sand_locs_2.insert(Point(500, 0)); break; },
            Some(p) => { 
                if !sand_locs_2.insert(p) { panic!("double stacking!") }
                println!("{}", sand_locs_2.len());
            },
            None => panic!("ran out of bottom!"), 
        }
    }

    draw_it(&new_wall_points, &sand_locs_2, left - 10, right + 10, new_bottom);
    let answer2 = sand_locs_2.len();
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
