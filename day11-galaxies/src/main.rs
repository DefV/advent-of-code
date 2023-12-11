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
    let galaxy_pairs = combine_galaxies(&galaxies);

    let galaxy_distance: usize = galaxy_pairs.into_iter().map(|(g1, g2)| g1.distance_to(g2) ).sum();

    println!("{}", galaxy_distance);
}

static BLANK_SPACE: char = '.';
static GALAXY: char = '#';

type Map = Vec<Vec<char>>;

#[derive(Debug)]
struct Galaxy {
    position: (i32, i32)
}

impl Galaxy {
    fn distance_to(&self, other: &Galaxy) -> usize {
        let (x1, y1) = self.position;
        let (x2, y2) = other.position;

        ((x1 - x2).abs() + (y1 - y2).abs()) as usize
    }
}

fn is_blank_space(c: &char) -> bool {
    *c == BLANK_SPACE
}

fn expand_map(document: &str) -> Map {
    let mut expanded_map = Vec::new();

    // Expand height
    let map: Vec<Vec<char>> = document
        .lines()
        .map(|line| line.chars().collect())
        .collect();
        
    for i in 0..map.len() {
        if map[i].iter().all(is_blank_space) {
            expanded_map.push(map[i].clone());
            expanded_map.push(map[i].clone());
        } else {
            expanded_map.push(map[i].clone());
        }
    }

    let mut shifted_idx = 0;
    for i in 0..map[0].len() {
        if map.iter().all(|line| is_blank_space(&line[i])) {
            for line in expanded_map.iter_mut() {
                line.insert(i + shifted_idx, BLANK_SPACE);
            }
            shifted_idx += 1
        }
    }

    expanded_map
}

fn map_galaxies(map: &Map) -> Vec<Galaxy> {
    let mut galaxies = Vec::new();

    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] == GALAXY {
                galaxies.push(Galaxy { position: (i as i32, j as i32) });
            }
        }
    }

    galaxies
}

fn combine_galaxies(galaxies: &Vec<Galaxy>) -> Vec<(&Galaxy, &Galaxy)> {
    let mut galaxy_pairs = Vec::new();

    for i in 0..galaxies.len() {
        for j in i+1..galaxies.len() {
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
            vec!['.', '.', '#', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '#'],
        ];
        assert_eq!(expand_map(input), expected);
    }
}
