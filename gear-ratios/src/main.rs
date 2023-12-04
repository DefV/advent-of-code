use std::{cmp, collections::HashMap};

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

#[derive(PartialEq, Eq, Hash,Debug)]
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

    let mut products: Vec<i32> = vec![];
    for (x, line) in grid.iter().enumerate() {
        for (y, char) in line.iter().enumerate() {
            if *char == '*' {
                let position = Position(x, y);
                let surrounding_numbers = surrounding_numbers(&grid, &position);
                if surrounding_numbers.len() == 2 {
                    products.push(surrounding_numbers[0] * surrounding_numbers[1])
                }
            }
        }
    }

    return products;
}

fn surrounding_numbers(grid: &Vec<Vec<char>>, position: &Position) -> Vec<i32> {
    let left_top = position.relative_move(-1, -1);
    let right_bottom: Position = position.relative_move(1, 1);

    let mut numbers: HashMap<Position,i32> = HashMap::new();

    for x in left_top.0..=right_bottom.0 {
        for y in left_top.1..=right_bottom.1 {
            let nr = grid[x][y];
            if nr.is_digit(10) {
                let (pos, number) = find_number_at_position(&grid, &Position(x, y));
                numbers.insert(pos, number);
            }
        }
    }

    return numbers.values().map(|x| *x).collect();
}

fn find_number_at_position(grid: &Vec<Vec<char>>, position: &Position) -> (Position, i32) {
    let row = grid[position.0].clone();
    let mut y = position.1;
    let mut number = vec![];


    // Find the right bound
    while y < row.len() && row[y].is_digit(10) {
        y += 1;
    }
    while y > 0 && row[y - 1].is_digit(10) {
        y -= 1;
        number.push(row[y]);
    }

    return (Position(position.0, y), number.iter().rev().collect::<String>().parse::<i32>().unwrap());
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
        assert_eq!(result, vec![16345, 451490])
    }
}
