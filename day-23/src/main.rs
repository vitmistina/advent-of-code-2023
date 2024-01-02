use std::{
    collections::{HashMap, HashSet},
    fs,
};

mod DAG;
mod parsing;

fn main() {
    let input = &fs::read_to_string("input.txt").expect("File needs to be here");
    let result = integrate(input);
    println!("Hello, world! {result}");
}

#[derive(Debug, PartialEq, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, PartialEq, Clone)]
enum Tile {
    Wall,
    Start,
    Path,
    Finish,
    Arrow(Direction),
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, PartialOrd, Ord)]
struct Coordinate {
    x: usize,
    y: usize,
}

#[derive(Debug, PartialEq, Clone)]
enum NodeType {
    Start,
    Finish,
    Crossroad,
}

#[derive(Debug, PartialEq, Clone)]
struct Node {
    is_visited: bool,
    node_type: NodeType,
    exits: Vec<Direction>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
struct Edge {
    starting_node: Coordinate,
    ending_node: Coordinate,
    length: usize,
}

struct Maze {
    grid: Vec<Vec<Tile>>,
    nodes: HashMap<Coordinate, Node>,
    edges: Vec<Edge>,
    sorted_nodes: Vec<Coordinate>,
}

enum SlopesBehavior {
    Slippery,
    Grippy,
}

fn integrate(input: &str) -> usize {
    let mut maze = Maze::parse(input, SlopesBehavior::Slippery);
    maze.topological_sort();
    maze.find_longest_path()
}

#[test]
fn integrates() {
    let input = "#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#";

    assert_eq!(integrate(input), 94);
}
