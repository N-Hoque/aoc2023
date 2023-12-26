use std::{cmp::Ordering, collections::HashMap};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Card {
    Ace,
    Number(u8),
    Ten,
    Jack,
    Queen,
    King,
}

impl Card {
    // Since I'm not implementing PartialOrd/Ord, I can't implement Hash either.
    const fn value(self) -> u8 {
        match self {
            Self::Ace => 1,
            Self::Number(x) => x,
            Self::Ten => 10,
            Self::Jack => 11,
            Self::Queen => 12,
            Self::King => 13,
        }
    }

    // I'm using these lints simply because I like it conforming to the
    // standard `cmp` signature. I could have this struct derive PartialOrd
    // and Ord, but it gets a bit awkward to use for part two so I'm leaving
    // it like this until I can think of something better.
    #[allow(clippy::match_same_arms, clippy::trivially_copy_pass_by_ref)]
    fn cmp(self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::Number(x), Self::Number(y)) => x.cmp(y),
            (Self::Number(_), _) => Ordering::Less,

            (Self::Ten, Self::Number(_)) => Ordering::Greater,
            (Self::Ten, Self::Ten) => Ordering::Equal,
            (Self::Ten, _) => Ordering::Less,

            (Self::Jack, Self::Number(_) | Self::Ten) => Ordering::Greater,
            (Self::Jack, Self::Jack) => Ordering::Equal,
            (Self::Jack, _) => Ordering::Less,

            (Self::Queen, Self::Queen) => Ordering::Equal,
            (Self::Queen, Self::King | Self::Ace) => Ordering::Less,
            (Self::Queen, _) => Ordering::Greater,

            (Self::King, Self::King) => Ordering::Equal,
            (Self::King, Self::Ace) => Ordering::Less,
            (Self::King, _) => Ordering::Greater,

            (Self::Ace, Self::Ace) => Ordering::Equal,
            (Self::Ace, _) => Ordering::Greater,
        }
    }

    #[allow(clippy::match_same_arms, clippy::trivially_copy_pass_by_ref)]
    fn cmp_jack(self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::Jack, Self::Jack) => Ordering::Equal,
            (Self::Jack, _) => Ordering::Less,

            (Self::Number(_), Self::Jack) => Ordering::Greater,
            (Self::Number(x), Self::Number(y)) => x.cmp(y),
            (Self::Number(_), _) => Ordering::Less,

            (Self::Ten, Self::Number(_) | Self::Jack) => Ordering::Greater,
            (Self::Ten, Self::Ten) => Ordering::Equal,
            (Self::Ten, _) => Ordering::Less,

            (Self::Queen, Self::Queen) => Ordering::Equal,
            (Self::Queen, Self::King | Self::Ace) => Ordering::Less,
            (Self::Queen, _) => Ordering::Greater,

            (Self::King, Self::King) => Ordering::Equal,
            (Self::King, Self::Ace) => Ordering::Less,
            (Self::King, _) => Ordering::Greater,

            (Self::Ace, Self::Ace) => Ordering::Equal,
            (Self::Ace, _) => Ordering::Greater,
        }
    }
}

impl TryFrom<char> for Card {
    type Error = String;

