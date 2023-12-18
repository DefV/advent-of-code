fn main() {
    let now = std::time::Instant::now();
    // Open file passed in ARGV
    let args: Vec<String> = std::env::args().collect();
    // Print usage if no file is passed
    if args.len() < 2 {
        println!("Usage: {} <filename>", args[0]);
        return;
    }

    let filename = &args[1];
    let document =
        std::fs::read_to_string(filename).expect("Something went wrong reading the file");

    part1(&document);
    part2(&document);

    println!("Runtime: {:?}", now.elapsed());
}

#[derive(Debug, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, PartialEq)]
struct Move {
    direction: Direction,
    distance: i64,
}

impl From<Hex> for Move {
    fn from(hex: Hex) -> Self {
        let distance = i64::from_str_radix(&hex.0[1..=5], 16).unwrap();
        let direction = hex.0.chars().last().unwrap();

        let direction = match direction {
            '0' => Direction::Right,
            '1' => Direction::Down,
            '2' => Direction::Left,
            '3' => Direction::Up,
            _ => panic!("Invalid direction: {}", direction),
        };

        Self {
            direction,
            distance: distance,
        }
    }
}

impl From<&str> for Move {
    fn from(s: &str) -> Self {
        let direction = s.chars().next().unwrap();
        let distance = s[2..].parse::<i64>().unwrap();

        let direction = match direction {
            'U' => Direction::Up,
            'D' => Direction::Down,
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!("Invalid direction"),
        };

        Self {
            direction,
            distance,
        }
    }
}

struct Hex(String);

type Point = (i64, i64);
struct Lagoon {
    circumference: i64,
    points: Vec<Point>,
}

impl From<Vec<Move>> for Lagoon {
    fn from(steps: Vec<Move>) -> Self {
        let mut points = Vec::new();
        let mut circumference = 0;

        points.push((0, 0));

        for mv in steps {
            let last_point = points.last().unwrap();
            let distance = mv.distance;

            circumference += distance;

            match mv.direction {
                Direction::Up => points.push((last_point.0 - distance, last_point.1)),
                Direction::Down => points.push((last_point.0 + distance, last_point.1)),
                Direction::Left => points.push((last_point.0, last_point.1 - distance)),
                Direction::Right => points.push((last_point.0, last_point.1 + distance)),
            }
        }

        assert_eq!(points.first(), points.last(), "Point area should be closed");

        Self {
            circumference,
            points,
        }
    }
}

impl Lagoon {
    fn area(&self) -> i64 {
        let len = self.points.len();
        let mut area = 0;

        for i in 0..len {
            let p1 = self.points[i];
            let p2 = self.points[(i + 1) % len];

            area += (p1.0 * p2.1) - (p1.1 * p2.0);
        }

        area.abs() / 2
    }
}

fn part1(document: &String) {
    let moves: Vec<Move> = document
        .lines()
        .map(|line| Move::from(line.rsplit_once(" ").unwrap().0))
        .collect();
    let lagoon = Lagoon::from(moves);
    println!(
        "Lagoon area: {}",
        lagoon.area() + lagoon.circumference as i64 / 2 + 1
    );
}

fn part2(document: &String) {
    let moves: Vec<Move> = document
        .lines()
        .map(|line| {
            let hex = &line.split_once("(").unwrap().1[0..=6];
            Hex(hex.to_string())
        })
        .map(|move_str| Move::from(move_str))
        .collect();
    let lagoon = Lagoon::from(moves);
    println!(
        "Lagoon area: {}",
        lagoon.area() + lagoon.circumference as i64 / 2 + 1
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move_from() {
        assert_eq!(
            Move::from("U 1"),
            Move {
                direction: Direction::Up,
                distance: 1
            }
        );
        assert_eq!(
            Move::from("D 1"),
            Move {
                direction: Direction::Down,
                distance: 1
            }
        );
        assert_eq!(
            Move::from("L 1"),
            Move {
                direction: Direction::Left,
                distance: 1
            }
        );
        assert_eq!(
            Move::from("R 1"),
            Move {
                direction: Direction::Right,
                distance: 1
            }
        );
    }

    #[test]
    fn test_move_from_hex() {
        assert_eq!(
            Move::from(Hex("#70c710".to_string())),
            Move {
                direction: Direction::Right,
                distance: 461937
            }
        )
    }
}
