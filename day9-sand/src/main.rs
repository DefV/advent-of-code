use nalgebra::{DMatrix, DVector};

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

    let sum: i64 = document
        .lines()
        .map(str::split_whitespace)
        .map(|seq| seq.map(str::parse::<i64>).filter_map(Result::ok).collect() )
        .map(|seq| { 
            assert_eq!(solve_n_for_seq(&seq, seq.len() - 1), seq[seq.len() - 1]);
            seq
        })
        .map(|seq: Vec<_>| dbg!(solve_n_for_seq(dbg!(&seq), seq.len())))
        .sum();

    println!("{}", sum)
}

fn solve_n_for_seq(sequence: &Vec<i64>, n: usize) -> i64 {
    let polynomial = find_polynomial(sequence);
    let mut result = 0.0;

    for (i, coefficient) in polynomial.iter().enumerate() {
        result += coefficient * (n as f64).powi(i as i32);
    }

    result.round() as i64
}

fn find_polynomial(sequence: &Vec<i64>) -> Vec<f64> {
    let degree = dbg!(find_degree(sequence.clone()));
    let sequence = sequence.iter().map(|&x| x as f64).collect::<Vec<f64>>();
    let rows = generate_vandermonde(sequence.len(), degree);
    let columns = DVector::from_column_slice(&sequence);

    
    let coefficients = rows.clone().svd(true, true).solve(&columns, 1e-12).unwrap();

    coefficients.iter().map(|&x| x).collect::<Vec<f64>>()
}

fn generate_vandermonde(size: usize, degree: usize) -> DMatrix<f64> {
    let mut x_poly = DMatrix::from_element(size, degree + 1, 1.0);

    for i in 0..size {
        for j in 1..=degree {
            x_poly[(i, j)] = (i as f64).powf(j as f64);
        }
    }

    x_poly
}

fn find_degree(mut sequence: Vec<i64>) -> usize {
    let mut degree = 0;

    while !sequence.iter().all(|&x| x == 0) {
        // Calculate the difference between each element
        sequence = sequence.windows(2).map(|w| &w[1] - &w[0]).collect();
        degree += 1;
    }

    degree - 1 // The last iteration will always be all 0s
}

#[cfg(test)]
mod tests {
    use super::*;

    static SEQUENCE_1: [i64; 6] = [0, 3, 6, 9, 12, 15];
    static SEQUENCE_2: [i64; 6] = [1, 3, 6, 10, 15, 21];
    static SEQUENCE_3: [i64; 6] = [10, 13, 16, 21, 30, 45];

    #[test]
    fn test_find_degree() {
        assert_eq!(find_degree(Vec::from(SEQUENCE_1)), 1);
        assert_eq!(find_degree(Vec::from(SEQUENCE_2)), 2);
        assert_eq!(find_degree(Vec::from(SEQUENCE_3)), 3);
    }

    #[test]
    fn test_solve_n_for_seq() {
        assert_eq!(solve_n_for_seq(&Vec::from(SEQUENCE_1), 6), 18);
        assert_eq!(solve_n_for_seq(&Vec::from(SEQUENCE_2), 6), 28);
        assert_eq!(solve_n_for_seq(&Vec::from(SEQUENCE_3), 6), 68);
    }
}
