pub struct Solver {
    map: Vec<Vec<Segment>>,
    start_point: (isize, isize),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Corner {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Segment {
    Horizontal,
    Vertical,
    Ground,
    Start,
    Corner(Corner),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Orientation {
    Up,
    Down,
    Left,
    Right,
}

impl Orientation {
    const fn value(self) -> (isize, isize) {
        match self {
            Self::Right => (0, 1),
            Self::Left => (0, -1),
            Self::Up => (-1, 0),
            Self::Down => (1, 0),
        }
    }

    const fn rotate(self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }

    const fn try_update(self, segment: Segment) -> Option<Self> {
        match (self, segment) {
            (Self::Up | Self::Down, Segment::Horizontal)
            | (Self::Left | Self::Right, Segment::Vertical)
            | (_, Segment::Ground) => None,

            (Self::Up | Self::Down, Segment::Vertical | Segment::Start)
            | (Self::Left | Self::Right, Segment::Horizontal | Segment::Start) => Some(self),

            (Self::Up, Segment::Corner(c)) => match c {
                Corner::TopLeft => Some(Self::Right),
                Corner::TopRight => Some(Self::Left),
                _ => None,
            },
            (Self::Down, Segment::Corner(c)) => match c {
                Corner::BottomLeft => Some(Self::Right),
                Corner::BottomRight => Some(Self::Left),
                _ => None,
            },
            (Self::Left, Segment::Corner(c)) => match c {
                Corner::TopLeft => Some(Self::Down),
                Corner::BottomLeft => Some(Self::Up),
                _ => None,
            },
            (Self::Right, Segment::Corner(c)) => match c {
                Corner::TopRight => Some(Self::Down),
                Corner::BottomRight => Some(Self::Up),
                _ => None,
            },
        }
    }
}

impl From<(isize, isize)> for Orientation {
    fn from(value: (isize, isize)) -> Self {
        match value {
            (0, 1) => Self::Right,
            (0, -1) => Self::Left,
            (-1, 0) => Self::Up,
            (1, 0) => Self::Down,
            _ => unreachable!("Got {value:?}"),
        }
    }
}

impl std::fmt::Display for Segment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Horizontal => write!(f, "-"),
            Self::Vertical => write!(f, "|"),
            Self::Ground => write!(f, "."),
            Self::Start => write!(f, "S"),
            Self::Corner(c) => match c {
                Corner::TopLeft => write!(f, "F"),
                Corner::TopRight => write!(f, "7"),
                Corner::BottomLeft => write!(f, "L"),
                Corner::BottomRight => write!(f, "J"),
            },
        }
    }
}

impl TryFrom<char> for Segment {
    type Error = String;

    fn try_from(segment: char) -> Result<Self, Self::Error> {
        match segment {
            'S' => Ok(Self::Start),
            '-' => Ok(Self::Horizontal),
            '|' => Ok(Self::Vertical),
            'F' => Ok(Self::Corner(Corner::TopLeft)),
            '7' => Ok(Self::Corner(Corner::TopRight)),
            'L' => Ok(Self::Corner(Corner::BottomLeft)),
            'J' => Ok(Self::Corner(Corner::BottomRight)),
            '.' => Ok(Self::Ground),
            _ => Err(format!("{segment} should not be parsed")),
        }
    }
}

impl Solver {
    #[must_use]
    pub fn new(data: Vec<String>) -> Self {
        let mut map = Vec::new();

        let mut start_point = (0, 0);

        for (row, line) in data.iter().enumerate() {
            let mut segments = Vec::new();
            for (col, segment) in line.chars().enumerate() {
                let segment = Segment::try_from(segment).unwrap();
                segments.push(segment);
                if segment == Segment::Start {
                    start_point = (row as isize, col as isize);
                }
            }
            map.push(segments);
        }

        Self { map, start_point }
    }

    #[must_use]
    pub fn solve_first(&self) -> usize {
        let main_loop = self.find_loop();

        main_loop.len() / 2
    }

    #[must_use]
    pub fn solve_second(&self) -> u64 {
        // A point is contained within the loop if, in every direction from that point, you hit the loop.

        let mut internal_area = 0;

        let main_loop = self.find_loop();

        internal_area
    }

