use crossterm::{
    cursor::MoveTo,
    execute,
    style::Print,
    terminal::{Clear, ClearType},
    ExecutableCommand,
};
use std::io::Result;
use std::{collections::HashSet, fs};

mod beaming;
mod parsing;
mod printing;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut grid = Grid::from(&input);
    let result = grid.calculate();
    println!("Hello, world! {result}");
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Beam {
    start_x: usize,
    start_y: usize,
    angle: u16,
    is_starting: bool,
}

#[derive(Debug, PartialEq)]
struct Mirror {
    angle: u16,
}

#[derive(Debug, PartialEq)]
struct Location {
    mirror: Option<Mirror>,
    is_energized: bool,
}

#[derive(Debug, PartialEq)]
struct Grid {
    data: Vec<Vec<Location>>,
}

impl Grid {
    fn calculate(&mut self) -> usize {
        let mut set = HashSet::new();
        let starting_beam = Beam {
            start_x: 0,
            start_y: 0,
            angle: 90,
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
    assert_eq!(grid.calculate(), 46);
}
