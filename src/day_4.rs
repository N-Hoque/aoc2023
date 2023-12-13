pub struct Solver {
    data: Vec<Card>,
}

#[derive(Debug)]
struct Card {
    winning_values: Vec<u32>,
    all_values: Vec<u32>,
}

impl Card {
    fn get_matches(&self) -> usize {
        self.winning_values
            .iter()
            .filter(|w| self.all_values.contains(w))
            .count()
    }
}

impl Solver {
    #[must_use]
    pub fn new(data: Vec<String>) -> Self {
        let mut cards = Vec::new();

        for card_line in data {
            let Some((_, right)) = card_line.split_once(": ") else {
                panic!("Splitting card info")
            };

            let Some((winning, all)) = right.split_once(" | ") else {
                panic!("Splitting card numbers")
            };

            let winning = winning
                .split_whitespace()
                .filter_map(|v| v.parse::<u32>().ok())
                .collect::<Vec<_>>();

            let all = all
                .split_whitespace()
                .filter_map(|v| v.parse::<u32>().ok())
                .collect::<Vec<_>>();

            let card = Card {
                winning_values: winning,
                all_values: all,
            };

            cards.push(card);
        }

        Self { data: cards }
    }

    #[must_use]
    pub fn solve_first(&self) -> usize {
        self.data
            .iter()
            .map(Card::get_matches)
            .filter(|matches| *matches != 0)
            .fold(0, |acc, matches| {
                acc + 2_usize.pow(matches.saturating_sub(1) as u32)
            })
    }

    #[must_use]
    pub fn solve_second(&self) -> u32 {
        let total_cards = self.data.len();

        let mut card_copies = vec![1; total_cards];

        for (idx, matches) in self.data.iter().map(Card::get_matches).enumerate() {
            let num_copies = *card_copies.get(idx).unwrap_or(&1);
            for offset in 1..=matches {
                if let Some(c) = card_copies.get_mut(idx + offset) {
                    *c += num_copies;
                }
            }
        }

        card_copies.into_iter().sum()
    }
}

#[test]
fn test_solve_first() {
    let data = vec![
        "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53".to_string(),
        "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19".to_string(),
        "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1".to_string(),
        "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83".to_string(),
        "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36".to_string(),
        "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11".to_string(),
    ];

    let solver = Solver::new(data);
    assert_eq!(13, solver.solve_first());
}

#[test]
fn test_solve_second() {
    let data = vec![
        "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53".to_string(),
        "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19".to_string(),
        "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1".to_string(),
        "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83".to_string(),
        "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36".to_string(),
        "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11".to_string(),
    ];

    let solver = Solver::new(data);
    assert_eq!(30, solver.solve_second());
}
