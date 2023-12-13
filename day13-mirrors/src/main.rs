pub mod sequence;

use sequence::Pattern;
use sequence::Reflection;

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
    
    let patterns: Vec<Pattern> = document.split("\n\n").map(Pattern::from).collect();
    let sum = patterns.iter().map(|pattern| pattern.find_smudge_reflection()).fold(0, |acc, (middle, reflection)| {
        if reflection == Reflection::Horizontal {
            acc + middle * 100
        } else {
            acc + middle
        }
    });

    println!("Sum: {}", sum);

    println!("Runtime: {:?}", now.elapsed());
}