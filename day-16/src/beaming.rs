use crate::{Beam, Grid, Location, Mirror};

impl Beam {
    pub fn project(&self, grid: &mut Grid) -> Vec<Beam> {
        let mut current_x = self.start_x;
        let mut current_y = self.start_y;
        {
            let loc = &mut grid.data[current_y][current_x];
            loc.is_energized = true;
        }

        if current_x == 0 && current_y == 0 && self.angle == 90 {
            if let Some(beams) = self.resolve_location(grid, current_y, current_x) {
                return beams;
            }
        }

        let (y_offset, x_offset) = match self.angle {
            180 => (1, 0),
            0 => (-1, 0),
            90 => (0, 1),
            270 => (0, -1),
            _ => panic!(),
        };

        if x_offset != 0 {
            while let Some(new_x) = get_coords(current_x, x_offset, grid.data[0].len()) {
                current_x = new_x;
                if let Some(beams) = self.resolve_location(grid, current_y, current_x) {
                    return beams;
                }
            }
        };

        if y_offset != 0 {
            while let Some(new_y) = get_coords(current_y, y_offset, grid.data.len()) {
                current_y = new_y;
                if let Some(beams) = self.resolve_location(grid, current_y, current_x) {
                    return beams;
                }
            }
        };
        vec![]
    }

    fn resolve_location(
        &self,
        grid: &mut Grid,
        current_y: usize,
        current_x: usize,
    ) -> Option<Vec<Beam>> {
        println!("");
        grid.print();
        let loc = &mut grid.data[current_y][current_x];
        loc.is_energized = true;
        if let Some(mirror) = &loc.mirror {
            if self.angle != mirror.angle && self.angle != mirror.angle + 180 {
                if mirror.angle % 90 == 0 {
                    return Some(vec![
                        Beam {
                            start_x: current_x,
                            start_y: current_y,
                            angle: (360 + self.angle + 90) % 360,
                        },
                        Beam {
                            start_x: current_x,
                            start_y: current_y,
                            angle: (360 + self.angle - 90) % 360,
                        },
                    ]);
                } else {
                    return Some(vec![Beam {
                        start_x: current_x,
                        start_y: current_y,
                        angle: calculate_angle(self.angle, mirror.angle),
                    }]);
                };
            }
        }
        None
    }
}

fn calculate_angle(beam: u16, mirror: u16) -> u16 {
    (360 + mirror * 2 - beam) % 360
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

#[test]
fn calculates_angles() {
    assert_eq!(calculate_angle(90, 45), 0);
    assert_eq!(calculate_angle(90, 135), 180);
    assert_eq!(calculate_angle(0, 45), 90);
    assert_eq!(calculate_angle(0, 135), 270);
    assert_eq!(calculate_angle(270, 45), 180);
}

#[test]
fn starts_on_mirror() {
    let beam = Beam {
        start_x: 0,
        start_y: 0,
        angle: 90,
    };

    let mut grid = Grid {
        data: vec![
            vec![
                Location {
                    mirror: Some(Mirror { angle: 135 }),
                    is_energized: false,
                },
                Location {
                    mirror: None,
                    is_energized: false,
                },
            ],
            vec![
                Location {
                    mirror: None,
                    is_energized: false,
                },
                Location {
                    mirror: None,
                    is_energized: false,
                },
            ],
        ],
    };

    let beams = beam.project(&mut grid);

    assert_eq!(
        beams[0],
        Beam {
            start_x: 0,
            start_y: 0,
            angle: 180,
        }
    );

    assert_eq!(
        grid.data,
        vec![
            vec![
                Location {
                    mirror: Some(Mirror { angle: 135 }),
                    is_energized: true,
                },
                Location {
                    mirror: None,
                    is_energized: false,
                },
            ],
            vec![
                Location {
                    mirror: None,
                    is_energized: false,
                },
                Location {
                    mirror: None,
                    is_energized: false,
                },
            ],
        ],
    );
}

#[test]
fn energises_and_splits() {
    let beam = Beam {
        start_x: 0,
        start_y: 0,
        angle: 90,
    };
    let mut grid = Grid {
        data: vec![vec![
            Location {
                mirror: None,
                is_energized: false,
            },
            Location {
                mirror: None,
                is_energized: false,
            },
            Location {
                mirror: Some(crate::Mirror { angle: 45 }),
                is_energized: false,
            },
            Location {
                mirror: None,
                is_energized: false,
            },
        ]],
    };
    let beams = beam.project(&mut grid);
    assert_eq!(
        grid.data,
        vec![vec![
            Location {
                mirror: None,
                is_energized: true,
            },
            Location {
                mirror: None,
                is_energized: true,
            },
            Location {
                mirror: Some(crate::Mirror { angle: 45 }),
                is_energized: true,
            },
            Location {
                mirror: None,
                is_energized: false,
            },
        ]]
    );

    assert_eq!(
        beams[0],
        Beam {
            start_x: 2,
            start_y: 0,
            angle: 0,
        }
    )
}
