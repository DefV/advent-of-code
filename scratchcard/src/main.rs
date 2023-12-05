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

    let mut cards: Vec<Card> = document
        .lines()
        .map(Card::from_str)
        .collect();

    for i in 0..cards.len() {
        let card = &cards[i];
        let win_count = card.win_count() as usize;
        let copies = card.copies;

        if win_count > 0 {
            for u in 1..=win_count {
                let c = &mut cards[i + u];
                c.copies += copies;
            }
        }
    }

    let sum: usize = cards.iter().map(|c| c.copies ).sum();

    println!("Sum: {}", sum);
}

struct Card {
    winning_numbers: Vec<i32>,
    numbers_you_have: Vec<i32>,
    copies: usize,
}

impl Card {
    pub fn from_str(card_str: &str) -> Self {
        let numbers: Vec<Vec<i32>> = card_str
            .split(":")
            .last()
            .unwrap()
            .split("|")
            .map(Self::get_numbers)
            .collect();

        let winning_numbers = numbers[0].clone();
        let numbers_you_have = numbers[1].clone();

        Self {
            winning_numbers,
            numbers_you_have,
            copies: 1,
        }
    }

    pub fn win_count(&self) -> usize {
        self.winning_numbers
        .iter()
        .filter(|&n| self.numbers_you_have.contains(n))
        .count() 
    }

    pub fn score(&self) -> i32 {
        let win_count = self.win_count();

        match win_count {
            0 => 0,
            _ => i32::pow(2, win_count as u32 - 1),
        }
    }

    fn get_numbers(number_string: &str) -> Vec<i32> {
        number_string
            .split(" ")
            .map(|s| s.parse::<i32>())
            .filter_map(|x| x.ok())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_card_from_str() {
        let card_str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let card = Card::from_str(card_str);

        assert_eq!(card.winning_numbers, vec![41, 48, 83, 86, 17]);
        assert_eq!(card.numbers_you_have, vec![83, 86, 6, 31, 17, 9, 48, 53]);
    }

    #[test]
    fn test_card_score() {
        let card = Card {
            winning_numbers: vec![1, 2, 3, 4],
            numbers_you_have: vec![1, 2, 3, 5],
            copies: 1,
        };

        assert_eq!(card.score(), 4);
    }
}
