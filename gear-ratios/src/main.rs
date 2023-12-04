use std::cmp;

fn main() {
    // Open file passed in ARGV
    let args: Vec<String> = std::env::args().collect();
    // Print usage if no file is passed
    if args.len() < 2 {
        println!("Usage: trebuchet <filename>");
        return;
    }

    let filename = &args[1];
    let document =
        std::fs::read_to_string(filename).expect("Something went wrong reading the file");

    let parts = parse_document(&document);
    let sum = parts.iter().sum::<i32>();

    println!("Parts: {:?}", parts);
    println!("Sum: {}", sum);
}

#[derive(Debug)]
struct Position(usize, usize);

impl Position {
    fn relative_move(&self, x: i16, y: i16) -> Position {
        let new_x = cmp::max(cmp::min(self.0 as i16 + x, 139), 0);
        let new_y = cmp::max(cmp::min(self.1 as i16 + y, 139), 0);

        return Position(new_x as usize, new_y as usize);
    }
}

fn parse_document(document: &str) -> Vec<i32> {
    let grid: Vec<Vec<char>> = document
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut good_parts: Vec<i32> = vec![];
    let mut current_part: Vec<char> = vec![];
    let mut last_position: Position = Position(0, 0);

    for (x, line) in grid.iter().enumerate() {
        for (y, char) in line.iter().enumerate() {
            match char {
                '0'..='9' => {
                    current_part.push(*char);
                    last_position = Position(x, y);
                }
                _ => {
                    if current_part.len() > 0 {
                        if check_surroundings(&grid, &current_part, &last_position) {
                            good_parts.push(
                                current_part
                                    .iter()
                                    .collect::<String>()
                                    .parse::<i32>()
                                    .unwrap(),
                            );
                        }

                        current_part = vec![];
                    }
                }
            }
        }

        if current_part.len() > 0 {
            if check_surroundings(&grid, &current_part, &last_position) {
                good_parts.push(
                    current_part
                        .iter()
                        .collect::<String>()
                        .parse::<i32>()
                        .unwrap(),
                );
            }

            current_part = vec![];
        }
    }

    return good_parts;
}

fn check_surroundings(grid: &Vec<Vec<char>>, part: &Vec<char>, position: &Position) -> bool {
    let left_top = position.relative_move(-1, 0 - part.len() as i16);
    let right_bottom: Position = position.relative_move(1, 1);

    for x in left_top.0..=right_bottom.0 {
        for y in left_top.1..=right_bottom.1 {
            match grid[x][y] {
                '0'..='9' => {}
                '.' => {}
                _ => return true,
            }
        }
    }

    return false;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_document() {
        let document = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

        let result = parse_document(&document);
        assert_eq!(result, vec![467, 35, 633, 617, 592, 755, 664, 598])
    }
}
