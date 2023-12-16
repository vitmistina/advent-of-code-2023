use crate::{Dish, Space, Span};

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
        }
    }
}

impl Dish {
    pub fn parse(input: &str) -> Self {
        let data: Vec<Vec<Space>> = input
            .lines()
            .map(|line| line.chars().map(|char| Space::from(char)).collect())
            .collect();

        let col_spans = (0..data[0].len())
            .map(|x| {
                let spaces = data.iter().map(|row| row[x]).collect::<Vec<_>>();
                get_spans(&spaces)
            })
            .collect();

        let row_spans = data.iter().map(|row| get_spans(&row)).collect();

        Self {
            data,
            col_spans,
            row_spans,
        }
    }
}

fn get_spans(spaces: &Vec<Space>) -> Vec<Span> {
    let mut spans = Vec::new();
    let mut span_start = 0;
    for (index, space) in spaces.iter().enumerate() {
        if index + 1 == spaces.len() || *space == Space::Cube {
            let len = index - span_start + if *space == Space::Cube { 0 } else { 1 };
            if len > 0 {
                spans.push(Span {
                    start: span_start,
                    len,
                });
            };
            span_start = index + 1;
        }
    }
    spans
}

#[test]
fn parses_input() {
    let input = "O....#
O.OO#.";
    let dish = Dish::parse(&input);
    assert_eq!(
        dish.data,
        vec![
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
    );

    assert_eq!(
        dish.col_spans,
        vec![
            vec![Span { start: 0, len: 2 }],
            vec![Span { start: 0, len: 2 }],
            vec![Span { start: 0, len: 2 }],
            vec![Span { start: 0, len: 2 }],
            vec![Span { start: 0, len: 1 }],
            vec![Span { start: 1, len: 1 }]
        ]
    );
    assert_eq!(
        dish.row_spans,
        vec![
            vec![Span { start: 0, len: 5 }],
            vec![Span { start: 0, len: 4 }, Span { start: 5, len: 1 }]
        ]
    );
}
