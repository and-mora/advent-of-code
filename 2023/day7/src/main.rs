use std::cmp::Ordering;
use std::collections::HashMap;

/**

Exercise https://adventofcode.com/2023/day/7


**/

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
enum Type {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind
}

#[derive(Debug)]
pub struct Hand {
    cards: Vec<char>,
    bid: u32,
}

impl Hand {
    pub fn new(cards_as_string: String, bid: u32) -> Self {
        //validate input
        if cards_as_string.len() != 5 {
            panic!("Hand must consist of 5 cards, got: {}", cards_as_string);
        }
        if !cards_as_string.chars().all(|c| {
            c.is_ascii_uppercase()
                || c.is_digit(10)
                || c == 'T'
                || c == 'J'
                || c == 'Q'
                || c == 'K'
                || c == 'A'
        }) {
            panic!(
                "Hand must consist of uppercase letters or digits, got: {}",
                cards_as_string
            );
        }
        let cards: Vec<char> = cards_as_string.chars().collect();
        Hand { cards, bid }
    }

    fn type_of_hand(&self) -> Type {
        let map_with_count = self
            .cards
            .iter()
            .fold(HashMap::<char, u8>::new(), |mut acc, card| {
                acc.entry(*card).and_modify(|v| *v += 1).or_insert(1);
                acc
            });

        if map_with_count.iter().map(|kv| *kv.1).max().unwrap() == 5 {
            return Type::FiveOfAKind;
        }
        if map_with_count.iter().map(|kv| *kv.1).max().unwrap() == 4 {
            return Type::FourOfAKind;
        }
        if map_with_count.iter().map(|kv| *kv.1).max().unwrap() == 3
            && map_with_count.iter().map(|kv| *kv.1).min().unwrap() == 2
        {
            return Type::FullHouse;
        }
        if map_with_count.iter().map(|kv| *kv.1).max().unwrap() == 3 {
            return Type::ThreeOfAKind;
        }
        if map_with_count.iter().filter(|a| *a.1 == 2).count() == 2 {
            return Type::TwoPair;
        }
        if map_with_count.iter().filter(|a| *a.1 == 2).count() == 1 {
            return Type::OnePair;
        }

        Type::HighCard
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.type_of_hand()
            .cmp(&other.type_of_hand())
            .then_with(|| {
                for (index, card) in self.cards.iter().enumerate() {
                    if card.cmp(&other.cards[index]) != Ordering::Equal {
                        return card.cmp(&other.cards[index]);
                    }
                }
                Ordering::Equal
            })
    }
}

impl Eq for Hand {}

impl PartialEq<Self> for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.type_of_hand() == other.type_of_hand()
    }
}

impl PartialOrd<Self> for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub struct Game {
    pub hands: Vec<Hand>,
}

impl Game {
    pub fn new(hands: Vec<Hand>) -> Self {
        Game { hands }
    }

    pub fn play(&self) -> u32 {
        let mut sorted_hands: Vec<&Hand> = self.hands.iter().clone().collect();
        sorted_hands.sort_by(|a, b| b.cmp(a));
        println!("Sorted hands: {:?}", sorted_hands);
        sorted_hands
            .iter()
            .enumerate()
            .map(|h| {
                println!("hand with bid {} multiply rank {}", h.1.bid, h.0 + 1);
                h.1.bid * (h.0 + 1) as u32
            })
            .sum()
    }
}

fn main() {
    println!("AOC 2023 day 7.");

    let hand1 = Hand::new("32T3K".to_string(), 765);
    let hand2 = Hand::new("T55J5".to_string(), 684);
    let hand3 = Hand::new("KK677".to_string(), 28);
    let hand4 = Hand::new("KTJJT".to_string(), 220);
    let hand5 = Hand::new("QQQJA".to_string(), 483);
    
    let game = Game::new(vec![hand1, hand2, hand3, hand4, hand5]);

    println!("Game hands: {:?}", game.hands);

    let total_bid = game.play();
    println!("Total bid: {}", total_bid);
    assert_eq!(total_bid, 6440);
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_hand_ordering_less() {
        let hand1 = Hand::new("32T3K".to_string(), 0);
        let hand2 = Hand::new("T55J5".to_string(), 0);
        assert_eq!(hand1 > hand2, false);
    }

    #[test]
    fn test_hand_ordering_second_criteria() {
        let hand1 = Hand::new("3233K".to_string(), 0);
        let hand2 = Hand::new("T55J5".to_string(), 0);
        assert_eq!(hand1 > hand2, false);
    }

    #[test]
    fn test_hand_ordering_second_criteria_greater() {
        let hand1 = Hand::new("QQQJA".to_string(), 0);
        let hand2 = Hand::new("T55J5".to_string(), 0);
        assert_eq!(hand1 > hand2, true);
    }

    #[test]
    fn test_hand_ordering_equal() {
        let hand1 = Hand::new("T55J5".to_string(), 0);
        let hand2 = Hand::new("T55J5".to_string(), 0);
        assert_eq!(hand1.cmp(&hand2), Ordering::Equal);
    }

    #[test]
    fn test_hand_ordering_greater() {
        let hand1 = Hand::new("55555".to_string(), 0);
        let hand2 = Hand::new("T55J5".to_string(), 0);
        assert_eq!(hand1.cmp(&hand2), Ordering::Greater);
    }

    #[test]
    fn test_hand_type_high_card() {
        let hand = Hand::new("23456".to_string(), 684);
        assert_eq!(hand.type_of_hand(), Type::HighCard);
    }
    #[test]
    fn test_hand_type_one_pair() {
        let hand = Hand::new("32T3K".to_string(), 765);
        assert_eq!(hand.type_of_hand(), Type::OnePair);
    }
    #[test]
    fn test_hand_type_two_pair() {
        let hand = Hand::new("32T32".to_string(), 765);
        assert_eq!(hand.type_of_hand(), Type::TwoPair);
    }
    #[test]
    fn test_hand_type_full_house() {
        let hand = Hand::new("T55J5".to_string(), 684);
        assert_eq!(hand.type_of_hand(), Type::ThreeOfAKind);
    }
    #[test]
    fn test_hand_type_four_of_a_kind() {
        let hand = Hand::new("K2KKK".to_string(), 765);
        assert_eq!(hand.type_of_hand(), Type::FourOfAKind);
    }
    #[test]
    fn test_hand_type_five_of_a_kind() {
        let hand = Hand::new("KKKKK".to_string(), 765);
        assert_eq!(hand.type_of_hand(), Type::FiveOfAKind);
    }
}