    #[allow(clippy::cast_possible_truncation)]
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' => Ok(Self::Ace),
            'K' => Ok(Self::King),
            'T' => Ok(Self::Ten),
            'Q' => Ok(Self::Queen),
            'J' => Ok(Self::Jack),
            '2'..='9' => Ok(Self::Number(value.to_digit(10).unwrap() as u8)),
            _ => Err(format!("Invalid char detected {value}")),
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct Hand {
    cards: [Card; 5],
    bid: u64,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Strength {
    High,
    One,
    Two,
    Three,
    Full,
    Four,
    Five,
}

impl Hand {
    const fn new(cards: [Card; 5], bid: u64) -> Self {
        Self { cards, bid }
    }

    fn strength(&self) -> Strength {
        let card_matches = self.count_card_matches();

        let unique_cards = card_matches.len();
        let highest_match = card_matches.values().max().unwrap();

        match (unique_cards, highest_match) {
            (1, _) => Strength::Five,
            (2, 4) => Strength::Four,
            (2, 3) => Strength::Full,
            (3, 3) => Strength::Three,
            (3, 2) => Strength::Two,
            (4, _) => Strength::One,
            (5, _) => Strength::High,
            _ => unreachable!("({unique_cards}, {highest_match}) is not a valid pair"),
        }
    }

    fn strength_jack(&self) -> Strength {
        let card_matches = self.count_card_matches();

        let unique_cards = card_matches.len();
        let highest_match = card_matches.values().max().unwrap();
        let num_jacks = card_matches.get(&11);

        match (unique_cards, highest_match, num_jacks) {
            (1, _, _) | (2, _, Some(_)) => Strength::Five,
            (2, 4, None) | (3, 3, Some(_)) | (3, 2, Some(2)) => Strength::Four,
            (2, 3, None) | (3, 2, Some(1)) => Strength::Full,
            (3, 3, None) | (4, _, Some(_)) => Strength::Three,
            (3, 2, None) => Strength::Two,
            (4, _, None) | (5, _, Some(_)) => Strength::One,
            (5, _, None) => Strength::High,
            _ => panic!("({unique_cards}, {highest_match}, {num_jacks:?}) hasn't been accounted"),
        }
    }

    fn count_card_matches(&self) -> HashMap<u8, i32> {
        let mut card_matches = HashMap::new();

        for card in self.cards {
            card_matches
                .entry(card.value())
                .and_modify(|c| *c += 1)
                .or_insert(1);
        }

        card_matches
    }

    fn cmp(&self, other: &Self) -> Ordering {
        match self.strength().cmp(&other.strength()) {
            Ordering::Equal => {}
            ord => return ord,
        }
        for (x, y) in self.cards.iter().zip(&other.cards) {
            match x.cmp(y) {
                Ordering::Equal => {}
                ord => return ord,
            }
        }
        Ordering::Equal
    }

    fn cmp_jack(&self, other: &Self) -> Ordering {
        match self.strength_jack().cmp(&other.strength_jack()) {
            Ordering::Equal => {}
            ord => return ord,
        }
        for (x, y) in self.cards.iter().zip(&other.cards) {
            match x.cmp_jack(y) {
                Ordering::Equal => {}
                ord => return ord,
            }
        }
        Ordering::Equal
    }
}

pub struct Solver {
    hands: Vec<Hand>,
}

impl Solver {
    #[must_use]
    pub fn new(data: Vec<String>) -> Self {
        let mut hands = Vec::new();

        for line in data {
            let (cards_str, bid) = line.split_once(' ').unwrap();
            let mut cards = [Card::Ace; 5];
            for (idx, card) in cards_str.chars().enumerate() {
                cards[idx] = Card::try_from(card).unwrap();
            }
            let bid = bid.parse().unwrap();
            let hand = Hand::new(cards, bid);
            hands.push(hand);
        }

        Self { hands }
    }

    #[must_use]
    pub fn solve_first(&self) -> u64 {
        let mut hands = self.hands.clone();
        hands.sort_by(Hand::cmp);
        hands
            .into_iter()
            .enumerate()
            .fold(0, |acc, (idx, hand)| acc + (hand.bid * (idx as u64 + 1)))
    }

    #[must_use]
    pub fn solve_second(&self) -> u64 {
        let mut hands = self.hands.clone();
        hands.sort_by(Hand::cmp_jack);
        hands
            .into_iter()
            .enumerate()
            .fold(0, |acc, (idx, hand)| acc + (hand.bid * (idx as u64 + 1)))
    }
}

#[test]
pub fn test_solve_first() {
    let data = vec![
        "32T3K 765".to_string(),
        "T55J5 684".to_string(),
        "KK677 28".to_string(),
        "KTJJT 220".to_string(),
        "QQQJA 483".to_string(),
    ];

    let solver = Solver::new(data);
    assert_eq!(6440, solver.solve_first());
}

#[test]
pub fn test_solve_second() {
    let data = vec![
        "32T3K 765".to_string(),
        "T55J5 684".to_string(),
        "KK677 28".to_string(),
        "KTJJT 220".to_string(),
        "QQQJA 483".to_string(),
    ];

    let solver = Solver::new(data);
    assert_eq!(5905, solver.solve_second());
}
