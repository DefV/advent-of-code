mod trebuchet;

fn main() {
    // Open file passed in ARGV
    let args: Vec<String> = std::env::args().collect();
    // Print usage if no file is passed
    if args.len() < 2 {
        println!("Usage: trebuchet <filename>");
        return;
    }
    
    let filename = &args[1];
    let document = std::fs::read_to_string(filename).expect("Something went wrong reading the file");

    // Sum document
    let sum = trebuchet::sum_document(&document);

    // Print result
    println!("{}", sum);
}
