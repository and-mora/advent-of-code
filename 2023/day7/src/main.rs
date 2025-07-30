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
    FiveOfAKind,
}

#[derive(Debug, Copy, Ord, PartialOrd, Eq, PartialEq, Hash, Clone)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl From<char> for Card {
    fn from(value: char) -> Self {
        match value {
            '2' => Card::Two,
            '3' => Card::Three,
            '4' => Card::Four,
            '5' => Card::Five,
            '6' => Card::Six,
            '7' => Card::Seven,
            '8' => Card::Eight,
            '9' => Card::Nine,
            'T' => Card::Ten,
            'J' => Card::Jack,
            'Q' => Card::Queen,
            'K' => Card::King,
            'A' => Card::Ace,
            _ => panic!("Invalid card: {}", value),
        }
    }
}

#[derive(Debug)]
pub struct Hand {
    cards: [Card; 5],
    bid: u32,
}

impl Hand {
    pub fn new(cards_as_string: &str, bid: u32) -> Result<Self, &str> {
        //validate input
        if cards_as_string.len() != 5 {
            return Err("Hand must consist of 5 cards");
        }
        if !cards_as_string
            .chars()
            .all(|c| c.is_digit(10) || "TJQKA".contains(c))
        {
            return Err("Hand must consist of uppercase letters or digits");
        }
        let cards = cards_as_string
            .chars()
            .map(|c| Card::from(c))
            .collect::<Vec<Card>>()
            .try_into()
            .unwrap();
        Ok(Hand { cards, bid })
    }

    fn type_of_hand(&self) -> Type {
        let map_with_count = self.cards.iter().fold(HashMap::new(), |mut acc, card| {
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
                    match card.cmp(&other.cards[index]) {
                        Ordering::Greater => return Ordering::Greater,
                        Ordering::Less => return Ordering::Less,
                        Ordering::Equal => continue,
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
        sorted_hands.sort();
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

    let hand1 = Hand::new("32T3K", 765).unwrap();
    let hand2 = Hand::new("T55J5", 684).unwrap();
    let hand3 = Hand::new("KK677", 28).unwrap();
    let hand4 = Hand::new("KTJJT", 220).unwrap();
    let hand5 = Hand::new("QQQJA", 483).unwrap();

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
        let hand1 = Hand::new("32T3K", 0);
        let hand2 = Hand::new("T55J5", 0);
        assert_eq!(hand1 > hand2, false);
    }

    #[test]
    fn test_hand_ordering_second_criteria() {
        let hand1 = Hand::new("3233K", 0);
        let hand2 = Hand::new("T55J5", 0);
        assert_eq!(hand1 > hand2, false);
    }

    #[test]
    fn test_hand_ordering_second_criteria_greater() {
        let hand1 = Hand::new("QQQJA", 0);
        let hand2 = Hand::new("T55J5", 0);
        assert_eq!(hand1 > hand2, true);
    }

    #[test]
    fn test_hand_ordering_equal() {
        let hand1 = Hand::new("T55J5", 0);
        let hand2 = Hand::new("T55J5", 0);
        assert_eq!(hand1.cmp(&hand2), Ordering::Equal);
    }

    #[test]
    fn test_hand_ordering_greater() {
        let hand1 = Hand::new("55555", 0);
        let hand2 = Hand::new("T55J5", 0);
        assert_eq!(hand1.cmp(&hand2), Ordering::Greater);
    }

    #[test]
    fn test_hand_type_high_card() {
        let hand = Hand::new("23456", 684).unwrap();
        assert_eq!(hand.type_of_hand(), Type::HighCard);
    }
    #[test]
    fn test_hand_type_one_pair() {
        let hand = Hand::new("32T3K", 765).unwrap();
        assert_eq!(hand.type_of_hand(), Type::OnePair);
    }
    #[test]
    fn test_hand_type_two_pair() {
        let hand = Hand::new("32T32", 765).unwrap();
        assert_eq!(hand.type_of_hand(), Type::TwoPair);
    }
    #[test]
    fn test_hand_type_full_house() {
        let hand = Hand::new("T55J5", 684).unwrap();
        assert_eq!(hand.type_of_hand(), Type::ThreeOfAKind);
    }
    #[test]
    fn test_hand_type_four_of_a_kind() {
        let hand = Hand::new("K2KKK", 765).unwrap();
        assert_eq!(hand.type_of_hand(), Type::FourOfAKind);
    }
    #[test]
    fn test_hand_type_five_of_a_kind() {
        let hand = Hand::new("KKKKK", 765).unwrap();
        assert_eq!(hand.type_of_hand(), Type::FiveOfAKind);
    }
}
