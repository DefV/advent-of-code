use std::collections::HashMap;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
  static ref NUMBER_WORDS: HashMap<&'static str, &'static str> = {
    let numbers = [
        ("one", "1"), ("two", "2"), ("three", "3"), ("four", "4"),
        ("five", "5"), ("six", "6"), ("seven", "7"), ("eight", "8"), ("nine", "9"),
    ];
    numbers.iter().cloned().collect()
  };

  static ref NUMBER_REGEX: Regex = Regex::new(
    r"\A(\d|one|two|three|four|five|six|seven|eight|nine)"
  ).unwrap();
}

pub fn sum_document(document: &str) -> i32 {
  let numbers = parse_document(document);
  let mut sum = 0;

  for number in numbers {
    sum += number;
  }

  return sum;
}

fn parse_document(document: &str) -> Vec<i32> {
  let mut numbers = Vec::new();

  for line in document.lines() {
    let number = parse_line(line);
    // Show your work
    numbers.push(number);
  }

  return numbers;
}

fn detect_numbers(line: &str) -> Vec<&str> {
  let mut idx = 0;
  let mut result = Vec::new();

  while idx < line.len() {
    match NUMBER_REGEX.captures(&line[idx..]) {
      Some(captures) => {
        let word = captures.get(0).unwrap().as_str();
        let number = words_to_number(word).unwrap();
        result.push(number);
      },
      None => {}
    }
    idx += 1;
  }

  result
}

fn parse_line(line: &str) -> i32 {
  let numbers_in_string = detect_numbers(line);

  let &first_number = numbers_in_string.first().unwrap();
  let &last_number = numbers_in_string.last().unwrap_or(&first_number);

  let mut full_number = String::new();
  full_number.push_str(first_number);
  full_number.push_str(last_number);

  let number: i32 = full_number.parse().unwrap();

  return number;
}

fn words_to_number(word: &str) -> Option<&str> {
  if word.chars().all(char::is_numeric) {
    return Some(word);
  } else {
    return NUMBER_WORDS.get(word).copied();
  }
}

// Test
mod tests {
    #[test]
    fn test_parse_line() {
        assert_eq!(super::parse_line("a1b7cz"), 17);
        assert_eq!(super::parse_line("a1b"), 11);
    }

    #[test]
    fn test_parse_line_also_detects_numbers_as_words() {
      assert_eq!(super::parse_line("aone2b7cz"), 17);
      assert_eq!(super::parse_line("eighthree"), 83);
    }

    #[test]
    fn test_parse_document() {
        let document = "a1b7cz\na1b";
        assert_eq!(super::parse_document(document), vec![17, 11]);
    }

    #[test]
    fn test_sum_document() {
        let document = "a1b7cz\na1b";
        assert_eq!(super::sum_document(document), 28);
    }
}