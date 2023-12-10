const INPUT: &str = include_str!("../../inputs/day5.txt");
const TEST_INPUT: &str = r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
"#;

fn part1(input: &str) -> usize {
    let seeds: Vec<usize> = input
        .lines()
        .nth(0)
        .unwrap()
        .split_once(":")
        .unwrap()
        .1
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect();
    let mut loc = Vec::new();
    for seed in seeds {
        let mut skip = 2;
        let mut s = seed;
        for (i, line) in input.lines().skip(skip).enumerate() {
            if let Some(cha) = line.chars().nth(0) {
                if cha.is_alphabetic() {
                    for (j, number_lines) in input.lines().skip(i + 3).enumerate() {
                        if let Some(num) = number_lines.chars().nth(0) {
                            if num.is_numeric() {
                                let map: Vec<usize> = number_lines
                                    .split_whitespace()
                                    .map(|a| a.parse::<usize>().unwrap())
                                    .collect();
                                let sr = map[1]..map[1] + map[2];
                                if sr.contains(&s) {
                                    let n = s - map[1];
                                    let dest = map[0] + n;
                                    s = dest;
                                    skip = i + j;
                                    break;
                                }
                            } else {
                                skip = i + j;
                                break;
                            }
                        }
                    }
                }
            }
        }
        loc.push(s);
    }
    return *loc.iter().min().unwrap();
}

fn part2(input: &str) -> f64 {
    let seeds: Vec<usize> = input
        .lines()
        .nth(0)
        .unwrap()
        .split_once(":")
        .unwrap()
        .1
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect();
    let mut loc = f64::INFINITY;
    let mut ranges = Vec::new();
    let mut i = 0;

    while i < seeds.len() {
        if i + 1 < seeds.len() {
            let range = seeds[i]..seeds[i] + seeds[i + 1];
            ranges.push(range);
            i += 2;
        } else {
            break; // Ensure there are pairs to form ranges
        }
    }
    for range in ranges {
        for seed in range.clone() {
            let mut skip = 2;
            let mut s = seed;
            for (i, line) in input.lines().skip(skip).enumerate() {
                if let Some(cha) = line.chars().nth(0) {
                    if cha.is_alphabetic() {
                        for (j, number_lines) in input.lines().skip(i + 3).enumerate() {
                            if let Some(num) = number_lines.chars().nth(0) {
                                if num.is_numeric() {
                                    let map: Vec<usize> = number_lines
                                        .split_whitespace()
                                        .map(|a| a.parse::<usize>().unwrap())
                                        .collect();
                                    let sr = map[1]..map[1] + map[2];
                                    if sr.contains(&s) {
                                        let n = s - map[1];
                                        let dest = map[0] + n;
                                        s = dest;
                                        skip = i + j;
                                        break;
                                    }
                                } else {
                                    skip = i + j;
                                    break;
                                }
                            }
                        }
                    }
                }
            }
            if (s as f64) < loc {
                loc = s as f64;
            }
        }
    }

    return loc;
}

fn main() {
    println!("Part 1 - {}", part1(INPUT));
    println!("PLEASE DON'T RUN PART 2 YET");
    println!("Part 2 - {}", part2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = part1(TEST_INPUT);
        assert_eq!(result, 35)
    }

    #[test]
    fn test_part2() {
        let result = part2(TEST_INPUT);
        assert_eq!(result, 46.00);
    }
}