    fn find_initial_orientation(&self) -> Option<Orientation> {
        for offset in [(-1, 0), (0, 1), (1, 0), (0, -1)] {
            let next_position = (self.start_point.0 + offset.0, self.start_point.1 + offset.1);

            if next_position.0 < 0
                || next_position.1 < 0
                || next_position.0 as usize >= self.map.len()
                || next_position.1 as usize >= self.map[0].len()
            {
                continue;
            }

            match self
                .map
                .get(next_position.0 as usize)
                .and_then(|row| row.get(next_position.1 as usize))
            {
                None | Some(Segment::Ground) => {}
                Some(Segment::Start) => unreachable!("There should be only one start point"),
                Some(_) => return Some(Orientation::from(offset)),
            }
        }

        None
    }

    fn find_loop(&self) -> Vec<(&Segment, (isize, isize))> {
        let mut current_orientation = self.find_initial_orientation().unwrap();

        let mut current_position = self.start_point;

        let mut segments = Vec::new();

        loop {
            match self
                .map
                .get(current_position.0 as usize)
                .and_then(|row| row.get(current_position.1 as usize))
            {
                None => {}
                Some(Segment::Start) if !segments.is_empty() => break,
                Some(current_segment) => match current_orientation.try_update(*current_segment) {
                    None => {
                        // If the next piece makes no sense physically, rotate orientation until you find a valid piece
                        current_orientation = current_orientation.rotate();
                        continue;
                    }
                    Some(next_orientation) => {
                        segments.push((current_segment, current_position));
                        current_orientation = next_orientation;
                        let offset = current_orientation.value();
                        current_position =
                            (current_position.0 + offset.0, current_position.1 + offset.1);
                    }
                },
            }
        }

        segments
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use crate::day_10::Solver;

    #[test]
    pub fn solve_first() {
        let data = vec![
            ".....".to_string(),
            ".S-7.".to_string(),
            ".|.|.".to_string(),
            ".L-J.".to_string(),
            ".....".to_string(),
        ];

        let solver = Solver::new(data);
        assert_eq!(4, solver.solve_first());
    }

    #[rstest]
    #[case(vec![
        "...........".to_string(),
        ".S-------7.".to_string(),
        ".|F-----7|.".to_string(),
        ".||.....||.".to_string(),
        ".||.....||.".to_string(),
        ".|L-7.F-J|.".to_string(),
        ".|..|.|..|.".to_string(),
        ".L--J.L--J.".to_string(),
        "...........".to_string(),
    ], 4)]
    #[case(vec![
        ".F----7F7F7F7F-7....".to_string(),
        ".|F--7||||||||FJ....".to_string(),
        ".||.FJ||||||||L7....".to_string(),
        "FJL7L7LJLJ||LJ.L-7..".to_string(),
        "L--J.L7...LJS7F-7L7.".to_string(),
        "....F-J..F7FJ|L7L7L7".to_string(),
        "....L7.F7||L7|.L7L7|".to_string(),
        ".....|FJLJ|FJ|F7|.LJ".to_string(),
        "....FJL-7.||.||||...".to_string(),
        "....L---J.LJ.LJLJ...".to_string(),
    ], 8)]
    #[case(vec![
        "FF7FSF7F7F7F7F7F---7".to_string(),
        "L|LJ||||||||||||F--J".to_string(),
        "FL-7LJLJ||||||LJL-77".to_string(),
        "F--JF--7||LJLJ7F7FJ-".to_string(),
        "L---JF-JLJ.||-FJLJJ7".to_string(),
        "|F|F-JF---7F7-L7L|7|".to_string(),
        "|FFJF7L7F-JF7|JL---7".to_string(),
        "7-L-JL7||F7|L7F-7F7|".to_string(),
        "L.L7LFJ|||||FJL7||LJ".to_string(),
        "L7JLJL-JLJLJL--JLJ.L".to_string(),
    ], 10)]
    pub fn solve_second(#[case] input: Vec<String>, #[case] expected: u64) {
        let solver = Solver::new(input);
        assert_eq!(expected, solver.solve_second());
    }
}
