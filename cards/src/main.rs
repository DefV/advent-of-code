mod hand;

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

    let mut cards = parse_document(document.as_str());
    cards.sort_unstable_by(|(a, _), (b, _)| a.cmp(b));

    let score = cards
        .iter()
        .enumerate()
        .fold(0, |acc, (idx, (_, value))| acc + (idx + 1) as u32 * value);

    println!("{}", score);
}

fn parse_document(document: &str) -> Vec<(hand::Hand, u32)> {
    document
        .lines()
        .map(|s| {
            let mut parts = s.split(char::is_whitespace);

            (
                hand::Hand::from_str(parts.next().unwrap()),
                parts.next().unwrap().parse().unwrap(),
            )
        })
        .to_owned()
        .collect()
}
