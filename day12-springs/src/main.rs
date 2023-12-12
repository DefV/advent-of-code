#[derive(Debug, PartialEq,Clone)]
enum Spring {
    Operational,
    Damaged,
    Unknown,
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

impl From<&str> for Row {
    fn from(str: &str) -> Self {
        let (pattern, expected_sizes) = str.split_once(" ").unwrap();

        let pattern: Vec<Spring> = pattern.chars().map(Spring::from).collect();
        let expected_sizes: Vec<usize> = expected_sizes.split(",").map(|s| s.parse().unwrap()).collect();

        Self {
            pattern,
            expected_sizes,
        }
    }
}

impl Row {
    pub fn possible_options(&self) -> Vec<Vec<Spring>> {
        let mut options = vec![];
        let mut pattern = self.pattern.clone();

        while let Some(spring) = pattern.pop() {
            match spring {
                Spring::Unknown => {
                    if options.is_empty() {
                        options.push(vec![Spring::Operational.clone()]);
                        options.push(vec![Spring::Damaged.clone()]);
                    } else {
                        let mut new_options = vec![];
                        for option in options.iter_mut() {
                            let mut new_option = option.clone();
                            new_option.insert(0, Spring::Operational.clone());
                            new_options.push(new_option);

                            let mut new_option = option.clone();
                            new_option.insert(0, Spring::Damaged.clone());
                            new_options.push(new_option);
                        }
                        options = new_options;
                    }
                },
                known_spring => {
                    if options.is_empty() {
                        options.push(vec![known_spring])
                    } else {
                        for option in options.iter_mut() {
                            option.insert(0, known_spring.clone());
                        }
                    }
                }
            }
        }

        options
    }

    pub fn valid_options(&self) -> usize {
        self.possible_options().into_iter().filter(|option| self.is_valid(option)).count()
    }

    fn is_valid(&self, option: &Vec<Spring>) -> bool {
        let mut counts: Vec<usize> = vec![];
        let mut counter = 0;

        for spring in option {
            match spring {
                Spring::Operational => { counts.push(counter); counter = 0 }
                Spring::Damaged => counter += 1,
                _ => panic!("Unknown spring in option!"),
            }
        }

        counts.push(counter);
        counts = counts.into_iter().filter(|count| *count > 0).collect();
        counts == self.expected_sizes
    }
}

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
    
    let sum = document.lines().map(Row::from).map(|row| row.valid_options()).sum::<usize>();
    
    println!("Sum: {}", sum);
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
        let row = Row::from("??..# 1,1");
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
    fn test_spring_possible_options() {
        let row = Row::from("??..# 1,1");
        assert_eq!(
            row.possible_options().len(), 4
        );

        assert_eq!(
            row.possible_options(),
            vec![
                vec![Spring::Operational, Spring::Operational, Spring::Operational, Spring::Operational, Spring::Damaged],
                vec![Spring::Damaged, Spring::Operational, Spring::Operational, Spring::Operational, Spring::Damaged],
                vec![Spring::Operational, Spring::Damaged, Spring::Operational, Spring::Operational, Spring::Damaged],
                vec![Spring::Damaged, Spring::Damaged, Spring::Operational, Spring::Operational, Spring::Damaged],
            ]
        )
    }

    #[test]
    fn test_spring_valid_options() {
        assert_eq!(Row::from("??..# 1,1").valid_options(), 2);
        assert_eq!(Row::from("?###???????? 3,2,1").valid_options(), 10);
    }
}
