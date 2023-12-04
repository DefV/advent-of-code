use std::collections::HashMap;
use std::cmp;

use regex::Regex;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: cubes <filename>");
        return;
    }

    let filename = &args[1];
    let document =
        std::fs::read_to_string(filename).expect("Something went wrong reading the file");

    let games = process_document(&document);
    let sum = games
        .iter()
        .filter(|game| game.possible())
        .fold(0, |sum, game| sum + game.id);

    let power = games.iter().fold(0, |sum, game| sum + game.power());

    println!("Sum of possible games: {}", sum);
    println!("Power of all games: {}", power);
}

struct Game<'g> {
    id: i32,
    turns: Vec<HashMap<&'g str, i32>>,
}

const POSSIBLE_CONTENTS: [(&str, i32); 3] = [("red", 12), ("green", 13), ("blue", 14)];
impl<'g> Game<'g> {
    fn possible(&self) -> bool {
        self.turns.iter().all(|turn| {
            POSSIBLE_CONTENTS
                .iter()
                .all(|(color, count)| turn.get(color).unwrap_or(&0) <= count)
        })
    }

    fn power(&self) -> i32 {
        POSSIBLE_CONTENTS.iter().fold(1, |power, (color, _)| {
            self.turns
                .iter()
                .fold(0,|max, turn| 
                    cmp::max(*turn.get(color).unwrap_or(&0), max)
                ) * power
        })
    }
}

fn process_document(document: &str) -> Vec<Game> {
    document.lines().map(|line| process_line(line)).collect()
}

fn process_line(line: &str) -> Game {
    let regex: Regex = Regex::new(r"Game (\d+):(.*)").unwrap();
    let captures = regex.captures(line).unwrap();
    let id: i32 = captures.get(1).unwrap().as_str().parse().unwrap();

    let mut game = Game {
        id,
        turns: Vec::new(),
    };

    let turns = captures.get(2).unwrap().as_str().split(";");
    let turn_regex: Regex = Regex::new(r"(\d+) (\w+)").unwrap();
    for turn_str in turns {
        let mut turn: HashMap<&str, i32> = HashMap::new();
        for captures in turn_regex.captures_iter(turn_str) {
            let count: i32 = captures.get(1).unwrap().as_str().parse().unwrap();
            let color: &str = captures.get(2).unwrap().as_str();
            turn.insert(color, count);
        }

        game.turns.push(turn);
    }

    game
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_line() {
        let line = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let game = process_line(line);

        assert_eq!(game.id, 1);

        assert_eq!(game.turns.len(), 3);
        assert_eq!(
            game.turns.iter().map(|t| t.len()).collect::<Vec<_>>(),
            [2, 3, 1]
        );

        assert_eq!(game.turns[0].get("blue"), Some(&3));
    }

    #[test]
    fn test_process_document() {
        let document = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\nGame 2: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let games = process_document(document);

        assert_eq!(games.len(), 2);
        assert_eq!(games[0].id, 1);
        assert_eq!(games[1].id, 2);
    }

    #[test]
    fn test_possible_with_possible() {
        let possible_game = Game {
            id: 1,
            turns: vec![
                [("red", 12), ("green", 13), ("blue", 14)]
                    .iter()
                    .cloned()
                    .collect(),
                [("red", 12), ("blue", 14)].iter().cloned().collect(),
                [("red", 10), ("green", 13), ("blue", 14)]
                    .iter()
                    .cloned()
                    .collect(),
            ],
        };

        assert!(possible_game.possible());
    }

    #[test]
    fn test_possible_with_impossible() {
        let impossible_game = Game {
            id: 1,
            turns: vec![
                [("red", 12), ("green", 13), ("blue", 14)]
                    .iter()
                    .cloned()
                    .collect(),
                [("red", 12), ("green", 13), ("blue", 15)]
                    .iter()
                    .cloned()
                    .collect(),
                [("red", 12), ("green", 13), ("blue", 13)]
                    .iter()
                    .cloned()
                    .collect(),
            ],
        };

        assert!(!impossible_game.possible());
    }

    #[test]
    fn test_power() {
        let game = Game {
            id: 1,
            turns: vec![
                [("red", 1), ("blue", 12)] .iter() .cloned() .collect(),
                [("red", 10), ("green", 13), ("blue", 15)] .iter() .cloned() .collect(),
                [("red", 5), ("green", 13), ("blue", 13)] .iter() .cloned() .collect(),
            ],
        };

        assert_eq!(game.power(), 10 * 15 * 13);
    }
}
