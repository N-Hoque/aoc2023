use std::collections::HashMap;

struct Symbol {
    symbol: char,
    position: (usize, usize),
}

fn find_symbol(row_idx: usize, col_idx: usize, data: &[Vec<char>]) -> Option<Symbol> {
    let indices = [
        (row_idx.checked_sub(1), col_idx.checked_sub(1)),
        (row_idx.checked_sub(1), Some(col_idx)),
        (row_idx.checked_sub(1), col_idx.checked_add(1)),
        (Some(row_idx), col_idx.checked_sub(1)),
        (Some(row_idx), col_idx.checked_add(1)),
        (row_idx.checked_add(1), col_idx.checked_sub(1)),
        (row_idx.checked_add(1), Some(col_idx)),
        (row_idx.checked_add(1), col_idx.checked_add(1)),
    ];

    for (row_idx, col_idx) in indices {
        if let (Some(valid_row), Some(valid_col)) = (row_idx, col_idx) {
            if let Some(row) = data.get(valid_row) {
                if let Some(c) = row.get(valid_col) {
                    if !c.is_ascii_digit() && *c != '.' {
                        return Some(Symbol {
                            symbol: *c,
                            position: (valid_row, valid_col),
                        });
                    }
                }
            }
        }
    }

    None
}

pub struct Solver {
    data: Vec<Vec<char>>,
}

impl Solver {
    #[must_use]
    pub fn new(data: Vec<String>) -> Self {
        // Convert lines into grid of characters

        let mut rows = Vec::new();
        for line in data {
            let chars = line.chars().collect::<Vec<_>>();
            rows.push(chars);
        }

        Self { data: rows }
    }

    #[must_use]
    pub fn solve_first(&self) -> u32 {
        self.scan_part_numbers()
            .into_iter()
            .fold(0, |acc, (s, v)| acc + if s.is_some() { v } else { 0 })
    }

    #[must_use]
    pub fn solve_second(&self) -> u32 {
        // Find all gears
        let gears = self.scan_part_numbers().into_iter().filter_map(|(s, v)| {
            if let Some(Symbol {
                symbol: '*',
                position,
            }) = s
            {
                Some((position, v))
            } else {
                None
            }
        });

        // Find all part numbers next to gears
        let mut pairs: HashMap<(usize, usize), (u32, u32)> = HashMap::new();
        for (position, value) in gears {
            pairs
                .entry(position)
                .and_modify(|x| x.1 = value)
                .or_insert((value, 0));
        }

        pairs.values().fold(0, |acc, (v0, v1)| acc + (v0 * v1))
    }

    fn scan_part_numbers(&self) -> Vec<(Option<Symbol>, u32)> {
        let mut part_numbers = Vec::new();

        for (row_idx, row) in self.data.iter().enumerate() {
            let mut current_part_numbers = Vec::new();

            let (mut possible_part_number, mut symbol) = (0, None);

            for (col_idx, col_char) in row.iter().enumerate() {
                if let Some(digit) = col_char.to_digit(10) {
                    possible_part_number = possible_part_number * 10 + digit;
                    let possible_symbol = find_symbol(row_idx, col_idx, &self.data);
                    if possible_symbol.is_some() {
                        symbol = possible_symbol;
                    }
                } else if !col_char.is_ascii_digit() || col_idx == row.len() - 1 {
                    current_part_numbers.push((symbol, possible_part_number));
                    possible_part_number = 0;
                    symbol = None;
                }
            }

            part_numbers.extend(current_part_numbers.into_iter());
        }

        part_numbers
    }
}

#[test]
fn test_solve_first() {
    let data = vec![
        "467..114..".to_string(),
        "...*......".to_string(),
        "..35..633.".to_string(),
        "......#...".to_string(),
        "617*......".to_string(),
        ".....+.58.".to_string(),
        "..592.....".to_string(),
        "......755.".to_string(),
        "...$.*....".to_string(),
        ".664.598..".to_string(),
    ];

    let solver = Solver::new(data);
    assert_eq!(4361, solver.solve_first());
}

#[test]
fn test_solve_second() {
    let data = vec![
        "467..114..".to_string(),
        "...*......".to_string(),
        "..35..633.".to_string(),
        "......#...".to_string(),
        "617*......".to_string(),
        ".....+.58.".to_string(),
        "..592.....".to_string(),
        "......755.".to_string(),
        "...$.*....".to_string(),
        ".664.598..".to_string(),
    ];

    let solver = Solver::new(data);
    assert_eq!(467_835, solver.solve_second());
}
