const INPUT: &str = include_str!("../../inputs/day6.txt");

const TEST_INPUT: &str = r#"Time:      7  15   30
Distance:  9  40  200"#;

fn part1(input: &str) -> usize {
    let mut sum = 1;
    let time: Vec<usize> = input
        .lines()
        .nth(0)
        .unwrap()
        .split_once(":")
        .unwrap()
        .1
        .split_whitespace()
        .map(|a| a.parse::<usize>().unwrap())
        .collect();

    let distance: Vec<usize> = input
        .lines()
        .nth(1)
        .unwrap()
        .split_once(":")
        .unwrap()
        .1
        .split_whitespace()
        .map(|a| a.parse::<usize>().unwrap())
        .collect();

    for i in 0..time.len() {
        let mut f = 0;
        let mut l = 0;

        for x in 1..time[i] {
            let remaining_time = time[i] - x;
            let distance_covered = x * remaining_time;

            if distance_covered > distance[i] {
                f = x;
                break;
            }
        }

        for y in (f..time[i]).rev() {
            let remaining_time = time[i] - y;
            let distance_covered = y * remaining_time;

            if distance_covered > distance[i] {
                l = y;
                break;
            }
        }

        let diff = (l as i32 - f as i32) + 1;
        sum *= diff;
    }

    sum as usize
}

fn part2(input: &str) -> usize {
    let mut sum = 0;
    let str_time: String = input
        .lines()
        .nth(0)
        .unwrap()
        .split_once(":")
        .unwrap()
        .1
        .split_whitespace()
        .collect();

    let str_distance: String = input
        .lines()
        .nth(1)
        .unwrap()
        .split_once(":")
        .unwrap()
        .1
        .split_whitespace()
        .collect();

    let time = str_time.parse::<usize>().unwrap();
    let distance = str_distance.parse::<usize>().unwrap();

    let mut f = 0;
    let mut l = 0;

    for i in 1..time {
        let remaining = time - i;
        let distance_covered = i * remaining;

        if distance_covered > distance {
            f = i;
            break;
        }
    }

    for j in (f..time).rev() {
        let remaining = time - j;
        let distance_covered = j * remaining;
        if distance_covered > distance {
            l = j;
            break;
        }
    }
    sum += l as i32 - f as i32 + 1;

    sum as usize
}

fn main() {
    println!("PART 1 - {}", part1(INPUT));
    println!("PART 2 - {}", part2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = part1(TEST_INPUT);
        assert_eq!(result, 288);
    }

    #[test]
    fn test_part2() {
        let result = part2(TEST_INPUT);
        assert_eq!(result, 71503);
    }
}
