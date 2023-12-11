use std::collections::HashMap;
use std::slice::Iter;
use std::time::Instant;

#[derive(Debug, PartialEq, Copy, Clone)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn opposite(&self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
        }
    }

    fn iter() -> Iter<'static, Direction> {
        static DIRECTIONS: [Direction; 4] = [
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
        ];

        DIRECTIONS.iter()
    }
}

type Map = Vec<Vec<Piece>>;

#[derive(Debug)]
struct Maze {
    map: Map,
    animal: (usize, usize),
    area_size: usize,
    path_size: usize
}

#[derive(Debug)]
struct Piece {
    directions: Vec<Direction>,
    is_animal: bool
}

impl Piece {
    fn empty() -> Self {
        Self {
            directions: vec![],
            is_animal: false,
        }
    }

    fn from_char(c: char) -> Self {
        match c {
            '|' => Self {
                directions: vec![Direction::North, Direction::South],
                ..Self::empty()
            },
            '-' => Self {
                directions: vec![Direction::East, Direction::West],
                ..Self::empty()
            },
            'L' => Self {
                directions: vec![Direction::North, Direction::East],
                ..Self::empty()
            },
            'J' => Self {
                directions: vec![Direction::North, Direction::West],
                ..Self::empty()
            },
            '7' => Self {
                directions: vec![Direction::South, Direction::West],
                ..Self::empty()
            },
            'F' => Self {
                directions: vec![Direction::South, Direction::East],
                ..Self::empty()
            },
            'S' => Self {
                is_animal: true,
                ..Self::empty()
            },
            _ => Self::empty(),
        }
    }
}

impl Maze {
    fn from_str(doc: &str) -> Self {
        let mut map = vec![];
        let mut animal = (0, 0);

        for (x, line) in doc.lines().enumerate() {
            let mut row = Vec::new();
            for (y, c) in line.chars().enumerate() {
                let piece = Piece::from_char(c);

                if piece.is_animal {
                    animal = (x, y);
                }

                row.push(piece);
            }
            map.push(row);
        }

        Self { map, animal, area_size: 0, path_size: 0 }
    }

    fn at(&self, (x, y): (usize, usize)) -> Option<&Piece> {
        self.map.get(x).and_then(|row| row.get(y))
    }

    fn go(&self, (x, y): (usize, usize), direction: Direction) -> Option<(usize, usize)> {
        let pos = match direction {
            Direction::North => (x.checked_sub(1)?, y),
            Direction::East => (x, y + 1),
            Direction::South => (x + 1, y),
            Direction::West => (x, y.checked_sub(1)?),
        };

        if pos.0 < self.map.len() && pos.1 < self.map[0].len() {
            Some(pos)
        } else {
            None
        }
    }

    fn build_path(&mut self) {
        let mut path = HashMap::new();
        // Check pieces around the animal
        let animal_directions: Vec<Direction> = Direction::iter()
            .filter_map(|direction| {
                let pos = self.go(self.animal, *direction)?;
                let piece = self.at(pos)?;

                if piece.directions.contains(&direction.opposite()) {
                    Some(*direction)
                } else {
                    None
                }
            })
            .collect();

        self.map[self.animal.0][self.animal.1].directions = animal_directions.clone();

        // Start the path
        let mut direction = animal_directions[0];
        let mut position = self.go(self.animal, direction).unwrap();
        let mut steps = 1;

        while position != self.animal {
            let piece = self.at(position).unwrap();
            let next_direction = piece
                .directions
                .iter()
                .find(|dir| **dir != direction.opposite())
                .unwrap();

            path.insert(position, piece);
            direction = *next_direction;
            position = self.go(position, direction).unwrap();
            steps += 1;
        }

        self.area_size = self.calculate_area_size(&path);
        self.path_size = steps;
    }
    
    fn calculate_area_size(&self, path: &HashMap<(usize, usize), &Piece>) -> usize {
        let mut area_size = 0;
        let mut in_area = false;

        for (x, row) in self.map.iter().enumerate() {
            for (y, _) in row.iter().enumerate() {
                if let Some(piece) = path.get(&(x, y)) {
                    if piece.directions.contains(&Direction::East) {
                        in_area = true;
                    } else {
                        in_area = !in_area;
                    }
                    if in_area {
                        print!("X");
                    } else {
                        print!("Y");
                    }
                } else {
                    if in_area {
                        print!("I");
                        area_size += 1;
                    } else {
                        print!("0");
                    }
                }

            }

            println!();
            in_area = false;
        }

        area_size
    }
}

fn main() {
    let now = Instant::now();
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

    let mut maze = Maze::from_str(&document);
    maze.build_path();

    println!("Path length: {}, farthest away: {}", maze.path_size, maze.path_size / 2);
    println!("Area size: {}", maze.area_size);
    println!("Runtime: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    static MAZE: &str = "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";

    #[test]
    fn test_parse_maze() {
        let maze = Maze::from_str(MAZE);

        assert_eq!(maze.map.len(), 5);
        assert_eq!(maze.map[0].len(), 5);

        assert_eq!(maze.animal, (2, 0));
    }
}
