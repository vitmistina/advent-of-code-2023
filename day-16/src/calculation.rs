use std::collections::HashSet;

use crate::{Beam, Grid};

impl Grid {
    pub fn calculate(&mut self, start_x: usize, start_y: usize, angle: u16) -> usize {
        let mut set = HashSet::new();
        let starting_beam = Beam {
            start_x,
            start_y,
            angle,
            is_starting: true,
        };
        let mut beams = vec![starting_beam];

        while let Some(beam) = beams.pop() {
            let is_new = set.insert(beam.clone());
            if is_new {
                let mut additional_beams = beam.project(self);
                beams.append(&mut additional_beams)
            };
        }
        // println!("");
        // self.print();
        self.data
            .iter()
            .map(|row| row.iter().filter(|loc| loc.is_energized).count())
            .sum()
    }
}

#[test]
fn integration() {
    let input = ".|...\\....
|.-.\\.....
.....|-...
........|.
..........
.........\\
..../.\\\\..
.-.-/..|..
.|....-|.\\
..//.|....";
    let mut grid = Grid::from(input);
    assert_eq!(grid.calculate(0, 0, 90), 46);
}
