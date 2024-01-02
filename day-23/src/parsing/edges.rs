use super::*;

impl Maze {
    pub(super) fn find_edges(&mut self) {
        self.edges.clear();

        for (start_coord, start_node) in self.nodes.clone() {
            for exit_dir in &start_node.exits {
                let mut visited = HashSet::new();
                visited.insert(start_coord.clone());

                self.explore_path(&start_coord, &start_coord, exit_dir, &mut visited, 0);
            }
        }

        self.edges.sort();
    }

    fn is_within_bounds(&self, coord: &Coordinate) -> bool {
        coord.x < self.grid[0].len() && coord.y < self.grid.len()
    }

    fn explore_path(
        &mut self,
        coord: &Coordinate,
        started_from: &Coordinate,
        dir: &Direction,
        visited: &mut HashSet<Coordinate>,
        length: usize,
    ) {
        // Calculate next coordinate based on direction
        let next_coord = match dir {
            Direction::Up => Coordinate {
                x: coord.x,
                y: coord.y.checked_sub(1).unwrap_or(0),
            },
            Direction::Down => Coordinate {
                x: coord.x,
                y: coord.y.saturating_add(1),
            },
            Direction::Left => Coordinate {
                x: coord.x.checked_sub(1).unwrap_or(0),
                y: coord.y,
            },
            Direction::Right => Coordinate {
                x: coord.x.saturating_add(1),
                y: coord.y,
            },
        };

        // Check if the next coordinate is within bounds, is not a wall, and is not already visited
        if !self.is_within_bounds(&next_coord)
            || visited.contains(&next_coord)
            || matches!(self.grid[next_coord.y][next_coord.x], Tile::Wall)
        {
            return;
        }

        visited.insert(next_coord.clone());

        // Check if the next coordinate is another node
        if let Some(_) = self.nodes.get(&next_coord) {
            // Found another node, create an edge
            self.edges.push(Edge {
                starting_node: started_from.clone(),
                ending_node: next_coord,
                length: length + 1,
            });
            return;
        }

        // Continue exploring in all directions except the opposite
        let opposite_dir = match dir {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        };

        for new_dir in [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ] {
            if new_dir != opposite_dir {
                self.explore_path(&next_coord, started_from, &new_dir, visited, length + 1);
            }
        }
    }
}

#[test]
fn creates_edges() {
    let mut maze = Maze {
        grid: vec![
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
                Tile::Path,
                Tile::Arrow(Direction::Left),
                Tile::Path,
                Tile::Arrow(Direction::Right),
                Tile::Path,
                Tile::Wall,
            ],
            vec![
                Tile::Path,
                Tile::Path,
                Tile::Wall,
                Tile::Wall,
                Tile::Finish,
                Tile::Wall,
            ],
            vec![
                Tile::Wall,
                Tile::Path,
                Tile::Path,
                Tile::Path,
                Tile::Path,
                Tile::Wall,
            ],
        ],
        nodes: HashMap::from([
            (
                Coordinate { x: 1, y: 0 },
                Node {
                    is_visited: false,
                    node_type: NodeType::Start,
                    exits: vec![Direction::Down],
                },
            ),
            (
                Coordinate { x: 2, y: 3 },
                Node {
                    is_visited: false,
                    node_type: NodeType::Crossroad,
                    exits: vec![Direction::Left, Direction::Right],
                },
            ),
            (
                Coordinate { x: 4, y: 4 },
                Node {
                    is_visited: false,
                    node_type: NodeType::Finish,
                    exits: vec![],
                },
            ),
        ]),
        edges: vec![],
        sorted_nodes: vec![],
    };

    let expected_edges = vec![
        Edge {
            starting_node: Coordinate { x: 1, y: 0 },
            ending_node: Coordinate { x: 2, y: 3 },
            length: 4,
        },
        Edge {
            starting_node: Coordinate { x: 2, y: 3 },
            ending_node: Coordinate { x: 4, y: 4 },
            length: 3,
        },
        Edge {
            starting_node: Coordinate { x: 2, y: 3 },
            ending_node: Coordinate { x: 4, y: 4 },
            length: 7,
        },
    ];
    maze.find_edges();

    assert_eq!(maze.edges, expected_edges);
}
