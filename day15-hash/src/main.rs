mod boxes;

use boxes::hash;
use boxes::Boxes;

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

fn part1(document: &str) {
    let total: u64 = document.split(",").map(|s| hash(s) as u64).sum();
    println!("Part 1: Total: {}", total);
}

fn part2(document: &str) {
    let mut boxes = Boxes::new();
    for command in document.split(",") {
        boxes.process_command(command)
    }
    let total = boxes.total_power();
    println!("Part 2: Total: {}", total);
}