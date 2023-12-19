use std::collections::HashMap;

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

#[derive(PartialEq, Debug, Clone)]
enum Rule {
    Lt(char, u32, String),
    Gt(char, u32, String),
    Default(String),
}

const ACCEPTED: &str = "A";
const REJECTED: &str = "R";
const ENTRANCE: &str = "in";


impl From<&str> for Rule {
    fn from(str: &str) -> Self {
        if let Some((condition, destination)) = str.split_once(':') {
            let key = condition.chars().next().unwrap();
            let conditional = condition.chars().nth(1).unwrap();
            let value = condition[2..].parse::<u32>().unwrap();

            match conditional {
                '<' => Rule::Lt(key, value, destination.to_string()),
                '>' => Rule::Gt(key, value, destination.to_string()),
                _ => unreachable!(),
            }
        } else {
            Rule::Default(str.to_string())
        }
    }
}

struct Workflow {
    steps: HashMap<String, Vec<Rule>>,
}

impl From<&str> for Workflow {
    fn from(document: &str) -> Self {
        let mut steps = HashMap::new();

        for line in document.lines() {
            let mut parts = line.split(&['{', '}', ',']).filter(|s| !s.is_empty());
            let key = parts.next().unwrap();
            let rules = parts.map(Rule::from).collect();

            steps.insert(key.to_string(), rules);
        }

        Workflow { steps }
    }
}

impl Workflow {
    fn is_decision(destination: &str) -> bool {
        destination == ACCEPTED || destination == REJECTED
    }

    fn is_accepted(&self, input: &Input) -> bool {
        let mut destination = String::from(ENTRANCE);

        while !Self::is_decision(&destination) {
            let rules = self.steps.get(&destination).unwrap();

            for rule in rules {
                match rule {
                    Rule::Lt(key, value, new_destination) => {
                        if input.values.get(key).unwrap() < value {
                            destination = new_destination.clone();
                            break;
                        }
                    }
                    Rule::Gt(key, value, new_destination) => {
                        if input.values.get(key).unwrap() > value {
                            destination = new_destination.clone();
                            break;
                        }
                    }
                    Rule::Default(new_destination) => {
                        destination = new_destination.clone();
                    }
                }
            }
        }

        destination == ACCEPTED
    }

    fn walk_possibilities(&self, rule_key: String, mut input_range: InputRange) -> u64 {
        let mut sum = 0_u64;
        let rules = self.steps.get(&rule_key).unwrap();

        for rule in rules {
            match rule {
                Rule::Lt(key, value, new_destination) => {
                    let (current_range, next_range) = input_range.create_lt_branches(key, value);
                    input_range = next_range;

                    sum += match new_destination.as_str() {
                        ACCEPTED => current_range.possibility_count(),
                        REJECTED => 0,
                        _ => self.walk_possibilities(new_destination.clone(), current_range),
                    }
                }
                Rule::Gt(key, value, new_destination) => {
                    let (current_range, next_range) = input_range.create_gt_branches(key, value);
                    input_range = next_range;

                    sum += match new_destination.as_str() {
                        ACCEPTED => current_range.possibility_count(),
                        REJECTED => 0,
                        _ => self.walk_possibilities(new_destination.clone(), current_range),
                    }
                }
                Rule::Default(new_destination) => {
                    let current_range = input_range.clone();

                    sum += match new_destination.as_str() {
                        ACCEPTED => current_range.possibility_count(),
                        REJECTED => 0,
                        _ => self.walk_possibilities(new_destination.clone(), current_range),
                    }
                }
            }
        }

        sum
    }
}

#[derive(Debug)]
struct Input {
    values: HashMap<char, u32>,
}

impl From<&str> for Input {
    fn from(s: &str) -> Self {
        let s = &s[1..s.len() - 1];

        let values = s
            .split(',')
            .map(|str| {
                let mut keyvalue = str.split('=');
                let key = keyvalue.next().unwrap().chars().next().unwrap();
                let value = keyvalue.next().unwrap().parse::<u32>().unwrap();
                (key, value)
            })
            .collect();

        Input { values }
    }
}

