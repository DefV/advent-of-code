use std::time::Instant;

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

    let mut sequences: Vec<Vec<i64>> = document
        .lines()
        .map(str::split_whitespace)
        .map(|seq| seq.map(str::parse::<i64>).filter_map(Result::ok).collect())
        .collect();

    println!("Part 1: {}", sequences.iter().map(solve_next_for_seq).sum::<i64>());
    println!("Part 2: {}", sequences.iter_mut().map(|seq| { seq.reverse(); solve_next_for_seq(seq)}).sum::<i64>());
    println!("Elapsed: {:.2?}", now.elapsed());
}

fn solve_next_for_seq(sequence: &Vec<i64>) -> i64 {
    let mut sequence = sequence.clone();
    let mut result = *sequence.last().unwrap();

    while !sequence.iter().all(|&x| x == 0) {
        // Calculate the difference between each element
        sequence = sequence.windows(2).map(|w| &w[1] - &w[0]).collect();
        result += sequence.last().unwrap();
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    static SEQUENCE_1: [i64; 6] = [0, 3, 6, 9, 12, 15];
    static SEQUENCE_2: [i64; 6] = [1, 3, 6, 10, 15, 21];
    static SEQUENCE_3: [i64; 6] = [10, 13, 16, 21, 30, 45];

    #[test]
    fn test_solve_next_for_seq() {
        assert_eq!(solve_next_for_seq(&Vec::from(SEQUENCE_1)), 18);
        assert_eq!(solve_next_for_seq(&Vec::from(SEQUENCE_2)), 28);
        assert_eq!(solve_next_for_seq(&Vec::from(SEQUENCE_3)), 68);
    }
}
