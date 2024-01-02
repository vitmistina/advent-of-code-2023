use self::nodes::get_nodes_from_grid;

use super::*;

mod edges;
mod nodes;

impl Maze {
    pub(super) fn parse(input: &str, slopes: SlopesBehavior) -> Self {
        let lines: Vec<&str> = input.split('\n').collect();
        let y_len = lines.len();
        let mut grid = Vec::new();

        for (y, line) in lines.iter().enumerate() {
            let mut row = Vec::new();
            let x_len = line.len();
            for (x, ch) in line.chars().enumerate() {
                let tile = match ch {
                    'v' => Tile::Arrow(Direction::Down),
                    '^' => Tile::Arrow(Direction::Up),
                    '<' => Tile::Arrow(Direction::Left),
                    '>' => Tile::Arrow(Direction::Right),
                    '#' => Tile::Wall,
                    '.' => Tile::Path,
                    _ => panic!("Unexpected character"),
                };

                if x == 1 && y == 0 {
                    row.push(Tile::Start);
                } else if x == x_len - 2 && y == y_len - 1 {
                    row.push(Tile::Finish);
                } else {
                    row.push(tile);
                }
            }
            grid.push(row);
        }

        let mut maze = Maze {
            grid: grid.clone(),
            nodes: get_nodes_from_grid(&grid, SlopesBehavior::Slippery),
            edges: vec![],
            sorted_nodes: vec![],
        };
        maze.find_edges();
        maze
    }
}

#[test]
fn parses_to_grid() {
    let input = "v.#^#
#.>.>";
    let expected = vec![
        vec![
            Tile::Arrow(Direction::Down),
            Tile::Start,
            Tile::Wall,
            Tile::Arrow(Direction::Up),
            Tile::Wall,
        ],
        vec![
            Tile::Wall,
            Tile::Path,
            Tile::Arrow(Direction::Right),
            Tile::Finish,
            Tile::Arrow(Direction::Right),
        ],
    ];

    let maze = Maze::parse(input, SlopesBehavior::Slippery);

    assert_eq!(maze.grid, expected);
}
