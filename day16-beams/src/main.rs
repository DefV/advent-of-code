use std::collections::{HashMap, HashSet};

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

    part1(document.to_string());
    part2(&document);

    println!("Runtime: {:?}", now.elapsed());
}

type Point = (usize, usize);
type Vector = (isize, isize);

struct Map {
    map: HashMap<Point, char>,
    visited_locations: HashSet<Point>,
    bounds: (usize, usize),
}

impl From<String> for Map {
    fn from(string: String) -> Self {
        let mut map = HashMap::with_capacity(string.len());
        let mut bounds = (0, 0);

        for (x, line) in string.lines().enumerate() {
            for (y, c) in line.chars().enumerate() {
                map.insert((x, y), c);
                bounds = (x, y); // Last point is the bottom right corner
            }
        }

        Map {
            map,
            bounds,
            visited_locations: HashSet::new(),
        }
    }
}

impl Map {
    fn resolve_location(&self, location: &Point, vector: &Vector) -> Option<Point> {
        let next_x = location.0.checked_add_signed(vector.0)?;
        let next_y = location.1.checked_add_signed(vector.1)?;

        if next_x > self.bounds.0 || next_y > self.bounds.1 {
            None
        } else {
            Some((next_x, next_y))
        }
    }

    fn traverse(&mut self) {
        let mut visit_queue: Vec<(Point, Vector)> = vec![];
        let mut loop_detector: HashSet<(Point, Vector)> = HashSet::new();
        visit_queue.push(((0, 0), (0, 1)));

        while let Some((location, vector)) = visit_queue.pop() {
            if !loop_detector.insert((location, vector)) {
                continue;
            }

            self.visited_locations.insert(location);

            let next_vectors = match self.map.get(&location) {
                Some('.') => vec![vector],
                Some('\\') => match vector {
                    (0, 1) => vec![(1, 0)],
                    (0, -1) => vec![(-1, 0)],
                    (1, 0) => vec![(0, 1)],
                    (-1, 0) => vec![(0, -1)],
                    _ => unreachable!(),
                },
                Some('/') => match vector {
                    (0, 1) => vec![(-1, 0)],
                    (0, -1) => vec![(1, 0)],
                    (1, 0) => vec![(0, -1)],
                    (-1, 0) => vec![(0, 1)],
                    _ => unreachable!(),
                },
                Some('-') => match vector {
                    (0, 1) | (0, -1) => vec![vector],
                    (1, 0) | (-1, 0) => vec![(0, 1), (0, -1)],
                    _ => unreachable!(),
                },
                Some('|') => match vector {
                    (1, 0) | (-1, 0) => vec![vector],
                    (0, 1) | (0, -1) => vec![(1, 0), (-1, 0)],
                    _ => unreachable!(),
                }

                _ => unreachable!(),
            };

            for next_vector in next_vectors {
                if let Some(next_location) = self.resolve_location(&location, &next_vector) {
                    visit_queue.push((next_location, next_vector));
                }
            }

        }
    }
}

fn part1(document: String) {
    let mut map = Map::from(document);
    map.traverse();
    println!("Part 1: {}", map.visited_locations.len());
}

fn part2(document: &str) {
    todo!()
}
