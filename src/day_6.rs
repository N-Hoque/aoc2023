use std::num::ParseIntError;

use itertools::Itertools;

pub struct Solver {
    data: Vec<String>,
}

impl Solver {
    #[must_use]
    pub fn new(data: Vec<String>) -> Self {
        assert_eq!(2, data.len());
        Self { data }
    }

    #[must_use]
    pub fn solve_first(&self) -> u64 {
        let times: Vec<u64> = self
            .transform_list(0, |x| x.split_whitespace().map(str::parse).collect())
            .unwrap();
        let distances: Vec<u64> = self
            .transform_list(1, |x| x.split_whitespace().map(str::parse).collect())
            .unwrap();

        times
            .iter()
            .zip(&distances)
            .fold(1, |acc, (&time, &distance)| {
                acc * run_boat_attempts(time, distance)
            })
    }

    #[must_use]
    pub fn solve_second(&self) -> u64 {
        let time = self
            .transform_list(0, |x| x.split_whitespace().join("").parse())
            .unwrap();
        let distance = self
            .transform_list(1, |x| x.split_whitespace().join("").parse())
            .unwrap();

        run_boat_attempts(time, distance)
    }

    fn transform_list<T>(
        &self,
        index: usize,
        transform: fn(&str) -> Result<T, ParseIntError>,
    ) -> Result<T, ParseIntError> {
        let label = if index == 0 { "Time:" } else { "Distance:" };
        self.data[index]
            .strip_prefix(label)
            .ok_or_else(|| panic!("stripping {label} prefix"))
            .and_then(transform)
    }
}

fn run_boat_attempts(max_time: u64, min_distance: u64) -> u64 {
    let det = ((max_time.pow(2) - 4 * min_distance) as f64).sqrt();

    let min_possible_time = ((max_time as f64 - det) / 2.0).floor();
    let max_possible_time = ((max_time as f64 + det) / 2.0).ceil();

    ((max_possible_time - min_possible_time) - 1.0).round() as u64
}

#[cfg(test)]
mod tests {
    use super::run_boat_attempts;
    use rstest::rstest;

    #[rstest]
    #[case((7, 9), 4)]
    #[case((15, 40), 8)]
    #[case((30, 200), 9)]
    #[case((71530, 940_200), 71503)]
    fn run_race(#[case] input: (u64, u64), #[case] expected: u64) {
        assert_eq!(expected, run_boat_attempts(input.0, input.1));
    }
}
