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
    numbers.push(number);
  }

  return numbers;
}

fn parse_line(line: &str) -> i32 {
  let first_number = line.find(char::is_numeric).unwrap();
  let last_number = line.rfind(char::is_numeric).unwrap();  

  let mut full_number = String::new();
  full_number.push_str(&line[first_number..first_number+1]);
  full_number.push_str(&line[last_number..last_number+1]);

  let number: i32 = full_number.parse().unwrap();
  return number;
}

// Test
mod tests {
    #[test]
    fn test_parse_line() {
        assert_eq!(super::parse_line("a1b7cz"), 17);
        assert_eq!(super::parse_line("a1b"), 11);
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