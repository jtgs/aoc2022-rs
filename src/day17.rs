use std::collections::HashSet;

enum Shape {
    HLine,
    Plus,
    L,
    VLine,
    Square,
}

const SHAPES: [Shape; 5] = [
    Shape::HLine,
    Shape::Plus,
    Shape::L,
    Shape::VLine,
    Shape::Square,
];

type Point = (i32, i32);

#[derive(Copy, Clone)]
enum Direction {
    Left,
    Right,
}

fn get_rock(shape: &Shape, y: i32) -> HashSet<Point> {
    let mut s = HashSet::new();
    match shape {
        Shape::HLine => {
            s.insert((2, y));
            s.insert((3, y));
            s.insert((4, y));
            s.insert((5, y));
        },
        Shape::Plus => {
            s.insert((3, y+2));
            s.insert((2, y+1));
            s.insert((3, y+1));
            s.insert((4, y+1));
            s.insert((3, y));
        },
        Shape::L => {
            s.insert((2, y));
            s.insert((3, y));
            s.insert((4, y));
            s.insert((4, y+1));
            s.insert((4, y+2));
        },
        Shape::VLine => {
            s.insert((2, y));
            s.insert((2, y+1));
            s.insert((2, y+2));
            s.insert((2, y+3));
        },
        Shape::Square => {
            s.insert((2, y));
            s.insert((2, y+1));
            s.insert((3, y));
            s.insert((3, y+1));
        },
    }

    s
}

fn move_left(rock: HashSet<Point>) -> HashSet<Point> {
    if rock.iter().any(|p| p.0 == 0) {
        return rock;
    }

    rock.iter().map(|p| (p.0 - 1, p.1)).collect()
}

const MAX_X: i32 = 6;

fn move_right(rock: HashSet<Point>) -> HashSet<Point> {
    if rock.iter().any(|p| p.0 == MAX_X) {
        return rock;
    }

    rock.iter().map(|p| (p.0 + 1, p.1)).collect()
}

fn move_down(rock: HashSet<Point>) -> HashSet<Point> {
    rock.iter().map(|p| (p.0, p.1 - 1)).collect()
}

fn move_up(rock: HashSet<Point>) -> HashSet<Point> {
    rock.iter().map(|p| (p.0, p.1 + 1)).collect()
}

fn draw(board: &HashSet<Point>) {
    let max_y = board.iter().map(|p| p.1).max().unwrap();

    for y in 0..=max_y {
        let mut row = "".to_string();
        for x in 0..7 {
            if board.contains(&(x, max_y - y)) {
                row = format!("{}{}", row, "#");
            } else {
                row = format!("{}{}", row, " ");
            }
        }
        println!("{}", row);
    }
}

pub fn day17(input_lines: &str) -> (String, String) {
    let moves: Vec<Direction> = input_lines.chars().map(|c| match c {
        '<' => Direction::Left,
        '>' => Direction::Right,
        _ => panic!("invalid move!"),
    }).collect();

    let mut board = HashSet::<Point>::new();
    for x in 0..=MAX_X {
        board.insert((x, 0));
    }
    let mut top = 0;
    let mut move_counter = 0;

    for ix in 0..2022 {
        let mut rock = get_rock(&SHAPES[ix % SHAPES.len()], top + 4);

        loop {
            let next_move = moves[move_counter % moves.len()];

            match next_move {
                Direction::Left => {
                    rock = move_left(rock);

                    if rock.intersection(&board).count() > 0 {
                        rock = move_right(rock);
                    }
                },
                Direction::Right => {
                    rock = move_right(rock);

                    if rock.intersection(&board).count() > 0 {
                        rock = move_left(rock);
                    }
                },
            };
            move_counter += 1;

            rock = move_down(rock);
            if rock.intersection(&board).count() > 0 {
                rock = move_up(rock);
                board = rock.union(&board).map(|t| t.to_owned()).collect();
                top = board.iter().map(|p| p.1).max().unwrap();
                break;
            }

        }
        // println!("done rock {}", ix);
        // draw(&board);
    }

    let answer1 = top;
    let answer2 = 0;
    (format!("{}", answer1), format!("{}", answer2))
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    #[test]
    fn check_day17_part1_case1() {
        assert_eq!(day17(TEST_INPUT).0, "3068".to_string())
    }

    #[test]
    fn check_day17_part2_case1() {
        assert_eq!(day17("").1, "0".to_string())
    }

    #[test]
    fn check_day17_both_case1() {
        assert_eq!(day17(""), ("0".to_string(), "0".to_string()))
    }
}
