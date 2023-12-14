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

fn gcd(first: usize, second: usize) -> usize {
    if second == 0 {
        return first;
    } else {
        return gcd(second, first % second);
    }
}

fn lcm(first: usize, second: usize) -> usize {
    first * second / gcd(first, second)
}

fn lcm_list(input: &[usize]) -> usize {
    let mut iter = input.iter();
    let first = iter.next().unwrap();
    let second = iter.next().unwrap();

    let mut ans = lcm(*first, *second);
    while let Some(next) = iter.next() {
        ans = lcm(ans, *next);
    }

    return ans;
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

    let mut answer = Vec::new();

    for entry in node {
        let mut current = entry;
        let mut c = 0;

        while !current.ends_with("Z") {
            let dir = directions[l_mod(c, directions.len() as u32)];

            match dir {
                'L' => {
                    current = map.get(current).unwrap().0;
                }
                'R' => {
                    current = map.get(current).unwrap().1;
                }
                _ => {
                    panic!("Direction is not recognized");
                }
            }
            c += 1;
        }

        answer.push(c as usize);
    }

    return lcm_list(&answer);
}

fn main() {
    println!("PART - 1: {}", part1(INPUT));
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
