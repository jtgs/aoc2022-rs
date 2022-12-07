use std::collections::HashMap;

use crate::helpers::split_into_lines;

struct DirectoryNode {
    size: i64,
    children: Vec<String>,
}

pub fn day07(input_lines: &str) -> (String, String) {
    let root = DirectoryNode {
        size: 0,
        children: Vec::new(),
    };

    let mut dir_list = HashMap::from([("/".to_string(), root)]);
    let mut current = "/".to_string();
    let mut previous: Vec<String> = Vec::new();

    // Iterate through the lines, but skip the first because it's always "$ cd /"
    for line in split_into_lines(input_lines).iter().skip(1) {
        if line == "$ls" {
            continue;
        } else if !line.starts_with('$') {
            // Must be listing stuff, so add it to the current point in the tree
            let (first, name) = line.split_once(' ').unwrap();
            let size = match first {
                "dir" => 0,
                _ => first.trim().parse().unwrap()
            };
            let current_dir = dir_list.get_mut(&current).unwrap();
            let full_path = format!("{}/{}", current, name);

            if !current_dir.children.contains(&full_path) {
                current_dir.size += size;
                current_dir.children.push(full_path.clone());

                for prev in previous.iter() {
                    dir_list.get_mut(prev).unwrap().size += size;
                }
                if first == "dir" {
                    dir_list.insert(full_path, DirectoryNode { size: size, children: Vec::new() });
                }
            }

        } else if line.starts_with("$ cd") {
            let (_, target) = line.split_at(5);
            if target == ".." {
                let (new, _) = current.rsplit_once("/").unwrap();
                current = new.to_string();
                previous.pop();
            } else {
                previous.push(current.clone());
                current = format!("{}/{}", current, target);
            }
        }
    }

    // Debug: print it out
    for (key, val) in dir_list.iter() {
        println!("{}, size {}", key, val.size);
        // println!("children: {:?}", val.children);
    }

    let answer1: i64 = dir_list.iter().filter(|(_, v)| {
        v.size <= 100000 && !v.children.is_empty()
    }).map(|(_, v)| v.size).sum();

    let total: i64 = 70000000;
    let target: i64 = 30000000;
    let starting = dir_list["/"].size;
    let gap = target - (total - starting);

    println!("/ is {} so need {}", starting, gap);

    let answer2 = dir_list.iter().map(|(_, v)| v.size).filter(|x| *x > gap).min().unwrap();

    (format!("{}", answer1), format!("{}", answer2))
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    #[test]
    fn check_day07_part1_case1() {
        assert_eq!(day07(TEST_INPUT).0, "95437".to_string())
    }

    #[test]
    fn check_day07_part2_case1() {
        assert_eq!(day07(TEST_INPUT).1, "24933642".to_string())
    }

    #[test]
    fn check_day07_both_case1() {
        assert_eq!(day07(TEST_INPUT), ("95437".to_string(), "24933642".to_string()))
    }
}
