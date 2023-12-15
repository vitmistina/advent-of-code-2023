use std::fs;

mod parsing;
mod score;
mod tilting;
fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let mut dish = Dish::parse(&input);

    dish.tilt();

    let result = dish.calculate_score();

    println!("Hello, world! {result}");

    let mut dish = Dish::parse(&input);

    dish.cycle(1_000_000_000);
    let result = dish.calculate_score();
    println!("Hello, cycles! {result}");
}

#[derive(Debug, PartialEq, Clone)]
enum Space {
    Round,
    Cube,
    Empty,
}

#[derive(Debug, PartialEq)]
struct Dish {
    data: Vec<Vec<Space>>,
}

impl Dish {
    fn transpose(&self) -> Self {
        let rows = self.data.len();
        let cols = self.data[0].len();

        let transposed_data = (0..cols)
            .map(|col_index| {
                (0..rows)
                    .map(|row_index| self.data[row_index][col_index].clone())
                    .collect()
            })
            .collect();

        Self {
            data: transposed_data,
        }
    }

    fn print(&self) {
        for row in &self.data {
            let row_string: String = row.iter().map(|space| space.char()).collect();
            println!("{row_string}");
        }
        println!("");
    }

    fn cycle(&mut self, repeats: u32) {
        for rep in 0..repeats {
            self.tilt();
            // println!("  North");
            // self.print();

            //west
            *self = self.transpose();
            self.tilt();
            // println!("  West");
            // self.print();

            //south
            *self = self.transpose();
            self.data.reverse();
            self.tilt();
            // println!("  South");
            // self.print();

            //east
            *self = self.transpose();
            self.data.reverse();
            self.tilt();
            // println!("  East");
            // self.print();

            //turn back north
            self.data.reverse();
            *self = self.transpose();
            self.data.reverse();
            // println!("  North");
            // self.print();
        }
    }
}

#[test]
fn cycles() {
    let input = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
    let expected_1_cycle = Dish::parse(
        ".....#....
....#...O#
...OO##...
.OO#......
.....OOO#.
.O#...O#.#
....O#....
......OOOO
#...O###..
#..OO#....",
    );

    let mut dish = Dish::parse(input);

    dish.cycle(1);

    assert_eq!(dish, expected_1_cycle);

    let expected_3_cycle = Dish::parse(
        ".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#...O###.O
#.OOO#...O",
    );
    let mut dish = Dish::parse(input);

    dish.cycle(3);

    assert_eq!(dish, expected_3_cycle);
}

#[test]
fn integration() {
    let input = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
    let mut dish = Dish::parse(&input);
    dish.tilt();
    assert_eq!(dish.calculate_score(), 136);
}
