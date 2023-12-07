use std::collections::HashMap;

#[derive(Eq,PartialEq,Debug)]
pub struct Hand {
    cards: Vec<char>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Rank {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

const CARD_ORDER: [char; 13] = [
    '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A'
];


impl Hand {
    pub fn from_str(s: &str) -> Self {
        let cards = s.chars().collect();

        Hand { cards }
    }

    // Returns the rank of the hand and the highest card
    fn rank(&self) -> Rank {
        let mut counts: Vec<u8> = self
            .cards
            .iter()
            .fold(HashMap::new(), |mut acc: HashMap<char, u8>, card: &char| {
                *acc.entry(*card).or_insert(0) += 1;
                acc
            })
            .values()
            .copied()
            .collect();

        counts.sort_unstable_by(|a, b| b.cmp(a));

        match counts.as_slice() {
            [5] => Rank::FiveOfAKind,
            [4, ..] => Rank::FourOfAKind,
            [3, 2] => Rank::FullHouse,
            [3, ..] => Rank::ThreeOfAKind,
            [2, 2, ..] => Rank::TwoPair,
            [2, ..] => Rank::OnePair,
            _ => Rank::HighCard,
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let rank_cmp = self.rank().cmp(&other.rank());

        if rank_cmp.is_eq() {
            for (idx, &card) in self.cards.iter().enumerate() {
                let other_card = other.cards[idx];

                let card_idx = CARD_ORDER.iter().position(|&c| c == card).unwrap();
                let other_card_idx = CARD_ORDER.iter().position(|&c| c == other_card).unwrap();

                let card_cmp = card_idx.cmp(&other_card_idx);
                if !card_cmp.is_eq() {
                    return card_cmp;
                }
            }

            return std::cmp::Ordering::Equal;
        } else {
            rank_cmp
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hand_from_str() {
        let hand = Hand::from_str("32T3K");
        assert_eq!(hand.cards, vec!('3', '2', 'T', '3', 'K'));
    }

    #[test]
    fn test_hand_rank() {
        assert_eq!(Hand::from_str("AAAAA").rank(), Rank::FiveOfAKind);
        assert_eq!(Hand::from_str("KAAAA").rank(), Rank::FourOfAKind);
        assert_eq!(Hand::from_str("KKAAA").rank(), Rank::FullHouse);
        assert_eq!(Hand::from_str("KKA23").rank(), Rank::OnePair);
        assert_eq!(Hand::from_str("KK223").rank(), Rank::TwoPair);
        assert_eq!(Hand::from_str("KKK23").rank(), Rank::ThreeOfAKind);
        assert_eq!(Hand::from_str("KQJT9").rank(), Rank::HighCard);
    }

    #[test]
    fn test_sorting_hands() {
        let mut hands = vec![
            Hand::from_str("QQQQQ"),
            Hand::from_str("KKKKK"),
            Hand::from_str("2AAAA"),
            Hand::from_str("32222"),
            Hand::from_str("23456"),
            Hand::from_str("23457"),
        ];

        hands.sort();

        assert_eq!(hands[0].cards, vec!('2', '3', '4', '5', '6'));
        assert_eq!(hands[1].cards, vec!('2', '3', '4', '5', '7'));
        assert_eq!(hands[2].cards, vec!('2', 'A', 'A', 'A', 'A'));
        assert_eq!(hands[3].cards, vec!('3', '2', '2', '2', '2'));
    }
}
