use crate::{Dish, Space};

impl Space {
    fn from(char: char) -> Self {
        match char {
            'O' => Self::Round,
            '.' => Self::Empty,
            '#' => Self::Cube,
            _ => panic!(),
        }
    }

    pub fn char(&self) -> char {
        match self {
            Self::Round => 'O',
            Self::Empty => '.',
            Self::Cube => '#',
            _ => panic!(),
        }
    }
}

impl Dish {
    pub fn parse(input: &str) -> Self {
        Self {
            data: input
                .lines()
                .map(|line| line.chars().map(|char| Space::from(char)).collect())
                .collect(),
        }
    }
}

#[test]
fn parses_input() {
    let input = "O....#
O.OO#.";
    assert_eq!(
        Dish::parse(&input),
        Dish {
            data: vec![
                vec![
                    Space::Round,
                    Space::Empty,
                    Space::Empty,
                    Space::Empty,
                    Space::Empty,
                    Space::Cube
                ],
                vec![
                    Space::Round,
                    Space::Empty,
                    Space::Round,
                    Space::Round,
                    Space::Cube,
                    Space::Empty
                ]
            ]
        }
    );
}
