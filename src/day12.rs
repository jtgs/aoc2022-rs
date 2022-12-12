use std::collections::{VecDeque, HashSet};

type Point = (usize, usize);
type Grid = Vec<Vec<i32>>;

fn try_to_step(
    grid: &Grid, 
    queue: &mut VecDeque<Point>, 
    visited_squares: &mut HashSet<Point>, 
    current_point: Point, 
    new_point: Point,
) {
    let old_val = grid[current_point.1][current_point.0];
    let new_val = grid[new_point.1][new_point.0];

    if -new_val + old_val <= 1 {
        if visited_squares.insert(new_point) {
            queue.push_back(new_point);
        }
    }
}

fn main(grid: &Grid, start: &Point, end: &Point, part_two: bool) -> usize {
    let mut queue = VecDeque::<Point>::new();
    let mut visited_squares = HashSet::<Point>::new();
    let mut steps = 0;
    queue.push_back(*end);

    'main_loop:
    while !queue.is_empty() {
        for _ in 0..queue.len() {
            let (x, y) = queue.pop_front().unwrap();

            if (!part_two && (x, y) == *start) ||
               (part_two && grid[y][x] == 'a' as i32) {
                break 'main_loop;
            }

            if x > 0 { // try to step left
                try_to_step(&grid, &mut queue, &mut visited_squares, (x, y), (x - 1, y));
            }
            
            if x + 1 < grid[0].len() { // try to step right
                try_to_step(&grid, &mut queue, &mut visited_squares, (x, y), (x + 1, y));
            }
            
            if y > 0 { // try to step up
                try_to_step(&grid, &mut queue, &mut visited_squares, (x, y), (x, y - 1));
            }
            
            if y + 1 < grid.len() { // try to step down
                try_to_step(&grid, &mut queue, &mut visited_squares, (x, y), (x, y + 1));
            }
        }
        steps += 1;
        
        if steps > grid.len() * grid[0].len() {
            panic!("too many steps");
        }
    }

    steps
}


pub fn day12(input_lines: &str) -> (String, String) {
    let mut grid: Grid = Vec::new();
    let mut start = (0, 0);
    let mut end = (0, 0);

    for (y, line) in input_lines.lines().enumerate() {
        grid.push(line.chars().enumerate().map(|(x, c)| {
            match c {
                'S' => {
                    start = (x, y);
                    'a' as i32
                },
                'E' => {
                    end = (x, y);
                    'z' as i32
                },
                _ => c as i32,
            }
        }).collect());
    }

    let answer1 = main(&grid, &start, &end, false);
    let answer2 = main(&grid, &start, &end, true);
    (format!("{}", answer1), format!("{}", answer2))
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    #[test]
    fn check_day12_part1_case1() {
        assert_eq!(day12(TEST_INPUT).0, "31".to_string())
    }

    #[test]
    fn check_day12_part2_case1() {
        assert_eq!(day12(TEST_INPUT).1, "29".to_string())
    }

    #[test]
    fn check_day12_both_case1() {
        assert_eq!(day12(TEST_INPUT), ("31".to_string(), "29".to_string()))
    }
}
