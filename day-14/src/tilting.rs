use crate::{Direction, Dish, Space};

impl Dish {
    pub fn tilt(&mut self, direction: &Direction) {
        let (row_offset, col_offset) = match direction {
            Direction::North => (-1, 0),
            Direction::South => (1, 0),
            Direction::East => (0, 1),
            Direction::West => (0, -1),
        };
        let mut is_changed = true;
        while is_changed == true {
            is_changed = false;
            for row_index in 0..self.data.len() {
                if let Some(target_row) = Self::get_coords(row_index, row_offset, self.data.len()) {
                    for col_index in 0..self.data[0].len() {
                        if let Some(target_col) =
                            Self::get_coords(col_index, col_offset, self.data[0].len())
                        {
                            let is_swap_needed = {
                                let upper_space = &self.data[target_row][target_col];
                                let current_space = &self.data[row_index][col_index];
                                *upper_space == Space::Empty && *current_space == Space::Round
                            };

                            if is_swap_needed {
                                self.data[target_row][target_col] = Space::Round;
                                self.data[row_index][col_index] = Space::Empty;
                                is_changed = true;
                            }
                        }
                    }
                }
            }
        }
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
    assert_eq!(
        dish,
        Dish {
            data: vec![
                vec![
                    Space::Round,
                    Space::Empty,
                    Space::Cube,
                    Space::Round,
                    Space::Empty,
                    Space::Cube
                ],
                vec![
                    Space::Round,
                    Space::Empty,
                    Space::Round,
                    Space::Empty,
                    Space::Cube,
                    Space::Empty
                ]
            ]
        }
    );
}

#[test]
fn tilts_west() {
    let input = "O.#..#
O.OO#.";
    let mut dish = Dish::parse(input);
    dish.tilt(&Direction::West);
    assert_eq!(
        dish,
        Dish {
            data: vec![
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
        }
    );
}
