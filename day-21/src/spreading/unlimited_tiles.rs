use super::*;

impl Coordinate {
    pub(crate) fn get_neighbors_with_wrapping(&self) -> Vec<Coordinate> {
        let offsets: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

        offsets
            .iter()
            .map(|off| Coordinate {
                y: self.y + off.0,
                x: self.x + off.1,
            })
            .collect()
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

impl Garden {
    pub fn spread_infinitely(&mut self, iteration: &usize) {
        let mut new_steps = HashSet::new();
        for step in &self.steps {
            let neighbors = step.get_neighbors_with_wrapping();
            for n in neighbors {
                let mapped = Coordinate {
                    x: shifted_rock(n.x, self.x_size),
                    y: shifted_rock(n.y, self.y_size),
                };
                if self.rocks.contains(&mapped) == false {
                    let tile_coord = Coordinate::identify_tile(&n, &self.x_size, &self.y_size);
                    if self.tiles.contains_key(&tile_coord) == false {
                        self.tiles.insert(
                            tile_coord,
                            Tile {
                                starting: mapped,
                                iteration_started: *iteration,
                            },
                        );
                    }
                    new_steps.insert(n);
                }
            }
        }

        self.steps = new_steps;
    }
}

impl Coordinate {
    fn identify_tile(coord: &Coordinate, x_size: &isize, y_size: &isize) -> Self {
        let x_shift = shift_negative(&coord.x, x_size);
        let y_shift = shift_negative(&coord.y, y_size);
        let x = x_shift / *x_size as isize;
        let y = y_shift / *y_size as isize;
        Self { x, y }
    }
}

fn shift_negative(i: &isize, size: &isize) -> isize {
    if i < &0 {
        i - *size as isize + 1
    } else {
        *i
    }
}

#[cfg(test)]
mod t {
    use super::*;

    #[test]
    fn get_coords() {
        let input = Coordinate { x: 1, y: 0 };

        let result: Vec<Coordinate> = input.get_neighbors_with_wrapping();

        assert_eq!(result.len(), 4);
        assert_eq!(result[0], Coordinate { x: 1, y: -1 });
        assert_eq!(result[1], Coordinate { x: 1, y: 1 });
        assert_eq!(result[2], Coordinate { x: 0, y: 0 });
        assert_eq!(result[3], Coordinate { x: 2, y: 0 });
    }

    #[test]
    fn spreads() {
        let mut garden = Garden {
            rocks: HashSet::from([Coordinate { x: 1, y: 0 }]),
            steps: HashSet::from([Coordinate { x: 1, y: -2 }]),
            y_size: 3,
            x_size: 3,
            tiles: HashMap::new(),
        };
        garden.spread_infinitely(&1);

        assert_eq!(
            garden.steps,
            HashSet::from([
                Coordinate { x: 0, y: -2 },
                Coordinate { x: 2, y: -2 },
                Coordinate { x: 1, y: -1 }
            ])
        );
    }

    #[test]
    fn identifies_tile() {
        let y_size = 3;
        let x_size = 3;

        let coord = Coordinate { x: 0, y: -1 };
        assert_eq!(
            Coordinate::identify_tile(&coord, &x_size, &y_size),
            Coordinate { x: 0, y: -1 }
        );

        let coord = Coordinate { x: -3, y: -3 };
        assert_eq!(
            Coordinate::identify_tile(&coord, &x_size, &y_size),
            Coordinate { x: -1, y: -1 }
        );

        let coord = Coordinate { x: 0, y: -4 };
        assert_eq!(
            Coordinate::identify_tile(&coord, &x_size, &y_size),
            Coordinate { x: 0, y: -2 }
        );

        let coord = Coordinate { x: 0, y: 3 };
        assert_eq!(
            Coordinate::identify_tile(&coord, &x_size, &y_size),
            Coordinate { x: 0, y: 1 }
        );

        let coord = Coordinate { x: 3, y: 3 };
        assert_eq!(
            Coordinate::identify_tile(&coord, &x_size, &y_size),
            Coordinate { x: 1, y: 1 }
        );
    }

    #[test]
    fn saves_first_step_in_a_tile() {
        let mut garden = Garden {
            rocks: HashSet::from([Coordinate { x: 0, y: 0 }]),
            steps: HashSet::from([Coordinate { x: 1, y: -3 }]),
            y_size: 3,
            x_size: 3,
            tiles: HashMap::from([(
                Coordinate { x: 0, y: -1 },
                Tile {
                    starting: Coordinate { x: 0, y: 0 },
                    iteration_started: 1,
                },
            )]),
        };
        garden.spread_infinitely(&2);

        assert_eq!(
            garden.steps,
            HashSet::from([
                Coordinate { x: 1, y: -2 },
                Coordinate { x: 1, y: -4 },
                Coordinate { x: 2, y: -3 },
            ])
        );
        assert_eq!(
            garden.tiles,
            HashMap::from([
                (
                    Coordinate { x: 0, y: -1 },
                    Tile {
                        starting: Coordinate { x: 0, y: 0 },
                        iteration_started: 1
                    }
                ),
                (
                    Coordinate { x: 0, y: -2 },
                    Tile {
                        starting: Coordinate { x: 1, y: 2 },
                        iteration_started: 2
                    }
                )
            ])
        );
    }
}
