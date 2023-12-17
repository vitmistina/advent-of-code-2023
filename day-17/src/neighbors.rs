use super::*;

impl Direction {
    fn opposite(&self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }
    fn full_set() -> HashSet<Self> {
        HashSet::from([
            Direction::Left,
            Direction::Up,
            Direction::Right,
            Direction::Down,
        ])
    }
}

impl Node {
    fn find_directions(&self) -> HashSet<Direction> {
        let mut direction = Direction::full_set();

        if let Some(last_dir) = self.prev_directions.iter().last() {
            direction.remove(&last_dir.opposite());

            let max_in_direction = 3;
            if self.prev_directions.len() >= max_in_direction {
                let mut slice = self.prev_directions.iter().rev().take(max_in_direction);
                if slice.all(|dir| dir == last_dir) {
                    direction.remove(last_dir);
                }
            }
        };

        direction
    }
}

#[test]
fn finds_valid_directions() {
    let node = Node {
        current_score: Some(0),
        is_visited: false,
        heat_loss: 1,
        is_target: false,
        prev_directions: vec![],
    };
    assert_eq!(
        node.find_directions(),
        HashSet::from([
            Direction::Left,
            Direction::Up,
            Direction::Right,
            Direction::Down
        ])
    );

    let node = Node {
        current_score: Some(0),
        is_visited: false,
        heat_loss: 1,
        is_target: false,
        prev_directions: vec![Direction::Right],
    };
    assert_eq!(
        node.find_directions(),
        HashSet::from([Direction::Up, Direction::Right, Direction::Down])
    );

    let node = Node {
        current_score: Some(0),
        is_visited: false,
        heat_loss: 1,
        is_target: false,
        prev_directions: vec![Direction::Right, Direction::Right, Direction::Right],
    };
    assert_eq!(
        node.find_directions(),
        HashSet::from([Direction::Up, Direction::Down])
    );
}

impl Grid {
    pub fn calculate_neighbors(&mut self, coord: &Coordinate) -> Option<u64> {
        let node_clone = &self.data[coord.y][coord.x].clone();
        println!("{} {} {:?}", coord.y, coord.x, node_clone);
        let directions = node_clone.find_directions();

        for direction in directions {
            let (y_offset, x_offset) = match direction {
                Direction::Down => (1, 0),
                Direction::Up => (-1, 0),
                Direction::Right => (0, 1),
                Direction::Left => (0, -1),
                _ => panic!(),
            };

            let new_x = get_coords(coord.x, x_offset, self.data[0].len());
            let new_y = get_coords(coord.y, y_offset, self.data.len());
            match (new_x, new_y) {
                (Some(x), Some(y)) => {
                    let next_node = &mut self.data[y][x];
                    if next_node.is_visited == false {
                        let potential_score =
                            node_clone.current_score.unwrap() + next_node.heat_loss as u64;
                        if (next_node.current_score.is_some()
                            && potential_score < next_node.current_score.unwrap()
                            || next_node.current_score.is_none())
                        {
                            next_node.current_score = Some(potential_score);
                            next_node.prev_directions =
                                [node_clone.prev_directions.as_slice(), &[direction]].concat();

                            if next_node.is_target == true {
                                return next_node.current_score;
                            }
                        }
                    }
                }
                _ => (),
            }
        }

        let node = &mut self.data[coord.y][coord.x];
        node.is_visited = true;
        None
    }
}

fn get_coords(index: usize, offset: i32, len: usize) -> Option<usize> {
    let adjusted_index = if offset.is_negative() {
        index.checked_sub(offset.wrapping_abs() as usize)
    } else {
        index.checked_add(offset as usize)
    };
    match adjusted_index {
        Some(new_index) if new_index < len => Some(new_index),
        _ => None,
    }
}

#[test]
fn calculates_neighbors() {
    let mut grid = Grid {
        data: vec![
            vec![
                Node {
                    current_score: Some(0),
                    is_visited: false,
                    heat_loss: 1,
                    is_target: false,
                    prev_directions: vec![Direction::Down],
                },
                Node {
                    current_score: None,
                    is_visited: false,
                    heat_loss: 2,
                    is_target: false,
                    prev_directions: vec![],
                },
                Node {
                    current_score: None,
                    is_visited: false,
                    heat_loss: 2,
                    is_target: false,
                    prev_directions: vec![],
                },
            ],
            vec![
                Node {
                    current_score: None,
                    is_visited: false,
                    heat_loss: 4,
                    is_target: false,
                    prev_directions: vec![],
                },
                Node {
                    current_score: None,
                    is_visited: false,
                    heat_loss: 5,
                    is_target: true,
                    prev_directions: vec![],
                },
                Node {
                    current_score: None,
                    is_visited: false,
                    heat_loss: 2,
                    is_target: true,
                    prev_directions: vec![],
                },
            ],
        ],
    };

    let result = grid.calculate_neighbors(&Coordinate { x: 0, y: 0 });

    assert_eq!(result, None);

    let expected = vec![
        vec![
            Node {
                current_score: Some(0),
                is_visited: true,
                heat_loss: 1,
                is_target: false,
                prev_directions: vec![Direction::Down],
            },
            Node {
                current_score: Some(2),
                is_visited: false,
                heat_loss: 2,
                is_target: false,
                prev_directions: vec![Direction::Down, Direction::Right],
            },
            Node {
                current_score: None,
                is_visited: false,
                heat_loss: 2,
                is_target: false,
                prev_directions: vec![],
            },
        ],
        vec![
            Node {
                current_score: Some(4),
                is_visited: false,
                heat_loss: 4,
                is_target: false,
                prev_directions: vec![Direction::Down, Direction::Down],
            },
            Node {
                current_score: None,
                is_visited: false,
                heat_loss: 5,
                is_target: true,
                prev_directions: vec![],
            },
            Node {
                current_score: None,
                is_visited: false,
                heat_loss: 2,
                is_target: true,
                prev_directions: vec![],
            },
        ],
    ];
    assert_eq!(grid.data[0][1], expected[0][1]);
    assert_eq!(grid.data[1][0], expected[1][0]);
    assert_eq!(grid.data, expected);
    let result = grid.calculate_neighbors(&Coordinate { x: 0, y: 1 });
    assert_eq!(result, Some(9));
}
