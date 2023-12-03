#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Color {
    Red,
    Green,
    Blue,
}

#[derive(Clone, PartialEq, Eq)]
pub struct Cube {
    color: Color,
    value: u32,
}

pub struct Bag(Vec<Cube>);
pub struct Game(Vec<Bag>);

pub struct Solver {
    data: Vec<Game>,
}

impl Solver {
    #[must_use]
    pub fn new(data: Vec<String>) -> Self {
        Self {
            data: parse_source(data),
        }
    }

    #[must_use]
    pub fn solve_first(&self) -> usize {
        const MAX_RED: u32 = 12;
        const MAX_GREEN: u32 = 13;
        const MAX_BLUE: u32 = 14;

        let mut id_sum = 0;

        for (game_id, game) in self.data.iter().enumerate() {
            let (red_max, green_max, blue_max) = find_maxima(game);
            if red_max <= MAX_RED && green_max <= MAX_GREEN && blue_max <= MAX_BLUE {
                id_sum += game_id + 1;
            }
        }

        id_sum
    }

    #[must_use]
    pub fn solve_second(&self) -> u32 {
        self.data.iter().fold(0, |acc, game| {
            let (red_max, green_max, blue_max) = find_maxima(game);

            acc + (red_max * green_max * blue_max)
        })
    }
}

fn find_maxima(game: &Game) -> (u32, u32, u32) {
    let (mut red_max, mut green_max, mut blue_max) = (0, 0, 0);
    for bag in &game.0 {
        for cube in &bag.0 {
            match cube.color {
                Color::Red if cube.value > red_max => {
                    red_max = cube.value;
                }
                Color::Green if cube.value > green_max => {
                    green_max = cube.value;
                }
                Color::Blue if cube.value > blue_max => {
                    blue_max = cube.value;
                }
                _ => {}
            }
        }
    }
    (red_max, green_max, blue_max)
}

impl<'a> TryFrom<&'a str> for Color {
    type Error = String;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        match value {
            "red" => Ok(Self::Red),
            "green" => Ok(Self::Green),
            "blue" => Ok(Self::Blue),
            x => Err(format!("{x} is not a valid color")),
        }
    }
}

fn parse_source(data: Vec<String>) -> Vec<Game> {
    data.into_iter()
        .filter_map(|e| parse_bag_entries(&e))
        .map(|e| parse_entry(&e))
        .map(Game)
        .collect()
}

fn parse_entry(entry: &str) -> Vec<Bag> {
    entry.split("; ").map(parse_bag).map(Bag).collect()
}

fn parse_bag_entries(entry: &str) -> Option<String> {
    entry.split_once(": ").map(|(_, entry)| entry.to_owned())
}

fn parse_bag(bag: &str) -> Vec<Cube> {
    bag.split(", ").filter_map(parse_cube).collect::<Vec<_>>()
}

fn parse_cube(cube: &str) -> Option<Cube> {
    cube.split_once(' ')
        .map(|(v, c)| {
            let color = Color::try_from(c);
            let value = v.parse::<u32>();
            (value, color)
        })
        .and_then(|(value, color)| match (value, color) {
            (Ok(value), Ok(color)) => Some(Cube { color, value }),
            _ => None,
        })
}

#[cfg(test)]
mod tests {
    use once_cell::sync::Lazy;

    use super::Solver;

    static TEST_DATA: Lazy<Vec<String>> = Lazy::new(|| {
        vec![
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green".to_string(),
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue".to_string(),
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red".to_string(),
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red".to_string(),
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green".to_string(),
        ]
    });

    #[test]
    fn test_solve_first() {
        let solver = Solver::new(TEST_DATA.clone());

        assert_eq!(8, solver.solve_first());
    }

    #[test]
    fn test_solve_second() {
        let solver = Solver::new(TEST_DATA.clone());

        assert_eq!(2286, solver.solve_second());
    }
}
