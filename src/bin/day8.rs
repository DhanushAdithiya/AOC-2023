use std::collections::HashMap;

const TEST_INPUT: &str = r#"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)"#;

const TEST_INPUT2: &str = r#"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)"#;

const INPUT: &str = include_str!("../../inputs/day8.txt");

fn l_mod(x: u32, len: u32) -> usize {
    return (x % len) as usize;
}

fn part1(input: &str) -> u32 {
    let mut steps = 0;
    let directions: Vec<char> = input.lines().nth(0).unwrap().chars().collect();

    let mut map = HashMap::new();

    for route in input.lines().skip(2) {
        route.split_once("=").map(|(a, b)| {
            let (left, right) = b[2..b.len() - 1].split_once(",").unwrap();
            map.insert(a.trim(), (left.trim(), right.trim()));
        });
    }

    let mut node = "AAA";
    let mut found = false;
    while !found {
        let dir = directions[l_mod(steps, directions.len() as u32)];

        match dir {
            'L' => {
                let next = map.get(node).unwrap().0;
                if next == "ZZZ" {
                    found = true;
                    break;
                } else {
                    node = next;
                }
            }
            'R' => {
                let next = map.get(node).unwrap().1;
                if next == "ZZZ" {
                    found = true;
                    break;
                } else {
                    node = next;
                }
            }
            _ => {
                panic!("Direction is not recognized");
            }
        }
        steps += 1;
    }

    steps + 1 as u32
}

fn part2(input: &str) -> usize {
    let mut steps = 0;
    let directions: Vec<char> = input.lines().nth(0).unwrap().chars().collect();

    let mut map = HashMap::new();
    let mut node = Vec::new();

    for route in input.lines().skip(2) {
        route.split_once("=").map(|(a, b)| {
            let (left, right) = b[2..b.len() - 1].split_once(",").unwrap();
            map.insert(a.trim(), (left.trim(), right.trim()));
            if a.trim().ends_with("A") {
                node.push(a.trim());
            }
        });
    }

    let mut found = false;
    while !found {
        let mut next = Vec::new();
        let dir = directions[l_mod(steps, directions.len() as u32)];

        println!("Looking at vecs {node:?} at step {steps}");

        match dir {
            'L' => {
                for route in &node {
                    next.push(map.get(route).unwrap().0)
                }

                let ends = next.iter().all(|word| word.ends_with("Z"));
                if ends {
                    found = true;
                    break;
                }

                node.clear();
                node = next;
            }
            'R' => {
                for route in &node {
                    next.push(map.get(route).unwrap().1)
                }

                let ends = next.iter().all(|word| word.ends_with("Z"));
                if ends {
                    found = true;
                    break;
                }

                node.clear();
                node = next;
            }
            _ => {
                panic!("Direction is not recognized");
            }
        }
        steps += 1;
    }

    (steps + 1) as usize
}

fn main() {
    //println!("PART - 1: {}", part1(INPUT));
    println!("PART - 2: {}", part2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = part1(TEST_INPUT);
        assert_eq!(result, 6)
    }

    #[test]
    fn test_part2() {
        let result = part2(TEST_INPUT2);
        assert_eq!(result, 6)
    }
}
