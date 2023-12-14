const INPUT: &str = include_str!("../../inputs/day9.txt");
const TEST_INPUT: &str = r#"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"#;

fn part1(input: &str) -> i64 {
    let mut sum = 0;
    for line in input.lines() {
        let mut last_values = Vec::new();
        let mut sequence: Vec<i64> = line
            .split_whitespace()
            .map(|a| a.parse::<i64>().unwrap())
            .collect();

        while !sequence.iter().all(|a| *a == 0) {
            let mut iteration = Vec::new();

            for i in 0..sequence.len() - 1 {
                let current = sequence[i];
                let next = sequence[i + 1];

                let new_val = next - current;
                iteration.push(new_val);
            }

            last_values.push(sequence.last().unwrap().clone());
            sequence.clear();
            iteration.iter().for_each(|a| sequence.push(*a));
        }

        sum += last_values.iter().sum::<i64>();
    }

    sum
}

fn part2(input: &str) -> i64 {
    let mut sum = 0;
    for line in input.lines() {
        let mut last_values = Vec::new();
        let mut sequence: Vec<i64> = line
            .split_whitespace()
            .map(|a| a.parse::<i64>().unwrap())
            .collect();
        sequence.reverse();

        while !sequence.iter().all(|a| *a == 0) {
            let mut iteration = Vec::new();

            for i in 0..sequence.len() - 1 {
                let current = sequence[i];
                let next = sequence[i + 1];

                let new_val = next - current;
                iteration.push(new_val);
            }

            last_values.push(sequence.last().unwrap().clone());
            sequence.clear();
            iteration.iter().for_each(|a| sequence.push(*a));
        }

        sum += last_values.iter().sum::<i64>();
    }

    sum
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
        assert_eq!(result, 114);
    }

    #[test]
    fn test_part2() {
        let result = part2(TEST_INPUT);
        assert_eq!(result, 2);
    }
}
