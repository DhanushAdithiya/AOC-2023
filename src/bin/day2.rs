const INPUT: &str = include_str!("../../inputs/day2.txt");
const TEST_INPUT1: &str = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;
const TEST_Q: Balls = Balls {
    red: 12,
    green: 13,
    blue: 14,
};

struct Balls {
    red: i32,
    blue: i32,
    green: i32,
}

fn part1(input: &str, quantity: Balls) -> usize {
    let mut sum = 0;

    'outer_loop: for line in input.lines() {
        let space_index = line.find(" ").unwrap();
        let colon_index = line.find(":").unwrap();
        let game_id = &line[space_index + 1..colon_index].parse::<usize>().unwrap();

        for game in line[colon_index + 1..].split(';') {
            let mut green = 0;
            let mut red = 0;
            let mut blue = 0;
            for draw in game.split(",") {
                let (count, color) = draw[1..].split_once(" ").unwrap();
                let c = count.parse::<i32>().unwrap();
                match color {
                    "blue" => blue += c,
                    "green" => green += c,
                    "red" => red += c,
                    _ => eprintln!("error"),
                }

                if blue <= quantity.blue && red <= quantity.red && green <= quantity.green {
                } else {
                    continue 'outer_loop;
                }
            }
        }
        sum += game_id
    }

    sum
}

fn part2(input: &str) -> i32 {
    let mut sum = 0;

    for line in input.lines() {
        let colon_index = line.find(":").unwrap();
        let mut green = 0;
        let mut red = 0;
        let mut blue = 0;
        for game in line[colon_index + 1..].split(';') {
            for draw in game.split(",") {
                let (count, color) = draw[1..].split_once(" ").unwrap();
                let c = count.parse::<i32>().unwrap();
                match color {
                    "blue" => {
                        if c > blue {
                            blue = c;
                        }
                    }
                    "green" => {
                        if c > green {
                            green = c;
                        }
                    }
                    "red" => {
                        if c > red {
                            red = c;
                        }
                    }
                    _ => eprintln!("error"),
                }
            }
        }
        sum += red * green * blue;
    }

    sum
}

fn main() {
    println!("PART 1 - {}", part1(INPUT, TEST_Q));
    println!("PART 2 - {}", part2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = part1(TEST_INPUT1, TEST_Q);
        assert_eq!(result, 8)
    }
    #[test]
    fn test2() {
        let result = part2(TEST_INPUT1);
        assert_eq!(result, 2286)
    }
}
