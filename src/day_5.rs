pub(super) mod parser;

use itertools::Itertools;
use winnow::Parser;

use crate::day_5::parser::{parse_all_maps, parse_seeds};

pub struct Solver {
    seeds: Vec<u64>,
    layers: Vec<Layer>,
}

impl Solver {
    #[must_use]
    pub fn new(data: Vec<String>) -> Self {
        let data = data.into_iter().join("\n");

        let mut data = data.as_str();

        let (seeds, layers) = (parse_seeds, parse_all_maps).parse_next(&mut data).unwrap();

        Self { seeds, layers }
    }

    #[must_use]
    pub fn solve_first(&self) -> u64 {
        self.seeds
            .iter()
            .map(|seed| self.seed_location(*seed))
            .min()
            .expect("finding minimum of non-empty seed list")
    }

    fn seed_location(&self, seed: u64) -> u64 {
        self.layers
            .iter()
            .fold(seed, |follow, current| current.destination(follow))
    }

    #[must_use]
    pub fn solve_second(&self) -> u64 {
        let seed_ranges = self.make_seed_ranges();

        self.find_min_location(seed_ranges)
    }

    fn make_seed_ranges(&self) -> Vec<(u64, u64)> {
        let mut seed_ranges = Vec::new();

        for seed_range in &self.seeds.iter().chunks(2) {
            let seed_range = seed_range.collect::<Vec<_>>();
            let (seed_source, seed_range) = (*seed_range[0], *seed_range[1]);

            seed_ranges.push((seed_source, seed_source + seed_range));
        }
        seed_ranges
    }

    fn find_min_location(&self, mut seed_ranges: Vec<(u64, u64)>) -> u64 {
        let mut current_min_location = u64::MAX;

        while !seed_ranges.is_empty() {
            let (start_seed, end_seed) = seed_ranges[0];

            let start_trace = self.seed_trace(start_seed);
            let end_trace = self.seed_trace(end_seed);

            let min_location = Iterator::zip(start_trace.iter(), end_trace.iter())
                .last()
                .map(|((l0, _), (l1, _))| l0.min(l1))
                .unwrap();

            if *min_location < current_min_location {
                current_min_location = *min_location;
            }

            self.update_seed_ranges(
                &mut seed_ranges,
                &start_trace,
                &end_trace,
                start_seed,
                end_seed,
            );
        }

        current_min_location
    }

    // Track seed as it passes through every layer
    fn seed_trace(&self, seed: u64) -> Vec<(u64, Option<usize>)> {
        let mut traces = Vec::with_capacity(7);

        let mut follow = seed;

        for layer in &self.layers {
            let trace = layer.trace(follow);
            traces.push(trace);
            follow = trace.0;
        }

        traces
    }

    fn update_seed_ranges(
        &self,
        seed_ranges: &mut Vec<(u64, u64)>,
        start_trace: &[(u64, Option<usize>)],
        end_trace: &[(u64, Option<usize>)],
        start_seed: u64,
        end_seed: u64,
    ) {
        seed_ranges.remove(0);

        if !Self::are_traces_matching(start_trace, end_trace) {
            let mid_seed = (start_seed + end_seed) / 2;
            let mid_trace = self.seed_trace(mid_seed);
            let left_match = Self::are_traces_matching(start_trace, &mid_trace);
            let right_match = Self::are_traces_matching(&mid_trace, end_trace);

            if left_match {
                seed_ranges.insert(0, (mid_seed + 1, end_seed));
            } else if right_match {
                seed_ranges.insert(0, (start_seed, mid_seed));
            } else {
                seed_ranges.insert(0, (start_seed, mid_seed + 1));
                seed_ranges.insert(0, (mid_seed, end_seed));
            }
        }
    }

    fn are_traces_matching(
        left_trace: &[(u64, Option<usize>)],
        right_trace: &[(u64, Option<usize>)],
    ) -> bool {
        Iterator::zip(left_trace.iter(), right_trace.iter())
            .all(|((_, first), (_, second))| first == second)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Mapping {
    source: u64,
    destination: u64,
    range: u64,
}

impl Mapping {
    fn find_destination(&self, source: u64) -> Option<u64> {
        if (self.source..(self.source + self.range)).contains(&source) {
            Some(self.destination + (source - self.source))
        } else {
            None
        }
    }
}

struct Layer(Vec<Mapping>);

impl Layer {
    pub fn destination(&self, source: u64) -> u64 {
        self.0
            .iter()
            .find_map(|map| map.find_destination(source))
            .unwrap_or(source)
    }

    // Track which range did the mapping for this layer
    pub fn trace(&self, source: u64) -> (u64, Option<usize>) {
        self.0
            .iter()
            .enumerate()
            .find_map(|(idx, map)| {
                map.find_destination(source)
                    .map(|destination| (destination, Some(idx)))
            })
            .unwrap_or((source, None))
    }
}

#[test]
fn test_solve_first() {
    let data = &self::parser::TEST_DATA;

    let solver = Solver::new(
        data.iter()
            .map(std::string::ToString::to_string)
            .collect::<Vec<_>>(),
    );

    assert_eq!(35, solver.solve_first());
}

#[test]
fn test_solve_second() {
    let data = &self::parser::TEST_DATA;

    let solver = Solver::new(
        data.iter()
            .map(std::string::ToString::to_string)
            .collect::<Vec<_>>(),
    );

    assert_eq!(46, solver.solve_second());
}
