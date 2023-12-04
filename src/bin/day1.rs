use std::collections::BTreeMap;

const INPUT: &str = include_str!("../../inputs/day1.txt");
const TEST_INPUT2: &str = r#"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"#;

const TEST_INPUT: &str = r#"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"#;

fn part1(input: &str) -> u32 {
    let mut sum = 0;
    for line in input.trim().split("\n") {
        let mut numbers = Vec::new();
        for c in line.chars() {
            if c.is_numeric() {
                numbers.push(c.to_digit(10).unwrap());
            }
        }

        sum += (numbers[0] * 10) + numbers[numbers.len() - 1];
    }

    sum
}

fn part2(input: &str) -> u32 {
    let lookup_table = BTreeMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);

    let mut sum = 0;

    for line in input.split("\n") {
        let mut new_string = String::from(line);
        for (key, val) in &lookup_table {
            new_string = new_string.replace(key, &format!("{key}{val}{key}"))
        }
        println!("{line}");
        println!("{new_string}");

        let mut numbers = Vec::new();
        for n in new_string.chars() {
            if n.is_numeric() {
                numbers.push(n.to_digit(10).unwrap())
            }
        }
        sum += (numbers[0] * 10) + numbers[numbers.len() - 1];
    }
    sum
}

fn main() {
    println!("PART 1:{}", part1(INPUT));
    println!("PART 2:{}", part2(TEST_INPUT2));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = part1(TEST_INPUT);
        assert_eq!(result, 142);
    }

    #[test]
    fn test_part2() {
        let result = part2(TEST_INPUT2);
        assert_eq!(result, 281)
    }
}
