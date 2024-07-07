use std::{
    collections::{HashMap, HashSet},
    fs,
};

mod DAG;
mod integer_programming;
mod parsing;
mod printing;

fn main() {
    let input = &fs::read_to_string("input.txt").expect("File needs to be here");
    let result = integrate(input, SlopesBehavior::Slippery);
    println!("Hello, world! {result}");

    let result = integrate(input, SlopesBehavior::Grippy);
    println!("Hello, traveling salesman! {result}");
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
    id: usize,
    is_visited: bool,
    node_type: NodeType,
    exits: Vec<Direction>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
struct Edge {
    starting_node_id: usize,
    ending_node_id: usize,
    starting_node_loc: Coordinate,
    ending_node_loc: Coordinate,
    length: usize,
}

struct Maze {
    grid: Vec<Vec<Tile>>,
    nodes: HashMap<Coordinate, Node>,
    edges: Vec<Edge>,
    sorted_nodes: Vec<Coordinate>,
}

#[derive(Debug, PartialEq)]
enum SlopesBehavior {
    Slippery,
    Grippy,
}

fn integrate(input: &str, slopes: SlopesBehavior) -> usize {
    let mut maze = Maze::parse(input, &slopes);
    if slopes == SlopesBehavior::Slippery {
        maze.topological_sort();
        maze.find_longest_path()
    } else {
        let _ = maze.save_to_graphml("big_mapmaze.graphml");
        0
    }
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

    assert_eq!(integrate(input, SlopesBehavior::Slippery), 94);
}

#[test]
fn integrates_grippy_slopes() {
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

    // assert_eq!(integrate(input, SlopesBehavior::Grippy), 154);
}
