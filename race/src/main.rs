mod race;

use crate::race::Race;

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

    let races = parse_document(&document);

    let result = races
        .iter()
        .map(|race| race.winnable().len())
        .reduce(|a, b| a * b )
        .unwrap();
    
    println!("{:?}", result);
}

fn parse_document(document: &str) -> Vec<Race> {
    let mut lines = document.lines();
    let times = get_numbers(lines.next().unwrap());
    let distances = get_numbers(lines.next().unwrap());

    times
        .iter()
        .zip(distances.iter())
        .map(|(a, b)| Race::new(*a, *b))
        .collect()
}

fn get_numbers(line: &str) -> Vec<i64> {
    line.split(":")
        .last()
        .unwrap()
        .trim()
        .split(char::is_whitespace)
        .map(str::parse::<i64>)
        .filter_map(Result::ok)
        .collect()
}
