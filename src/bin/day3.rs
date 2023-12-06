use std::collections::HashSet;

const INPUT:&str = include_str!("../../inputs/day3.txt");
const TEST_INPUT1: &str = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;

const DIRECTIONS: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0,1),
    (1, -1),
    (1, 0),
    (1,1)
];

struct Grid {
    grid: Vec<Vec<char>>,
    height: usize,
    width: usize,
}


fn is_symbol(symbol: char) -> bool {
    if !symbol.is_numeric() && symbol != '.' {
        return true
    }
    false
}

fn is_star(symbol: char) -> bool {
    if symbol == '*' {
        return true;
    }
    false
}

fn parse_grid(input: &str) -> Grid {
    let g: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let height = g.len();
    let width = g[0].len();


    let grid: Grid = Grid {
        grid: g,
        height,
        width,
    };

    grid
} 

fn is_adjacent(g: &Grid, row: usize, col: usize) -> bool {
    for (dy, dx) in DIRECTIONS {
        let r = row as i32 + dy;
        let c = col as i32 + dx;

        if (0 <= r && r < g.height as i32) && (0 <= c && c < g.width as i32)  {
            if is_symbol(g.grid[r as usize][c as usize]) {
                return true;
            }
        }
    }

    return false
}


fn find_gear_ratio(g: &Grid, row: usize, col: usize) -> usize {
    let mut adjacent: Vec<usize> = vec![];
    for (dy,dx) in DIRECTIONS {
        let r = row as i32 + dy;
        let c = col as i32 + dx;

        if (0 <= r && r < g.height as i32) && (0 <= c && c < g.width as i32)  {
            if g.grid[r as usize][c as usize].is_numeric() {
                let p_number = get_number(&g, r as usize, c as usize);
                if !adjacent.contains(&p_number) {
                    adjacent.push(p_number)
                }
            }
        }
    }
    if adjacent.len() == 2 {
        return adjacent[0] * adjacent[1];
    } 
    0
}

fn get_number(g: &Grid, row: usize, col: usize) -> usize {
    let mut number = String::from(g.grid[row][col]);

    let mut left_col = (col - 1) as i32;
    while left_col >= 0 && g.grid[row][left_col as usize].is_numeric() {
        number.insert(0, g.grid[row][left_col as usize]);
        left_col -= 1;
    }

    let mut right_col = col + 1;
    while right_col < g.width && g.grid[row][right_col].is_numeric() {
        number.push(g.grid[row][right_col]);
        right_col += 1;
    }

    return number.parse::<usize>().unwrap();
}



fn part1(input: &str) -> usize {
    let mut sum = 0;
    let g = parse_grid(input);
    let mut processed: HashSet<(usize, usize)> = HashSet::new();

    for (i,row) in g.grid.iter().enumerate() {
        let mut j = 0;
        while j < g.width {
            if row[j].is_numeric() && !processed.contains(&(i,j)) {
                let mut number = String::from(row[j]);
                let init_j = j;

                while ((j + 1) < g.width) && (row[j + 1].is_numeric()) {
                    number.push(row[j + 1]);
                    j += 1
                }

                for col_index in init_j..j+1 {
                    processed.insert((i, col_index));
                    if is_adjacent(&g, i, col_index) == true {
                        sum += number.trim().parse::<usize>().unwrap();
                        break
                    }
                }
            }
            j += 1
        }
    } 
    sum as usize
}

fn part2(input: &str) -> usize {
    let mut sum = 0;
    let g = parse_grid(input);

    for (i, row) in g.grid.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            if is_star(g.grid[i][j]) == true {
                sum += find_gear_ratio(&g, i, j);
            }
        }
    }

    sum
}

fn main() {
    println!("PART 1 -  {}", part1(INPUT));
    println!("PART 2 -  {}", part2(INPUT));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = part1(TEST_INPUT1);
        assert_eq!(result, 4361)
    }

    #[test]
    fn test2() {
        let result = part2(TEST_INPUT1);
        assert_eq!(result, 467835)
    }
}
