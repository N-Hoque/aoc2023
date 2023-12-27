use itertools::Itertools;

pub struct Solver {
    history: Vec<Vec<i64>>,
}

impl Solver {
    #[must_use]
    pub fn new(data: Vec<String>) -> Self {
        let history = data
            .into_iter()
            .map(|line| {
                line.split_whitespace()
                    .filter_map(|c| c.parse().ok())
                    .collect()
            })
            .collect();

        Self { history }
    }

    #[must_use]
    pub fn solve_first(&self) -> i64 {
        self.history
            .iter()
            .map(|line| build_history(line))
            .filter_map(|history| history.last().and_then(|line| line.last()).copied())
            .sum()
    }

    #[must_use]
    pub fn solve_second(&self) -> i64 {
        self.history
            .iter()
            .map(|line| build_history(line))
            .filter_map(|history| history.last().and_then(|line| line.first()).copied())
            .sum()
    }
}

fn build_history(line: &[i64]) -> Vec<Vec<i64>> {
    let mut updated_history = Vec::new();
    updated_history.push(line.to_vec());
    compute_differences(0, &mut updated_history);
    updated_history.reverse();
    back_propagate(0, &mut updated_history);
    updated_history
}

fn compute_differences(current_line_idx: usize, history: &mut Vec<Vec<i64>>) {
    if let Some(current_line) = history.get(current_line_idx) {
        if current_line.iter().all_equal() {
            return;
        }
        let mut differences = Vec::new();
        for pair in current_line.windows(2) {
            let (a, b) = (pair[0], pair[1]);
            differences.push(b - a);
        }
        history.push(differences.clone());
        compute_differences(current_line_idx + 1, history);
    }
}

fn back_propagate(current_line_idx: usize, history: &mut Vec<Vec<i64>>) {
    if let Some(current_line) = history.get(current_line_idx) {
        let current_first = current_line.first().copied().unwrap();
        let current_last = current_line.last().copied().unwrap();
        if let Some(next_line) = history.get_mut(current_line_idx + 1) {
            let first_value = next_line.first().copied().unwrap();
            let last_value = next_line.last().copied().unwrap();
            next_line.push(last_value + current_last);
            next_line.insert(0, first_value - current_first);
            back_propagate(current_line_idx + 1, history);
        }
    }
}

#[test]
pub fn test_solve_first() {
    let data = vec![
        "0 3 6 9 12 15".to_string(),
        "1 3 6 10 15 21".to_string(),
        "10 13 16 21 30 45".to_string(),
    ];

    let solver = Solver::new(data);
    assert_eq!(114, solver.solve_first());
}

#[test]
pub fn test_solve_second() {
    let data = vec![
        "0 3 6 9 12 15".to_string(),
        "1 3 6 10 15 21".to_string(),
        "10 13 16 21 30 45".to_string(),
    ];

    let solver = Solver::new(data);
    assert_eq!(2, solver.solve_second());
}
