use std::{
    collections::{HashMap, HashSet},
    fs,
};

mod parsing;
mod printing;
mod simplification;
mod spreading;

fn main() {
    let input = &fs::read_to_string("input.txt").expect("File should be there");
    let result = integrate(input, 64);
    println!("Hello, world! {result}");

    let result = integrate(input, 600);
    println!("Hello, infinite world! {result}");
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Coordinate {
    x: isize,
    y: isize,
}

#[derive(Debug, PartialEq)]
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

fn integrate(input: &str, steps: u64) -> u64 {
    let mut garden = Garden::parse(input);

    for i in 1..(steps + 1) {
        println!("{i}");
        garden.spread_infinitely(&(i as usize));

        if i > 42 {
            garden.print_big(1, 4, 0);
        }
        // garden.print_tiles();
    }

    garden.steps.len() as u64
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
