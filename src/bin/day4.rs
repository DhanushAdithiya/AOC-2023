use std::collections::HashMap;

const INPUT: &str = include_str!("../../inputs/day4.txt");
const TEST_INPUT: &str = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
"#;

fn part1(input: &str) -> i32 {
    let mut sum = 0;
    let base: i32 = 2;
    for line in input.lines() {
        let mut n: i32 = -1;
        let (_, cards) = line.split_once(":").unwrap();
        let (winners, collected) = cards.split_once("|").unwrap();
        let w: Vec<usize> = winners
            .split_whitespace()
            .map(|a| a.parse::<usize>().unwrap())
            .collect();
        let c: Vec<usize> = collected
            .split_whitespace()
            .map(|a| a.parse::<usize>().unwrap())
            .collect();
        for i in w {
            for j in &c {
                if i == *j {
                    n += 1;
                }
            }
        }
        if n >= 0 {
            sum += base.pow(n as u32);
        }
    }

    sum
}

fn part2(input: &str) -> i32 {
    let mut sum = 0;
    let mut cards_map: HashMap<String, usize> = HashMap::new();
    for line in input.lines() {
        let mut n: i32 = -1;
        let (id, cards) = line.split_once(":").unwrap();
        let (_, num) = id.split_once(" ").unwrap();
        cards_map.entry(num.trim().to_string()).or_insert(1);
        let (winners, collected) = cards.split_once("|").unwrap();
        let w: Vec<usize> = winners
            .split_whitespace()
            .map(|a| a.parse::<usize>().unwrap())
            .collect();
        let c: Vec<usize> = collected
            .split_whitespace()
            .map(|a| a.parse::<usize>().unwrap())
            .collect();
        for i in w {
            for j in &c {
                if i == *j {
                    n += 1;
                }
            }
        }

        let n_avail = cards_map.get(num.trim()).unwrap().clone();
        let new_cards = num.trim().parse::<i32>().unwrap();

        for i in new_cards + 1..new_cards + n + 2 {
            let s = i.to_string();
            let entry = cards_map.entry(s).or_insert(1);
            *entry += n_avail;
        }
    }
    for (_, val) in &cards_map {
        sum += val;
    }

    println!("{:#?}", cards_map.len());
    sum as i32
}

fn main() {
    println!("Part 1 - {}", part1(INPUT));
    println!("Part 2 - {}", part2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = part1(TEST_INPUT);
        assert_eq!(result, 13);
    }

    #[test]
    fn test_part2() {
        let result = part2(TEST_INPUT);
        assert_eq!(result, 30);
    }
}
