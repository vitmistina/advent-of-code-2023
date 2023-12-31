use std::{
    collections::{HashMap, HashSet},
    fs,
};

use symmetry::{get_blue_initial, get_purple_initial};

mod parsing;
mod printing;
mod simplification;
mod spreading;
mod symmetry;

fn main() {
    let input = &fs::read_to_string("input.txt").expect("File should be there");
    // let result = integrate(input, 64);
    // println!("Hello, world! {result}");

    let result = integrate(input, 26501365);

    //636309575728147 too low :((((((
    println!("Hello, infinite world! {result}");
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, PartialOrd, Ord)]
struct Coordinate {
    x: isize,
    y: isize,
}

#[derive(Debug, PartialEq, Clone)]
struct Garden {
    rocks: HashSet<Coordinate>,
    steps: HashSet<Coordinate>,
    y_size: isize,
    x_size: isize,
    tiles: HashMap<Coordinate, Tile>,
}

#[derive(Debug, PartialEq, Clone)]
struct Tile {
    starting: Coordinate,
    iteration_started: usize,
}

struct Stats {
    horizontals: Vec<(TileStatistics)>,
    quadrants: Vec<TileStatistics>,
}

#[derive(Debug, PartialEq)]
struct TileStatistics {
    position: Coordinate,
    tile: Tile,
    repeats_every: usize,
    snapshots: HashMap<usize, usize>,
}

fn integrate(input: &str, steps: u64) -> u64 {
    let mut garden = Garden::parse(input);

    if steps < 65 {
        for i in 0..steps {
            println!("{i}");
            garden.spread();

            // if i > 98 {
            //     garden.print_big(9, 0, -0);
            // }
        }

        // garden.print_tiles();

        return garden.steps.len() as u64;
    }

    let mut blue_garden = garden.clone();
    blue_garden.steps = get_blue_initial(&blue_garden);

    let mut purple_garden = garden.clone();
    purple_garden.steps = get_purple_initial(&purple_garden);

    let mut lime_garden = garden.clone();

    for i in 0..65 {
        garden.spread();
        if i < 64 {
            lime_garden.spread();
        }
        if i < 62 {
            blue_garden.spread();
            purple_garden.spread();
        }
    }
    // garden.print();
    // lime_garden.print();
    // purple_garden.print();
    // blue_garden.print();

    let green = garden.steps.len() as u64;
    let blue = blue_garden.steps.len() as u64;
    let lime = lime_garden.steps.len() as u64;
    let purple = purple_garden.steps.len() as u64;

    let x = (steps - 65) / 131;

    let result = (x + 1) * ((x + 1) * green + x * purple) + x * ((x + 1) * blue + x * lime);

    result as u64
}

#[cfg(test)]
mod t {
    use super::*;

    #[test]
    fn integrates() {
        let input = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";
        let result: u64 = integrate(input, 6);
        assert_eq!(result, 16);
    }

    #[test]
    fn integrates_50_steps() {
        let input = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";
        let result: u64 = integrate(input, 50);
        assert_eq!(result, 1594);
    }

    #[test]
    fn integrates_100_steps() {
        let input = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";
        let result: u64 = integrate(input, 100);
        assert_eq!(result, 6536);
    }

    #[ignore = "too big with inefficient implementation"]
    #[test]
    fn integrates_5000_steps() {
        let input = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";
        let result: u64 = integrate(input, 5000);
        assert_eq!(result, 16733044);
    }
}
