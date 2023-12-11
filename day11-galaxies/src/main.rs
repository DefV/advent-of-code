fn main() {
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

    let expanded_map = expand_map(&document);
    let galaxies = map_galaxies(&expanded_map);

    let universe = Universe {
        map: expanded_map,
        galaxies: galaxies,
    };

    let galaxy_pairs = combine_galaxies(&universe.galaxies);

    let galaxy_distance: usize = galaxy_pairs
        .into_iter()
        .map(|(g1, g2)| universe.distance_to(g1, g2))
        .sum();

    println!("{}", galaxy_distance);
}

const BLANK_SPACE: char = '.';
const GALAXY: char = '#';

// Part 1
// const EXPANDED_STEP_SIZE: usize = 2;
// Part 2 example
// const EXPANDED_STEP_SIZE: usize = 10;
// Part 2
const EXPANDED_STEP_SIZE: usize = 1_000_000;


#[derive(PartialEq, Debug,Copy, Clone)]
enum Space {
    Nothing,
    ExpandedNothing,
    Galaxy,
}

impl From<&char> for Space {
    fn from(c: &char) -> Space {
        match *c {
            GALAXY => Space::Galaxy,
            _ => Space::Nothing,
        }
    }
}

type Map = Vec<Vec<Space>>;

#[derive(Debug)]
struct Universe {
    map: Map,
    galaxies: Vec<Galaxy>,
}

impl Universe {
    fn distance_to(&self, a: &Galaxy, b: &Galaxy) -> usize {
        let mut position = a.position;
        let mut distance_traveled = 0;

        while position != b.position {
            let (x, y) = position;

            if x < b.position.0 {
                position.0 += 1;
            } else if x > b.position.0 {
                position.0 -= 1;
            } else if y < b.position.1 {
                position.1 += 1;
            } else if y > b.position.1 {
                position.1 -= 1;
            }

            let step_size = match self.map[position.0 as usize][position.1 as usize] {
                Space::ExpandedNothing => EXPANDED_STEP_SIZE,
                _ => 1
            };
            distance_traveled += step_size;
            // println!("Step to #{},{} found a {:?}", position.0, position.1, self.map[position.0 as usize][position.1 as usize]);
        }

        distance_traveled
    }
}

#[derive(Debug)]
struct Galaxy {
    position: (i32, i32),
}

fn is_blank_space(c: &char) -> bool {
    *c == BLANK_SPACE
}

fn expand_map(document: &str) -> Map {
    let mut expanded_map: Map = Vec::new();

    // Expand height
    let map: Vec<Vec<char>> = document
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    for i in 0..map.len() {
        if map[i].iter().all(is_blank_space) {
            expanded_map.push(map[i].iter().map(|_| Space::ExpandedNothing).collect());
        } else {
            expanded_map.push(map[i].iter().map(Space::from).collect());
        }
    }

    for i in 0..map[0].len() {
        if map.iter().all(|line| is_blank_space(&line[i])) {
            for line in expanded_map.iter_mut() {
                line[i] = Space::ExpandedNothing;
            }
        }
    }

    expanded_map
}

fn map_galaxies(map: &Map) -> Vec<Galaxy> {
    let mut galaxies = Vec::new();

    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] == Space::Galaxy {
                galaxies.push(Galaxy {
                    position: (i as i32, j as i32),
                });
            }
        }
    }

    galaxies
}

fn combine_galaxies(galaxies: &Vec<Galaxy>) -> Vec<(&Galaxy, &Galaxy)> {
    let mut galaxy_pairs = Vec::new();

    for i in 0..galaxies.len() {
        for j in i + 1..galaxies.len() {
            galaxy_pairs.push((&galaxies[i], &galaxies[j]));
        }
    }

    galaxy_pairs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expand_map() {
        let input = ".#..
....
...#";
        let expected = vec![
            vec![
                Space::ExpandedNothing,
                Space::Galaxy,
                Space::ExpandedNothing,
                Space::Nothing,
            ],
            vec![
                Space::ExpandedNothing,
                Space::ExpandedNothing,
                Space::ExpandedNothing,
                Space::ExpandedNothing,
            ],
            vec![
                Space::ExpandedNothing,
                Space::Nothing,
                Space::ExpandedNothing,
                Space::Galaxy,
            ],
        ];
        assert_eq!(expand_map(input), expected);
    }
}
