pub struct Solver {
    data: Vec<String>,
}

impl Solver {
    #[must_use]
    pub fn new(data: Vec<String>) -> Self {
        Self { data }
    }

    #[must_use]
    pub fn solve_first(&self) -> u32 {
        self.data
            .iter()
            .map(|l| {
                let digits = l.chars().filter_map(|c| c.to_digit(10)).collect::<Vec<_>>();
                digits[0] * 10 + digits[digits.len() - 1]
            })
            .sum()
    }

    #[must_use]
    #[allow(clippy::too_many_lines)]
    pub fn solve_second(&self) -> u32 {
        self.data
            .iter()
            .map(|l| {
                let mut digits = Vec::new();

                let chars = l.chars().collect::<Vec<_>>();

                for (char_idx, next_char) in chars.iter().enumerate() {
                    if let Some(digit) = next_char.to_digit(10) {
                        digits.push(digit);
                    } else {
                        if chars.get(char_idx + 2).is_some() {
                            // Find three letter digits
                            let maybe_word =
                                chars[char_idx..char_idx + 3].iter().collect::<String>();
                            match maybe_word.as_str() {
                                "one" => digits.push(1),
                                "two" => digits.push(2),
                                "six" => digits.push(6),
                                _ => {}
                            }
                        }
                        if chars.get(char_idx + 3).is_some() {
                            // Find four letter digits
                            let maybe_word =
                                chars[char_idx..char_idx + 4].iter().collect::<String>();
                            match maybe_word.as_str() {
                                "zero" => digits.push(0),
                                "four" => digits.push(4),
                                "five" => digits.push(5),
                                "nine" => digits.push(9),
                                _ => {}
                            }
                        }
                        if chars.get(char_idx + 4).is_some() {
                            // Find five letter digits
                            let maybe_word =
                                chars[char_idx..char_idx + 5].iter().collect::<String>();
                            match maybe_word.as_str() {
                                "three" => digits.push(3),
                                "seven" => digits.push(7),
                                "eight" => digits.push(8),
                                _ => {}
                            }
                        }
                    }
                }

                println!("{l} {digits:?}");

                digits[0] * 10 + digits[digits.len() - 1]
            })
            .sum()
    }
}

#[test]
fn test_solve_first() {
    let data = vec![
        "1abc2".to_string(),
        "pqr3stu8vwx".to_string(),
        "a1b2c3d4e5f".to_string(),
        "treb7uchet".to_string(),
    ];

    let solver = Solver::new(data);
    assert_eq!(142, solver.solve_first());
}

#[test]
fn test_solve_second() {
    let data = vec![
        "two1nine".to_string(),
        "eightwothree".to_string(),
        "abcone2threexyz".to_string(),
        "xtwone3four".to_string(),
        "4nineeightseven2".to_string(),
        "zoneight234".to_string(),
        "7pqrstsixteen".to_string(),
    ];

    let solver = Solver::new(data);
    assert_eq!(281, solver.solve_second());
}