impl Input {
    fn value(&self) -> u32 {
        self.values.values().sum()
    }
}

#[derive(Debug, Clone)]
struct InputRange {
    inputs: HashMap<char, std::ops::Range<u16>>,
}

impl InputRange {
    fn default() -> Self {
        Self {
            inputs: HashMap::from([
                ('x', 1..4001),
                ('m', 1..4001),
                ('a', 1..4001),
                ('s', 1..4001),
            ]),
        }
    }

    fn possibility_count(&self) -> u64 {
        self.inputs
            .values()
            .map(|v| v.len() as u64)
            .product::<u64>()
    }

    fn create_lt_branches(&self, key: &char, value: &u32) -> (Self, Self) {
        let mut current_range = self.clone();
        let mut next_range = self.clone();

        let current_key_range = current_range.inputs.get_mut(key).unwrap();
        *current_key_range = current_key_range.start..current_key_range.end.min(*value as u16);

        let next_key_range = next_range.inputs.get_mut(key).unwrap();
        *next_key_range = (*value as u16)..next_key_range.end;

        (current_range, next_range)
    }

    fn create_gt_branches(&self, key: &char, value: &u32) -> (Self, Self) {
        let mut current_range = self.clone();
        let mut next_range = self.clone();

        let current_key_range = current_range.inputs.get_mut(key).unwrap();
        *current_key_range = (*value as u16 + 1)..current_key_range.end;

        let next_key_range = next_range.inputs.get_mut(key).unwrap();
        *next_key_range = next_key_range.start..(*value as u16 + 1);

        (current_range, next_range)
    }
}

fn part1(document: &str) {
    let (rules, inputs) = document.split_once("\n\n").unwrap();
    let workflow = Workflow::from(rules);
    let inputs: Vec<Input> = inputs.lines().map(Input::from).collect();

    let sum = inputs
        .into_iter()
        .filter(|i| workflow.is_accepted(i))
        .map(|i| i.value())
        .sum::<u32>();

    println!("Part 1: {}", sum);
}

fn part2(document: &str) {
    let (rules, _inputs) = document.split_once("\n\n").unwrap();
    let workflow = Workflow::from(rules);
    let sum = workflow.walk_possibilities(String::from(ENTRANCE), InputRange::default());

    println!("Part 2: {}", sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_input() {
        let input = Input::from("{x=1679,m=44,a=2067,s=496}");

        assert_eq!(input.values.get(&'x'), Some(&1679));
        assert_eq!(input.values.get(&'m'), Some(&44));
        assert_eq!(input.values.get(&'a'), Some(&2067));
        assert_eq!(input.values.get(&'s'), Some(&496));
    }

    #[test]
    fn test_rule_from() {
        assert_eq!(
            Rule::from("x<1679:px"),
            Rule::Lt('x', 1679, String::from("px"))
        );
        assert_eq!(
            Rule::from("x>1679:px"),
            Rule::Gt('x', 1679, String::from("px"))
        );
        assert_eq!(Rule::from("px"), Rule::Default(String::from("px")))
    }

    #[test]
    fn test_create_lt_branches() {
        let input_range = InputRange::default();
        let (current_range, next_range) = input_range.create_lt_branches(&'x', &1679);

        assert_eq!(current_range.inputs.get(&'x'), Some(&(1..1679)));
        assert_eq!(next_range.inputs.get(&'x'), Some(&(1679..4001)));
    }

    #[test]
    fn test_create_gt_branches() {
        let input_range = InputRange::default();
        let (current_range, next_range) = input_range.create_gt_branches(&'x', &1679);

        assert_eq!(current_range.inputs.get(&'x'), Some(&(1680..4001)));
        assert_eq!(next_range.inputs.get(&'x'), Some(&(1..1680)));
    }
}
