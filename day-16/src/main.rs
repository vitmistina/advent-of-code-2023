use std::fs;

mod beaming;
mod calculation;
mod parsing;
mod printing;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut grid = Grid::from(&input);
    let result = grid.calculate(0, 0, 90);
    println!("Hello, world! {result}");

    let mut grid = Grid::from(&input);
    let result = grid.calculate_brute_force();
    println!("Hello, brute! {result}");
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Beam {
    start_x: usize,
    start_y: usize,
    angle: u16,
    is_starting: bool,
}

#[derive(Debug, PartialEq, Clone)]
struct Mirror {
    angle: u16,
}

#[derive(Debug, PartialEq, Clone)]
struct Location {
    mirror: Option<Mirror>,
    is_energized: bool,
}

#[derive(Debug, PartialEq, Clone)]
struct Grid {
    data: Vec<Vec<Location>>,
}

impl Grid {
    pub fn calculate_brute_force(&mut self) -> usize {
        let mut max = 0;

        let y_len = self.data.len();
        let x_len = self.data[0].len();

        for y in 0..y_len {
            let result = self.clone().calculate(0, y, 90);
            update_max(result, &mut max);
        }

        for x in 0..x_len {
            let result = self.clone().calculate(x, y_len - 1, 0);
            update_max(result, &mut max);
        }

        for y in 0..y_len {
            let result = self.clone().calculate(x_len - 1, y, 270);
            update_max(result, &mut max);
        }

        for x in 0..x_len {
            let result = self.clone().calculate(x, 0, 180);
            update_max(result, &mut max);
        }

        max
    }
}

fn update_max(result: usize, max: &mut usize) {
    if result > *max {
        *max = result;
    }
}

#[test]
fn integration_part2() {
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

    assert_eq!(grid.calculate_brute_force(), 51);
}
