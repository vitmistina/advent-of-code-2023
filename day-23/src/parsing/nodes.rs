use super::*;

pub(super) fn get_nodes_from_grid(
    grid: &[Vec<Tile>],
    slopes: &SlopesBehavior,
) -> HashMap<Coordinate, Node> {
    let mut nodes = HashMap::new();

    for (y, row) in grid.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            let coord = Coordinate { x, y };

            // Determine if this tile is a node
            let is_node = match tile {
                Tile::Path => {
                    // Check if surrounded by walls or arrows
                    let mut surrounded = true;
                    if y > 0 {
                        surrounded &= matches!(grid[y - 1][x], Tile::Wall | Tile::Arrow(_));
                    }
                    if y < grid.len() - 1 {
                        surrounded &= matches!(grid[y + 1][x], Tile::Wall | Tile::Arrow(_));
                    }
                    if x > 0 {
                        surrounded &= matches!(grid[y][x - 1], Tile::Wall | Tile::Arrow(_));
                    }
                    if x < row.len() - 1 {
                        surrounded &= matches!(grid[y][x + 1], Tile::Wall | Tile::Arrow(_));
                    }
                    surrounded
                }
                Tile::Start | Tile::Finish => true,
                _ => false,
            };

            if is_node {
                let mut exits = Vec::new();
                let node_type = match tile {
                    Tile::Start => NodeType::Start,
                    Tile::Finish => NodeType::Finish,
                    _ => NodeType::Crossroad, // Assuming all Paths are Crossroads for simplicity
                };

                // Check adjacent tiles for arrows pointing away
                if y > 0 {
                    if let Tile::Arrow(direction) = &grid[y - 1][x] {
                        if slopes == &SlopesBehavior::Slippery && direction == &Direction::Up {
                            exits.push(Direction::Up);
                        } else if slopes == &SlopesBehavior::Grippy {
                            exits.push(Direction::Up);
                        }
                    }
                }
                if y < grid.len() - 1 {
                    if let Tile::Arrow(direction) = &grid[y + 1][x] {
                        if slopes == &SlopesBehavior::Slippery && direction == &Direction::Down {
                            exits.push(Direction::Down);
                        } else if slopes == &SlopesBehavior::Grippy {
                            exits.push(Direction::Down);
                        }
                    }
                }
                if x > 0 {
                    if let Tile::Arrow(direction) = &grid[y][x - 1] {
                        if slopes == &SlopesBehavior::Slippery && direction == &Direction::Left {
                            exits.push(Direction::Left);
                        } else if slopes == &SlopesBehavior::Grippy {
                            exits.push(Direction::Left);
                        }
                    }
                }
                if x < row.len() - 1 {
                    if let Tile::Arrow(direction) = &grid[y][x + 1] {
                        if slopes == &SlopesBehavior::Slippery && direction == &Direction::Right {
                            exits.push(Direction::Right);
                        } else if slopes == &SlopesBehavior::Grippy {
                            exits.push(Direction::Right);
                        }
                    }
                }

                if node_type == NodeType::Start {
                    exits.push(Direction::Down);
                }

                // Insert the node into the hashmap
                nodes.insert(
                    coord,
                    Node {
                        id: nodes.len(),
                        is_visited: false,
                        node_type,
                        exits,
                    },
                );
            }
        }
    }

    nodes
}

#[test]
fn parses_grid_to_nodes() {
    let grid = vec![
        vec![
            Tile::Wall,
            Tile::Start,
            Tile::Wall,
            Tile::Wall,
            Tile::Wall,
            Tile::Wall,
        ],
        vec![
            Tile::Wall,
            Tile::Path,
            Tile::Path,
            Tile::Wall,
            Tile::Wall,
            Tile::Wall,
        ],
        vec![
            Tile::Wall,
            Tile::Wall,
            Tile::Arrow(Direction::Down),
            Tile::Wall,
            Tile::Wall,
            Tile::Wall,
        ],
        vec![
            Tile::Wall,
            Tile::Arrow(Direction::Left),
            Tile::Path,
            Tile::Arrow(Direction::Right),
            Tile::Path,
            Tile::Wall,
        ],
        vec![
            Tile::Wall,
            Tile::Wall,
            Tile::Wall,
            Tile::Wall,
            Tile::Finish,
            Tile::Wall,
        ],
    ];

    let expected: HashMap<Coordinate, Node> = HashMap::from([
        (
            Coordinate { x: 1, y: 0 },
            Node {
                id: 0,
                is_visited: false,
                node_type: NodeType::Start,
                exits: vec![Direction::Down],
            },
        ),
        (
            Coordinate { x: 2, y: 3 },
            Node {
                id: 1,
                is_visited: false,
                node_type: NodeType::Crossroad,
                exits: vec![Direction::Left, Direction::Right],
            },
        ),
        (
            Coordinate { x: 4, y: 4 },
            Node {
                id: 2,
                is_visited: false,
                node_type: NodeType::Finish,
                exits: vec![],
            },
        ),
    ]);

    let nodes: HashMap<Coordinate, Node> = get_nodes_from_grid(&grid, &SlopesBehavior::Slippery);

    assert_eq!(nodes, expected);
}

#[test]
fn parses_grippy_slopes_to_nodes() {
    let grid = vec![
        vec![
            Tile::Wall,
            Tile::Start,
            Tile::Wall,
            Tile::Wall,
            Tile::Wall,
            Tile::Wall,
        ],
        vec![
            Tile::Wall,
            Tile::Path,
            Tile::Path,
            Tile::Wall,
            Tile::Wall,
            Tile::Wall,
        ],
        vec![
            Tile::Wall,
            Tile::Wall,
            Tile::Arrow(Direction::Down),
            Tile::Wall,
            Tile::Wall,
            Tile::Wall,
        ],
        vec![
            Tile::Wall,
            Tile::Arrow(Direction::Left),
            Tile::Path,
            Tile::Arrow(Direction::Right),
            Tile::Path,
            Tile::Wall,
        ],
        vec![
            Tile::Wall,
            Tile::Wall,
            Tile::Wall,
            Tile::Wall,
            Tile::Finish,
            Tile::Wall,
        ],
    ];

    let expected: HashMap<Coordinate, Node> = HashMap::from([
        (
            Coordinate { x: 1, y: 0 },
            Node {
                id: 0,
                is_visited: false,
                node_type: NodeType::Start,
                exits: vec![Direction::Down],
            },
        ),
        (
            Coordinate { x: 2, y: 3 },
            Node {
                id: 1,
                is_visited: false,
                node_type: NodeType::Crossroad,
                exits: vec![Direction::Up, Direction::Left, Direction::Right],
            },
        ),
        (
            Coordinate { x: 4, y: 4 },
            Node {
                id: 2,
                is_visited: false,
                node_type: NodeType::Finish,
                exits: vec![],
            },
        ),
    ]);

    let nodes: HashMap<Coordinate, Node> = get_nodes_from_grid(&grid, &SlopesBehavior::Grippy);

    assert_eq!(nodes, expected);
}
