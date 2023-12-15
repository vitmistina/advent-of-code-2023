use crate::{Dish, Space};

impl Dish {
    pub fn tilt(&mut self) {
        let mut is_changed = true;
        while is_changed == true {
            is_changed = false;
            for row_index in 1..self.data.len() {
                for col_index in 0..self.data[0].len() {
                    let is_swap_needed = {
                        let upper_space = &self.data[row_index - 1][col_index];
                        let current_space = &self.data[row_index][col_index];
                        *upper_space == Space::Empty && *current_space == Space::Round
                    };

                    if is_swap_needed {
                        self.data[row_index - 1][col_index] = Space::Round;
                        self.data[row_index][col_index] = Space::Empty;
                        is_changed = true;
                    }
                }
            }
        }
    }
}

#[test]
fn tilts_north() {
    let input = "O.#..#
O.OO#.";
    let mut dish = Dish::parse(input);
    dish.tilt();
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
