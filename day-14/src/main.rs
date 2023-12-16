use std::{
    collections::{hash_map::DefaultHasher, HashSet},
    fs,
    hash::{Hash, Hasher},
};

mod effictient_tilt;
mod parsing;
mod score;
mod tilting;
fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let mut dish = Dish::parse(&input);

    dish.tilt(&Direction::North);

    let result = dish.calculate_score();

    println!("Hello, world! {result}");

    let mut dish = Dish::parse(&input);

    dish.cycle(1000000000);
    let result = dish.calculate_score();

    //93180 too high (141 until start of cycle 180 second cycle, end on 999)
    //93192 too high (140 until start of cycle 179 second cycle, end on 1 bill)
    //93114 not right (141 until start of cycle, 180 second)
    //93101 not right
    println!("Hello, cycles! {result}");
}

#[derive(Debug, PartialEq, Clone, Copy, Hash, Eq)]
enum Space {
    Round,
    Cube,
    Empty,
}

#[derive(Debug, PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug, PartialEq, Hash, Eq)]
struct Span {
    start: usize,
    len: usize,
}

#[derive(Debug, PartialEq, Hash, Eq)]
struct Dish {
    data: Vec<Vec<Space>>,
    col_spans: Vec<Vec<Span>>,
    row_spans: Vec<Vec<Span>>,
}

impl Dish {
    #[allow(unused)]
    fn print(&self) {
        for row in &self.data {
            let row_string: String = row.iter().map(|space| space.char()).collect();
            println!("{row_string}");
        }
        println!("");
    }

    fn cycle(&mut self, repeats: u32) {
        let mut set = HashSet::new();
        let mut rep = 0;
        let mut cycles = 0;
        let mut prev_cycles = 0;
        while rep < repeats {
            if rep % 1 == 0 {
                println!("{rep}");
            }
            self.tilt(&Direction::North);
            // println!("  North");
            // self.print();

            //west
            self.tilt(&Direction::West);
            // println!("  West");
            // self.print();

            //south
            self.tilt(&Direction::South);
            // println!("  South");
            // self.print();

            //east
            self.tilt(&Direction::East);
            // println!("  East");
            // self.print();

            let mut hasher = DefaultHasher::new();
            self.hash(&mut hasher);
            let hash = hasher.finish();

            // println!("  North");
            // self.print();

            rep += 1;

            let is_new = set.insert(hash);
            if is_new == false {
                println!("Repeat! {rep} {cycles}");
                set.clear();
                set.insert(hash);
                if prev_cycles == cycles && prev_cycles > 0 {
                    let skippable = (repeats - rep) / cycles;
                    rep += skippable * cycles;
                }
                prev_cycles = cycles;
                cycles = 1;
            } else {
                cycles += 1;
            }
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
    dish.tilt(&Direction::North);
    assert_eq!(dish.calculate_score(), 136);
}

#[test]
fn integration_big() {
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
    dish.cycle(1000000000);
    assert_eq!(dish.calculate_score(), 64);
}
