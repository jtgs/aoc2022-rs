fn visible<'a>(trees: impl Iterator<Item = &'a i8>, height: i8) -> (bool, i32) {
    let mut seen_from_edge = true;
    let mut trees_visible = 0;

    for tree in trees {
        if *tree == -1 {
            break;
        } else if *tree < height {
            trees_visible += 1;
        } else { 
            trees_visible += 1;
            seen_from_edge = false;
            break;
        }
    }

    (seen_from_edge, trees_visible)
}

pub fn day08(input_lines: &str) -> (String, String) {
    const SIZE: usize = 99;
    let mut grid = [[-1_i8; SIZE]; SIZE];

    for (y, line) in input_lines.lines().enumerate() {
        for (x, cell) in line.chars().enumerate() {
            grid[x][y] = cell.to_digit(10).unwrap() as i8;
        }
    }

    let mut visible_trees = 0;
    let mut best_scenic_score = 0;

    for x in 0..SIZE {
        for y in 0..SIZE {
            let tree = grid[x][y];

            if tree == -1 { continue }; // empty space

            let (above_visible, above_dist) = visible(grid[x][..y].iter().rev(), tree);
            let (below_visible, below_dist) = visible(grid[x][y+1..].iter(), tree);
            let (left_visible, left_dist) = visible(grid[..x].iter().rev().map(|z| &z[y]), tree);
            let (right_visible, right_dist) = visible(grid[x+1..].iter().map(|z| &z[y]), tree);

            if above_visible || below_visible || left_visible || right_visible {
                visible_trees += 1;
            }

            let scenic_score = above_dist * below_dist * left_dist * right_dist;
            if scenic_score > best_scenic_score {
                best_scenic_score = scenic_score;
            }
        }
    }

    let answer1 = visible_trees;
    let answer2 = best_scenic_score;
    (format!("{}", answer1), format!("{}", answer2))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::helpers::load_input;

    const TEST_INPUT : &str = "30373
25512
65332
33549
35390";

    #[test]
    fn check_day08_part1_case1() {
        assert_eq!(day08(TEST_INPUT).0, "21".to_string())
    }

    #[test]
    fn check_day08_part2_case1() {
        assert_eq!(day08(TEST_INPUT).1, "8".to_string())
    }

    #[test]
    fn check_day08_both_case1() {
        assert_eq!(day08(TEST_INPUT), ("21".to_string(), "8".to_string()))
    }

    #[test]
    fn check_day08_puzzle() {
        let input = load_input(8);
        assert_eq!(day08(&input), ("1832".to_string(), "157320".to_string()))
    }
}
