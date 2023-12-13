use std::collections::HashMap;

use rayon::prelude::*;

#[derive(PartialEq, Eq, Hash, Clone)]
enum Spring {
    Operational,
    Damaged,
    Unknown,
}

impl std::fmt::Debug for Spring {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Spring::Operational => write!(f, "."),
            Spring::Damaged => write!(f, "#"),
            Spring::Unknown => write!(f, "?"),
        }
    }
}

impl From<char> for Spring {
    fn from(c: char) -> Self {
        match c {
            '.' => Spring::Operational,
            '#' => Spring::Damaged,
            '?' => Spring::Unknown,
            _ => panic!("Not a spring character!"),
        }
    }
}

#[derive(Debug, PartialEq)]
struct Row {
    pattern: Vec<Spring>,
    expected_sizes: Vec<usize>,
}

impl From<String> for Row {
    fn from(str: String) -> Self {
        let (pattern, expected_sizes) = str.split_once(" ").unwrap();

        let pattern: Vec<Spring> = pattern.chars().map(Spring::from).collect();
        let expected_sizes: Vec<usize> = expected_sizes
            .split(",")
            .map(|s| s.parse().unwrap())
            .collect();

        Self {
            pattern,
            expected_sizes,
        }
    }
}

fn valid_options_for(pattern: Vec<Spring>, expected_sizes: Vec<usize>, cache: &mut HashMap<(Vec<Spring>, Vec<usize>), u128>) -> u128 {
    if let Some(result) = cache.get(&(pattern.clone(), expected_sizes.clone())) {
        return *result;
    }

    let mut valid_options = 0;

    let mut pattern = pattern;
    let mut expected_sizes = expected_sizes;

    // Find the first unknown or operational spring index
    let first_unknown_spring_index =
        pattern.iter().position(|spring| *spring != Spring::Damaged);

    match first_unknown_spring_index {
        Some(index) if pattern[index] == Spring::Operational => {
            let part = &pattern[..=index];
            match part {
                [.., Spring::Damaged, Spring::Operational] => {
                    let damaged_springs = part
                        .iter()
                        .filter(|spring| **spring == Spring::Damaged)
                        .count();
                    if expected_sizes.len() == 0 {
                        // All good
                    } else if damaged_springs == expected_sizes[0] {
                        // We can consume a part
                        expected_sizes.remove(0);
                        pattern = pattern[index + 1..].to_vec();

                        valid_options +=
                            valid_options_for(pattern.clone(), expected_sizes.clone(), cache);
                    } else {
                        // All good
                    }
                }
                _ => {
                    pattern = pattern[index + 1..].to_vec();
                    valid_options +=
                        valid_options_for(pattern.clone(), expected_sizes.clone(), cache);
                }
            }
        }
        Some(index) => {
            pattern[index] = Spring::Operational;
            valid_options += valid_options_for(pattern.clone(), expected_sizes.clone(), cache);

            pattern[index] = Spring::Damaged;
            valid_options += valid_options_for(pattern.clone(), expected_sizes.clone(), cache);
        }
        None => {
            let damaged_spring_count = pattern
            .iter()
            .filter(|&spring| *spring == Spring::Damaged)
            .count();

            if expected_sizes.len() == 1 && expected_sizes[0] == damaged_spring_count
            {
                valid_options += 1;
            } else if expected_sizes.is_empty() && damaged_spring_count == 0 {
                valid_options += 1;
            } else {
                // Fine
            }
        }
    }

    cache.insert((pattern, expected_sizes), valid_options);
    valid_options
}

impl Row {
    pub fn valid_options(&self) -> u128 {
        let mut cache = HashMap::new();
        valid_options_for(self.pattern.clone(), self.expected_sizes.clone(), &mut cache)
    }
}

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

    let sum = document
        .lines()
        .map(|line| {
            let (pattern, sizes) = line.split_once(" ").unwrap();
            vec![vec![pattern, pattern, pattern, pattern, pattern].join("?"), vec![sizes, sizes, sizes, sizes, sizes].join(",")].join(" ")
        })
        .map(Row::from)
        .par_bridge()
        .map(|row| dbg!(row.valid_options()))
        .sum::<u128>();

    println!("Sum: {}", sum);
    println!("Runtime: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spring_from_char() {
        let springs: Vec<Spring> = "?.#".chars().map(Spring::from).collect();
        assert_eq!(
            springs,
            vec![Spring::Unknown, Spring::Operational, Spring::Damaged]
        )
    }

    #[test]
    fn test_row_from_str() {
        let row = Row::from("??..# 1,1".to_string());
        assert_eq!(
            row,
            Row {
                pattern: vec![
                    Spring::Unknown,
                    Spring::Unknown,
                    Spring::Operational,
                    Spring::Operational,
                    Spring::Damaged
                ],
                expected_sizes: vec![1, 1]
            }
        )
    }

    #[test]
    fn test_spring_valid_options() {
        assert_eq!(Row::from("??..# 1,1".to_string()).valid_options(), 2);
        assert_eq!(Row::from("?###???????? 3,2,1".to_string()).valid_options(), 10);
    }

    #[test]
    fn test_performance_of_big_expansion() {
        let row = Row::from(".?.??????????#????#?.?.??????????#????#?.?.??????????#????#?.?.??????????#????#?.?.??????????#????#? 1,1,2,7,2,1,1,2,7,2,1,1,2,7,2,1,1,2,7,2,1,1,2,7,2".to_string());
        assert_eq!(row.valid_options(), 2);
    }
}
