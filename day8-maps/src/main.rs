use std::collections::HashMap;

#[derive(Debug, PartialEq)]
enum Direction {
    Left,
    Right,
}

type Node = u32;
type Turns = (Node, Node);

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

    let mut lines = document.lines();

    let directions = parse_directions(lines.next().unwrap());
    lines.next();

    let map = parse_map(lines.into_iter());

    let destination = get_node("ZZZ");
    let mut node: Node = get_node("AAA");
    let mut turn_count = 0;
    let mut turns = directions.iter().cycle();

    while node != destination {
        let (left, right) = map.get(&node).unwrap();
        let direction = turns.next().unwrap();

        node = match direction {
            Direction::Left => *left,
            Direction::Right => *right,
        };

        turn_count += 1;
    }

    println!("Number of turns: {}", turn_count);
}

fn parse_directions(line: &str) -> Vec<Direction> {
    let mut directions = Vec::new();

    for c in line.chars() {
        let direction = match c {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!("Invalid direction"),
        };
        directions.push(direction)
    }

    directions
}

fn parse_map<'a, I>(lines: I) -> HashMap<Node, Turns>
where
    I: Iterator<Item = &'a str>,
{
    let mut map = HashMap::new();

    for direction in lines {
        let source = get_node(&direction[0..3]);
        let left = get_node(&direction[7..10]);
        let right = get_node(&direction[12..15]);

        map.insert(source, (left, right));
    }

    map
}

// Get a unique node for a 3-letter address
fn get_node(address: &str) -> Node {
    let mut node: Node = 0;

    for c in address.bytes() {
        node = node << 8;
        node += c as Node;
    }

    node
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_directions() {
        let directions = parse_directions("LLRRLR");
        assert_eq!(
            directions,
            vec![
                Direction::Left,
                Direction::Left,
                Direction::Right,
                Direction::Right,
                Direction::Left,
                Direction::Right
            ]
        );
    }

    #[test]
    fn test_parse_map() {
        let map = parse_map(vec!["AAA = (LLL, RRR)"].into_iter());

        assert! {
            map.contains_key(&0x414141)
        }

        assert_eq! {
            map.get(&0x414141),
            Some(&(0x4c4c4c, 0x525252))
        }
    }

    #[test]
    fn test_get_node() {
        assert_eq!(get_node("ABC"), 0x414243);
        assert_eq!(get_node("AAA"), 0x414141);
        assert_eq!(get_node("ZZZ"), 0x5a5a5a);
    }
}
