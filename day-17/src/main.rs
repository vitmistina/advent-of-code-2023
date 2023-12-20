use std::{
    collections::{HashMap, HashSet},
    fs,
};

mod directions;
mod neighbors;
mod parsing;
mod printing;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut grid = Grid::parse(&input, &1, &3);
    let result = grid.find_path();

    //1244 too high
    //1050 too low
    // 1238 is right
    println!("Hello, world! {result}");

    let mut grid = Grid::parse(&input, &4, &10);
    let result = grid.find_path();

    println!("Hello, ultra crucible! {result}");
}

struct Grid {
    data: HashMap<(usize, Direction), Vec<Vec<Node>>>,
    min_repeat: u8,
    max_repeat: u8,
}

#[derive(Eq, Hash, Debug, PartialEq, Clone)]
struct Coordinate {
    x: usize,
    y: usize,
}

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy, Default)]
enum Direction {
    #[default]
    Right,
    Down,
    Up,
    Left,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Node {
    current_score: Option<u64>,
    heuristic_current_score: Option<u64>,
    heat_loss: u8,
    is_target: bool,
    prev_directions: Vec<Direction>,
    coord: Coordinate,
    is_visited: bool,
}

impl Grid {
    fn find_path(&mut self) -> u64 {
        let mut unvisited = vec![self.data.get(&(1, Direction::Right)).unwrap()[0][0].clone()];
        let mut max_x = 0;
        let mut max_y = 0;
        while let Some(next_node) = unvisited.pop() {
            // if next_node.coord.x > max_x {
            //     max_x = next_node.coord.x;
            //     println!("{:?}", next_node.coord);
            // }
            // if next_node.coord.y > max_y {
            //     max_y = next_node.coord.y;
            //     println!("{:?}", next_node.coord);
            // }

            // if next_node.coord.x == 6 && next_node.coord.y == 0 {
            //     println!();
            // };
            match self.calculate_neighbors(&next_node) {
                (_, Some(result)) => {
                    // println!("{result}");
                    println!("{:?}", next_node.prev_directions);
                    // self.print(&next_node.prev_directions);
                    return result;
                }
                (new_unvisited, None) => {
                    // let filtered_unvisited = new_unvisited
                    //     .iter()
                    //     .filter(|new| {
                    //         unvisited
                    //             .iter()
                    //             .any(|old| old.prev_directions == new.prev_directions)
                    //             == false
                    //     })
                    //     .map(|elem| elem.clone())
                    //     .collect::<Vec<Node>>();
                    let mut next = [unvisited.as_slice(), &new_unvisited].concat();
                    Node::sort_by_score(&mut next);
                    unvisited = next;
                }
            }
        }
        0
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
    let mut grid = Grid::parse(input, &1, &3);

    let result = grid.find_path();
    // grid.print();
    assert_eq!(result, 102);
}

#[test]
fn integration_ultra_crucible() {
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
    let mut grid = Grid::parse(input, &4, &10);

    let result = grid.find_path();
    // grid.print();
    assert_eq!(result, 94);

    let input = "111111111111
999999999991
999999999991
999999999991
999999999991";
    let mut grid = Grid::parse(input, &4, &10);

    let result = grid.find_path();
    // grid.print();
    assert_eq!(result, 71);
}
