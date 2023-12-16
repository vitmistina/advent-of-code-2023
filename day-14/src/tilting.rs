use crate::{Direction, Dish, Space};

impl Dish {
    pub fn tilt(&mut self, direction: &Direction) {
        let (row_offset, col_offset) = match direction {
            Direction::North => (-1, 0),
            Direction::South => (1, 0),
            Direction::East => (0, 1),
            Direction::West => (0, -1),
        };

        for row_index in if *direction == Direction::South {
            Box::new((0..self.data.len()).rev()) as Box<dyn Iterator<Item = usize>>
        } else {
            Box::new(0..self.data.len()) as Box<dyn Iterator<Item = usize>>
        } {
            for col_index in if *direction == Direction::East {
                Box::new((0..self.data[0].len()).rev()) as Box<dyn Iterator<Item = usize>>
            } else {
                Box::new(0..self.data[0].len()) as Box<dyn Iterator<Item = usize>>
            } {
                let is_space_round = { self.data[row_index][col_index] == Space::Round };

                if is_space_round {
                    if let Some(target) =
                        self.find_furthest_empty(row_index, col_index, row_offset, col_offset)
                    {
                        self.data[target.0][target.1] = Space::Round;
                        self.data[row_index][col_index] = Space::Empty;
                    };
                }
            }
        }
    }

    fn find_furthest_empty(
        &self,
        row_index: usize,
        col_index: usize,
        row_offset: i32,
        col_offset: i32,
    ) -> Option<(usize, usize)> {
        let mut result = None;
        let mut current_row = row_index;
        let mut current_col = col_index;
        if row_offset != 0 {
            while let Some(new_row) = Self::get_coords(current_row, row_offset, self.data.len()) {
                current_row = new_row;
                if self.data[current_row][current_col] == Space::Empty {
                    result = Some((current_row, current_col));
                } else {
                    return result;
                }
            }
        }
        if col_offset != 0 {
            while let Some(new_col) = Self::get_coords(current_col, col_offset, self.data[0].len())
            {
                current_col = new_col;
                if self.data[current_row][current_col] == Space::Empty {
                    result = Some((current_row, current_col));
                } else {
                    return result;
                }
            }
        }
        result
    }

    fn get_coords(index: usize, offset: i32, len: usize) -> Option<usize> {
        let adjusted_index = if offset.is_negative() {
            index.checked_sub(offset.wrapping_abs() as usize)
        } else {
            index.checked_add(offset as usize)
        };
        match adjusted_index {
            Some(new_index) if new_index < len => Some(new_index),
            _ => None,
        }
    }
}

#[test]
fn finds_empty() {
    let input = Dish {
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
                Space::Empty,
                Space::Round,
                Space::Round,
                Space::Empty,
                Space::Empty,
            ],
        ],
        col_spans: Vec::new(),
        row_spans: Vec::new(),
    };

    assert_eq!(input.find_furthest_empty(1, 3, -1, 0), Some((0, 3)));
    assert_eq!(input.find_furthest_empty(0, 0, 0, 1), Some((0, 1)));
    assert_eq!(input.find_furthest_empty(0, 0, 1, 0), None);
}

#[test]
fn coordinates_properly() {
    assert_eq!(Dish::get_coords(0, -1, 1), None);
    assert_eq!(Dish::get_coords(1, 1, 1), None);
    assert_eq!(Dish::get_coords(0, 1, 2), Some(1))
}

#[test]
fn tilts_north() {
    let input = "O.#..#
O.OO#.";
    let mut dish = Dish::parse(input);
    dish.tilt(&Direction::North);
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
fn tilts_west() {
    let input = "O.#..#
O.OO#.";
    let mut dish = Dish::parse(input);
    dish.tilt(&Direction::West);
    assert_eq!(
        dish.data,
        vec![
            vec![
                Space::Round,
                Space::Empty,
                Space::Cube,
                Space::Empty,
                Space::Empty,
                Space::Cube
            ],
            vec![
                Space::Round,
                Space::Round,
                Space::Round,
                Space::Empty,
                Space::Cube,
                Space::Empty
            ]
        ]
    );
}
