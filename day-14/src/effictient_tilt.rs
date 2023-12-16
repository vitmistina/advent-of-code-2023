use crate::{Direction, Dish, Space, Span};

impl Dish {
    pub fn tilt_efficiently(&mut self, direction: &Direction) {
        if *direction == Direction::North || *direction == Direction::South {
            for x in 0..self.data[0].len() {
                // element number x from each row
                let column = &self.data.iter().map(|row| row[x]).collect::<Vec<_>>();
                let current_spans = &self.col_spans[x];
                let new_vector = create_new_vector(current_spans, column, direction);

                // take each
                self.data
                    .iter_mut()
                    .enumerate()
                    .for_each(|(index, row)| row[x] = new_vector[index]);
            }
        } else {
            for y in 0..self.data.len() {
                let current_spans = &self.row_spans[y];
                let new_vector = create_new_vector(current_spans, &self.data[y], direction);
                self.data[y] = new_vector;
            }
        }
    }
}

fn create_new_vector(
    current_spans: &Vec<Span>,
    vector: &Vec<Space>,
    direction: &Direction,
) -> Vec<Space> {
    let mut new_vector = Vec::new();
    let mut current_loc = 0;
    for span in current_spans {
        while span.start > current_loc {
            new_vector.push(Space::Cube);
            current_loc += 1;
        }
        let count = span.count_round(vector);
        let mut new_span_vec = span.create_new_vector(&count);
        if *direction == Direction::South || *direction == Direction::East {
            new_span_vec.reverse()
        }
        current_loc += new_span_vec.len();
        new_vector = [new_vector, new_span_vec].concat();
    }
    while vector.len() != new_vector.len() {
        new_vector.push(Space::Cube);
    }
    new_vector
}

impl Span {
    fn create_new_vector(&self, round_count: &usize) -> Vec<Space> {
        (0..self.len)
            .map(|i| {
                if i < *round_count {
                    Space::Round
                } else {
                    Space::Empty
                }
            })
            .collect()
    }

    fn count_round(&self, vector: &Vec<Space>) -> usize {
        vector[self.start..self.start + self.len]
            .iter()
            .filter_map(|space| {
                if *space == Space::Round {
                    Some(1)
                } else {
                    None
                }
            })
            .sum()
    }
}

#[test]
fn creates_new_vector() {
    let span = Span { start: 0, len: 3 };
    assert_eq!(
        span.create_new_vector(&2),
        vec![Space::Round, Space::Round, Space::Empty,]
    )
}

#[test]
fn counts_round_in_span() {
    let vector = &vec![
        Space::Round,
        Space::Empty,
        Space::Empty,
        Space::Cube,
        Space::Round,
        Space::Round,
    ];
    let span = Span { start: 0, len: 3 };
    assert_eq!(span.count_round(vector), 1);

    let span = Span { start: 4, len: 2 };
    assert_eq!(span.count_round(vector), 2);
}

#[test]
fn tilts_north_efficently() {
    let input = "O.#..#
O.OO#.";
    let mut dish = Dish::parse(input);
    dish.tilt_efficiently(&Direction::North);
    let expected_dish = Dish {
        data: vec![
            vec![
                Space::Round,
                Space::Empty,
                Space::Cube,
                Space::Round,
                Space::Empty,
                Space::Cube,
            ],
            vec![
                Space::Round,
                Space::Empty,
                Space::Round,
                Space::Empty,
                Space::Cube,
                Space::Empty,
            ],
        ],
        col_spans: Vec::new(),
        row_spans: Vec::new(),
    };
    assert_eq!(dish.data, expected_dish.data);
}

#[test]
fn tilts_west_efficently() {
    let input = "O.#..#
O.OO#.";
    let mut dish = Dish::parse(input);
    dish.tilt_efficiently(&Direction::West);
    let expected_dish = Dish {
        data: vec![
            vec![
                Space::Round,
                Space::Empty,
                Space::Cube,
                Space::Empty,
                Space::Empty,
                Space::Cube,
            ],
            vec![
                Space::Round,
                Space::Round,
                Space::Round,
                Space::Empty,
                Space::Cube,
                Space::Empty,
            ],
        ],
        col_spans: Vec::new(),
        row_spans: Vec::new(),
    };
    assert_eq!(dish.data, expected_dish.data);
}
