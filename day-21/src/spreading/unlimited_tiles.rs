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
    pub fn spread_infinitely(&mut self, iteration: &usize, stats: &mut Stats) {
        let mut ordered_steps = self.steps.iter().collect::<Vec<_>>();
        ordered_steps.sort();
        let mut new_steps = HashSet::new();
        for step in ordered_steps {
            let neighbors = step.get_neighbors_with_wrapping();
            for n in neighbors {
                let mapped = Coordinate {
                    x: shifted_rock(n.x, self.x_size),
                    y: shifted_rock(n.y, self.y_size),
                };
                if self.rocks.contains(&mapped) == false {
                    let tile_coord = Coordinate::identify_tile(&n, &self.x_size, &self.y_size);
                    if self.tiles.contains_key(&tile_coord) == false {
                        let tile = Tile {
                            starting: mapped,
                            iteration_started: *iteration,
                        };
                        self.tiles.insert(tile_coord.clone(), tile.clone());
                        stats.identify_horizontal((&tile_coord, &tile), &self);
                        stats.identify_quadrant((&tile_coord, &tile), &self.tiles);
                    }
                    new_steps.insert(n);
                }
            }
        }
        self.steps = new_steps;

        for ord in &mut stats.horizontals {
            ord.snapshots.insert(
                *iteration - ord.tile.iteration_started - ord.repeats_every,
                self.get_snapshot(&ord.position),
            );
        }
    }
}

impl Garden {
    fn get_snapshot(&self, tile_coord: &Coordinate) -> usize {
        let bounds = get_step_bounds(tile_coord, &self.x_size, &self.y_size);
        self.steps
            .iter()
            .filter(|step| step.is_in_bounds(&bounds))
            .count()
    }
}

#[derive(Debug, PartialEq)]
struct StepBounds {
    x_min: isize,
    x_max: isize,
    y_min: isize,
    y_max: isize,
}

fn get_step_bounds(tile_coord: &Coordinate, x_size: &isize, y_size: &isize) -> StepBounds {
    let (x_min, x_max) = if tile_coord.y == 0 {
        if tile_coord.x > 0 {
            (tile_coord.x * *x_size, isize::MAX)
        } else {
            (isize::MIN, (tile_coord.x + 1) * *x_size - 1)
        }
    } else {
        (0, *x_size - 1)
    };

    let (y_min, y_max) = if tile_coord.x == 0 {
        if tile_coord.y > 0 {
            (tile_coord.y * *y_size, isize::MAX)
        } else {
            (isize::MIN, (tile_coord.y + 1) * *y_size - 1)
        }
    } else {
        (0, *y_size - 1)
    };

    StepBounds {
        x_min,
        x_max,
        y_min,
        y_max,
    }
}

impl Coordinate {
    fn is_in_bounds(&self, bounds: &StepBounds) -> bool {
        self.x >= bounds.x_min
            && self.x <= bounds.x_max
            && self.y >= bounds.y_min
            && self.y <= bounds.y_max
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
    fn recognizes_bounds() {
        let bounds = StepBounds {
            x_min: 6,
            x_max: isize::MAX,
            y_min: 0,
            y_max: 2,
        };
        let coord = Coordinate { x: 0, y: 0 };
        assert_eq!(coord.is_in_bounds(&bounds), false);

        let coord = Coordinate { x: 6, y: 0 };
        assert_eq!(coord.is_in_bounds(&bounds), true);

        let coord = Coordinate { x: 200, y: 0 };
        assert_eq!(coord.is_in_bounds(&bounds), true);

        let coord = Coordinate { x: 200, y: -1 };
        assert_eq!(coord.is_in_bounds(&bounds), false);
    }

    #[test]
    fn gets_bounds() {
        assert_eq!(
            get_step_bounds(&Coordinate { x: 2, y: 0 }, &3, &3),
            StepBounds {
                x_min: 6,
                x_max: isize::MAX,
                y_min: 0,
                y_max: 2
            }
        );
        assert_eq!(
            get_step_bounds(&Coordinate { x: -2, y: 0 }, &3, &3),
            StepBounds {
                x_min: isize::MIN,
                x_max: -4,
                y_min: 0,
                y_max: 2
            }
        );
        assert_eq!(
            get_step_bounds(&Coordinate { x: 0, y: 2 }, &3, &3),
            StepBounds {
                x_min: 0,
                x_max: 2,
                y_min: 6,
                y_max: isize::MAX
            }
        );
        assert_eq!(
            get_step_bounds(&Coordinate { x: 0, y: -2 }, &3, &3),
            StepBounds {
                x_min: 0,
                x_max: 2,
                y_min: isize::MIN,
                y_max: -4
            }
        );
    }

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
        garden.spread_infinitely(
            &1,
            &mut Stats {
                horizontals: Vec::new(),
                quadrants: Vec::new(),
            },
        );

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
        garden.spread_infinitely(
            &2,
            &mut Stats {
                horizontals: Vec::new(),
                quadrants: Vec::new(),
            },
        );

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
