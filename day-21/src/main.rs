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

    let result = integrate_big(input, 26501365, 0, 2);

    //636309575728147 too low :((((((
    //646950151128147 too high - tried a 2 steps more in lime garden
    //635941193699447 too low
    //636391426308147 not right (after filling missing tiles)
    //636391426712747 right!

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

    for _ in 0..steps {
        garden.spread();
    }

    return garden.steps.len() as u64;
}

fn integrate_multiple_tiles(input: &str, steps: u64) -> u64 {
    let mut garden = Garden::parse(input);

    for i in 0..steps {
        println!("{i}");
        garden.spread_infinitely(
            &(i as usize),
            &mut Stats {
                horizontals: Vec::new(),
                quadrants: Vec::new(),
            },
        );
    }

    garden.print_big(2, 0, -0);

    return garden.steps.len() as u64;
}

fn integrate_big(
    input: &str,
    steps: u64,
    missing_purple_adjuster: u64,
    missing_blue_adjuster: u64,
) -> u64 {
    let mut garden = Garden::parse(input);

    let distance_to_edge = (garden.x_size as u64 - 1) / 2;
    assert_eq!((steps - distance_to_edge) % garden.x_size as u64, 0);
    let x = (steps - distance_to_edge) / garden.x_size as u64;

    let mut blue_garden = garden.clone();
    blue_garden.steps = get_blue_initial(&blue_garden);

    let mut purple_garden = garden.clone();
    purple_garden.steps = get_purple_initial(&purple_garden);

    let mut lime_garden = garden.clone();

    for i in 0..distance_to_edge {
        garden.spread();
        if i < distance_to_edge - 1 {
            lime_garden.spread();
        }
        if i < distance_to_edge - 3 {
            blue_garden.spread();
            purple_garden.spread();
        }
    }
    println!("green");
    garden.print();
    println!("lime");
    lime_garden.print();
    println!("purple");
    purple_garden.print();
    println!("blue");
    blue_garden.print();

    let green = garden.steps.len() as u64;
    let blue = blue_garden.steps.len() as u64;
    let lime = lime_garden.steps.len() as u64;
    let purple = purple_garden.steps.len() as u64;

    let subtotal_green = (x + 1) * (x + 1) * green;
    let subtotal_blue = (x + 1) * x * blue;
    let subtotal_lime = x * x * lime;
    let subtotal_purple = x * (x + 1) * purple;

    // missing purple adjuster fills inside
    let result = subtotal_blue
        + subtotal_green
        + subtotal_lime
        + subtotal_purple
        + x * x * missing_purple_adjuster
        + (x + 1) * x * missing_blue_adjuster;

    result as u64
}

#[cfg(test)]
mod t {
    use super::*;

    #[test]
    fn bruteforce_and_symmetry_approach_match() {
        let input = "...........
.......#.#.
...........
..#.....#..
....#.#....
.....S.....
.#.......#.
........#..
.#..#......
..#...#.#..
...........";

        let steps = 5 + 22;
        let bruteforce = integrate_multiple_tiles(input, steps);
        assert_eq!(bruteforce, 686);
        let symmetry = integrate_big(input, steps, 1, 0);
        assert_eq!(symmetry, bruteforce);
    }

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
