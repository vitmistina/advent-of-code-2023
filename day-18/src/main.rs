use std::fs;

mod commands;
mod counting;
mod floodfill;
mod parsing;
mod vertices;

fn main() {
    let input = &fs::read_to_string("input.txt").unwrap();
    let result = fill_with_lava(input, Command::parse);
    println!("Hello, world! {result}");

    let result = fill_with_lava(input, Command::parse_hex);
    println!("Hello, hex lava pool! {result}");
}

#[derive(Debug, PartialEq, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, PartialEq)]
struct Command {
    dir: Direction,
    len: usize,
}

struct Node {
    is_dug: bool,
}

struct Grid {
    data: Vec<Vec<Node>>,
    current_coord: Coordinate,
}

struct Vertices {
    data: Vec<BigCoordinate>,
    horizontal: Direction,
    vertical: Direction,
}

#[derive(Debug, PartialEq)]
struct BigCoordinate {
    x: isize,
    y: isize,
}

struct Coordinate {
    x: usize,
    y: usize,
}

fn fill_with_lava(input: &str, parse_strategy: fn(&str) -> Command) -> usize {
    let mut vert = Vertices::new();

    let commands: Vec<Command> = input.lines().map(|line| parse_strategy(line)).collect();

    let mut iter = commands.iter().peekable();

    while let Some(current) = iter.next() {
        if let Some(next) = iter.peek() {
            vert.execute(current, &next);
        }
    }

    vert.count()
}

#[test]
fn integration() {
    let input = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";
    assert_eq!(fill_with_lava(input, Command::parse), 62);
}
