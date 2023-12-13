use std::collections::BTreeMap;

const INPUT: &str = include_str!("../../inputs/day7.txt");
const TEST_INPUT: &str = r#"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"#;

#[derive(Debug, Clone, Copy)]
enum HandType {
    FiveOfAKind = 6,
    FourOfAKind = 5,
    FullHouse = 4,
    ThreeOfAKind = 3,
    TwoPair = 2,
    OnePair = 1,
    HighCard = 0,
}

#[derive(Debug)]
struct HandOrder {
    h_type: HandType,
    card: String,
}

fn find_type(input: &str) -> HandType {
    use HandType::*;

    let mut map = BTreeMap::new();
    for i in input.chars() {
        let n = input.matches(i).count();
        map.insert(i, n as i32);
    }

    let mut s = map.into_values().collect::<Vec<i32>>();
    s.sort();
    s.reverse();
    let x: String = s.iter().map(|a| a.to_string()).collect();
    match x.as_str() {
        "5" => FiveOfAKind,
        "41" => FourOfAKind,
        "32" => FullHouse,
        "311" => ThreeOfAKind,
        "221" => TwoPair,
        "2111" => OnePair,
        "11111" => HighCard,
        _ => panic!("IMPOSSIBLE"),
    }
}

fn find_type2(input: &str) -> HandType {
    use HandType::*;

    let mut map = BTreeMap::new();
    let mut new_map = BTreeMap::new();
    for i in input.chars() {
        let n = input.matches(i).count();
        map.insert(i, n as i32);
    }

    if let Some(v) = map.get(&'J') {
        if v < &5 {
            map.remove(&'J');
        }
    }

    let (cm, _) = map.iter().max_by_key(|&(_, value)| value).unwrap();

    let new_i = input.replace('J', &cm.to_string()[..]);

    for i in new_i.chars() {
        let n = new_i.matches(i).count();
        new_map.insert(i, n as i32);
    }

    let mut s = new_map.into_values().collect::<Vec<i32>>();
    s.sort();
    s.reverse();

    let x: String = s.iter().map(|a| a.to_string()).collect();

    match x.as_str() {
        "5" => FiveOfAKind,
        "41" => FourOfAKind,
        "32" => FullHouse,
        "311" => ThreeOfAKind,
        "221" => TwoPair,
        "2111" => OnePair,
        "11111" => HighCard,
        _ => panic!("IMPOSSIBLE"),
    }
}

fn parse_input(input: &str) -> Vec<(&str, u32)> {
    let parsed = input
        .lines()
        .map(|line| {
            let (card, bid) = line.split_once(" ").unwrap();
            (card, bid.parse::<u32>().unwrap())
        })
        .collect();

    parsed
}

fn convert(input: char) -> u8 {
    if input.is_alphabetic() {
        match input {
            'A' => return 14,
            'K' => return 13,
            'Q' => return 12,
            'J' => return 11,
            'T' => return 10,
            _ => panic!("Wrong value"),
        }
    } else {
        return input.to_digit(10).unwrap() as u8;
    }
}

fn convert2(input: char) -> u8 {
    if input.is_alphabetic() {
        match input {
            'A' => return 13,
            'K' => return 12,
            'Q' => return 11,
            'T' => return 10,
            'J' => return 1,
            _ => panic!("Wrong value"),
        }
    } else {
        return input.to_digit(10).unwrap() as u8;
    }
}

fn convert_values(input: &str) -> HandOrder {
    let h_type = find_type(input);

    HandOrder {
        h_type,
        card: String::from(input),
    }
}

fn convert_values2(input: &str) -> HandOrder {
    let h_type = find_type2(input);

    HandOrder {
        h_type,
        card: String::from(input),
    }
}

fn part1(input: &str) -> usize {
    let mut sum: usize = 0;
    let parsed = parse_input(input);

    let mut ordering = Vec::new();

    for (card, bid) in parsed {
        let val = convert_values(card);
        ordering.push((bid, val));
    }
    ordering.sort_by(|a, b| {
        // First, compare by h_type
        let type_comparison = (a.1.h_type as u8).cmp(&(b.1.h_type as u8));

        if type_comparison != std::cmp::Ordering::Equal {
            return type_comparison;
        } else {
            for (i, (card_a, card_b)) in a.1.card.chars().zip(b.1.card.chars()).enumerate() {
                if convert(card_a) > convert(card_b) {
                    return std::cmp::Ordering::Greater;
                } else if convert(card_a) < convert(card_b) {
                    return std::cmp::Ordering::Less;
                }
                // If cards are equal so far, move to the next card position
                if i == 4 {
                    break; // All cards are equal, break the loop
                }
            }
            panic!("a")
        }
    });

    for (i, val) in ordering.iter().enumerate() {
        sum += (i + 1) as usize * val.0 as usize;
    }
    sum
}

fn part2(input: &str) -> usize {
    let mut sum: usize = 0;
    let parsed = parse_input(input);

    let mut ordering = Vec::new();

    for (card, bid) in parsed {
        let val = convert_values2(card);
        ordering.push((bid, val));
    }

    ordering.sort_by(|a, b| {
        // First, compare by h_type
        let type_comparison = (a.1.h_type as u8).cmp(&(b.1.h_type as u8));

        if type_comparison != std::cmp::Ordering::Equal {
            return type_comparison;
        } else {
            for (i, (card_a, card_b)) in a.1.card.chars().zip(b.1.card.chars()).enumerate() {
                if convert2(card_a) > convert2(card_b) {
                    return std::cmp::Ordering::Greater;
                } else if convert2(card_a) < convert2(card_b) {
                    return std::cmp::Ordering::Less;
                }
                // If cards are equal so far, move to the next card position
                if i == 4 {
                    break; // All cards are equal, break the loop
                }
            }
            panic!("a")
        }
    });

    for (i, val) in ordering.iter().enumerate() {
        sum += (i + 1) as usize * val.0 as usize;
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
        assert_eq!(result, 6440);
    }

    #[test]
    fn test_part2() {
        let result = part2(TEST_INPUT);
        assert_eq!(result, 5905);
    }
}
