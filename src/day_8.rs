use std::collections::{HashMap, HashSet};

use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Direction {
    Left,
    Right,
}

pub struct Solver {
    order: Vec<Direction>,
    nodes: HashMap<String, (String, String)>,
}

impl Solver {
    #[must_use]
    pub fn new(data: Vec<String>) -> Self {
        let order = data[0].clone();
        let order = order
            .chars()
            .map(|c| {
                if c == 'L' {
                    Direction::Left
                } else {
                    Direction::Right
                }
            })
            .collect();
        let mut nodes = HashMap::new();
        for node_line in &data[2..] {
            let (start, nodes_str) = node_line.split_once(" = ").unwrap();
            let (left, right) = nodes_str
                .strip_prefix('(')
                .and_then(|x| x.strip_suffix(')'))
                .and_then(|x| x.split(", ").collect_tuple::<(&str, &str)>())
                .unwrap();

            nodes
                .entry(start.to_string())
                .or_insert((left.to_string(), right.to_string()));
        }
        Self { order, nodes }
    }

    #[must_use]
    pub fn solve_first(&self) -> u64 {
        self.count_steps("AAA", |node| node == "ZZZ")
    }

    #[must_use]
    pub fn solve_second(&self) -> u64 {
        let node_steps = self
            .nodes
            .keys()
            .filter(|node| node.ends_with('A'))
            .map(|node| self.count_steps(node, |node| node.ends_with('Z')));

        lcm(node_steps)
    }

    fn next_node(&self, current_node: &str, direction: Direction) -> &String {
        self.nodes
            .get(current_node)
            .map(|(left, right)| match direction {
                Direction::Left => left,
                Direction::Right => right,
            })
            .unwrap()
    }

    fn count_steps(&self, initial_node: &str, node_check: fn(&str) -> bool) -> u64 {
        let mut current_node = initial_node;

        let mut counter = 0;

        for direction in self.order.iter().cycle() {
            current_node = self.next_node(current_node, *direction);
            counter += 1;
            if node_check(current_node) {
                break;
            }
        }

        counter
    }
}

fn factorize(mut n: u64) -> HashSet<u64> {
    let mut factors = HashSet::new();
    while n > 0 && n % 2 == 0 {
        factors.insert(2);
        n /= 2;
    }

    for factor in (3..=n).step_by(2) {
        while n % factor == 0 {
            factors.insert(factor);
            n /= factor;
            if n == 0 {
                break;
            }
        }
    }

    factors
}

fn lcm(numbers: impl Iterator<Item = u64>) -> u64 {
    numbers
        .map(factorize)
        .fold(HashSet::new(), |acc, set| {
            acc.union(&set).copied().collect()
        })
        .into_iter()
        .product()
}

#[test]
pub fn test_solve_first() {
    let data = vec![
        "RL".to_string(),
        String::new(),
        "AAA = (BBB, CCC)".to_string(),
        "BBB = (DDD, EEE)".to_string(),
        "CCC = (ZZZ, GGG)".to_string(),
        "DDD = (DDD, DDD)".to_string(),
        "EEE = (EEE, EEE)".to_string(),
        "GGG = (GGG, GGG)".to_string(),
        "ZZZ = (ZZZ, ZZZ)".to_string(),
    ];

    let solver = Solver::new(data);
    assert_eq!(2, solver.solve_first());
}

#[test]
pub fn test_solve_second() {
    let data = vec![
        "LR".to_string(),
        String::new(),
        "11A = (11B, XXX)".to_string(),
        "11B = (XXX, 11Z)".to_string(),
        "11Z = (11B, XXX)".to_string(),
        "22A = (22B, XXX)".to_string(),
        "22B = (22C, 22C)".to_string(),
        "22C = (22Z, 22Z)".to_string(),
        "22Z = (22B, 22B)".to_string(),
        "XXX = (XXX, XXX)".to_string(),
    ];

    let solver = Solver::new(data);
    assert_eq!(6, solver.solve_second());
}
