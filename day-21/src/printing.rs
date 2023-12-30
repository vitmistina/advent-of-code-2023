use std::ops::Range;

use crate::spreading::shifted_rock;

use super::*;

impl Garden {
    pub(crate) fn print(&self) {
        for y in 0..self.y_size {
            let line: String = (0..self.x_size)
                .map(|x| {
                    let (step, rock) = {
                        (
                            self.steps.get(&Coordinate { x, y }),
                            self.rocks.get(&Coordinate { x, y }),
                        )
                    };
                    match (step, rock) {
                        (None, None) => '.',
                        (None, Some(_)) => '#',
                        (Some(_), None) => 'O',
                        (Some(_), Some(_)) => panic!("Rock should not overlap with step!"),
                    }
                })
                .collect();
            println!("{line}");
        }
        println!("");
    }

    pub(crate) fn print_big(&self, span: isize, offset_x: isize, offset_y: isize) {
        let y_min = -self.y_size * span + offset_y * self.y_size;
        let y_max = self.y_size * (1 + span) + offset_y * self.y_size;
        let x_min = -self.x_size * span + offset_x * self.x_size;
        let x_max = self.x_size * (1 + span) + offset_x * self.x_size;

        for y in y_min..y_max {
            let line: String = (x_min..x_max)
                .map(|x| {
                    let (step, rock) = {
                        let rock_coord = &Coordinate {
                            x: shifted_rock(x, self.x_size),
                            y: shifted_rock(y, self.y_size),
                        };
                        (
                            self.steps.get(&Coordinate { x, y }),
                            self.rocks.get(rock_coord),
                        )
                    };
                    match (step, rock) {
                        (None, None) => '.',
                        (None, Some(_)) => '#',
                        (Some(_), None) => 'O',
                        (Some(_), Some(_)) => panic!("Rock should not overlap with step!"),
                    }
                })
                .collect();
            println!("{line}");
        }
        println!("");
    }

    pub(crate) fn print_tiles(&self) {
        let s: isize = 5;
        for y in -s..s {
            let line: String = (-s..s)
                .map(|x| match self.tiles.get(&Coordinate { x, y }) {
                    Some(tile) => format!(
                        "({:>+3},{:>+3},{:>3})",
                        tile.starting.x, tile.starting.y, tile.iteration_started
                    ),
                    None => "(   ,   ,   )".to_string(),
                })
                .collect();
            println!("{line}");
        }
    }
}
