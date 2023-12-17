use std::collections::HashSet;

mod finder;
mod neighbors;
mod parsing;

fn main() {
    println!("Hello, world!");
}

struct Grid {
    data: Vec<Vec<Node>>,
}

#[derive(Debug, PartialEq)]
struct Coordinate {
    x: usize,
    y: usize,
}

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, PartialEq, Clone)]
struct Node {
    current_score: Option<u64>,
    is_visited: bool,
    heat_loss: u8,
    is_target: bool,
    prev_directions: Vec<Direction>,
}

impl Grid {
    fn find_path(&mut self) -> u64 {
        while let Some(unvisited) = self.find_lowest_unvisited() {
            if let Some(potential_result) = self.calculate_neighbors(&unvisited) {
                return potential_result;
            };
        }
        0
    }
}

impl Grid {
    fn print(&self) {
        self.data.iter().for_each(|row| {
            let string: String = row
                .iter()
                .map(|node| if node.is_visited { "#" } else { "." })
                .collect();
            println!("{string}");
        });
    }
}

#[test]
fn integration() {
    let input = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";
    let mut grid = Grid::parse(input);

    let result = grid.find_path();
    grid.print();
    assert_eq!(result, 102)
}
