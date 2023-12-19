use std::collections::HashMap;

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
    pub fn full_set() -> HashSet<Self> {
        HashSet::from([
            Direction::Right,
            Direction::Down,
            Direction::Left,
            Direction::Up,
        ])
    }

    pub fn n_full_sets(steps: &u8) -> HashMap<u8, HashSet<Self>> {
        let mut map = HashMap::new();
        for i in 1..*steps + 1 {
            map.insert(
                i,
                HashSet::from([
                    Direction::Right,
                    Direction::Down,
                    Direction::Left,
                    Direction::Up,
                ]),
            );
        }
        map
    }
}

impl Node {
    pub fn sort_by_score(nodes: &mut Vec<Node>) {
        // let mut fset = HashSet::from(
        //     nodes
        //         .iter()
        //         .map(|node| node.prev_directions.clone())
        //         .collect::<Vec<_>>(),
        // );
        // let mut set = HashSet::new();
        // set.insert(nodes[0].prev_directions.clone());
        nodes.sort_by(|b, a| a.heuristic_current_score.cmp(&b.heuristic_current_score));
    }

    fn find_directions(&self, max_in_direction: &usize) -> Vec<Direction> {
        let mut direction = vec![
            Direction::Right,
            Direction::Down,
            Direction::Left,
            Direction::Up,
        ];

        if let Some(last_dir) = self.prev_directions.iter().last() {
            if let Some(index) = direction.iter().position(|dir| dir == &last_dir.opposite()) {
                direction.remove(index);
            };

            if self.prev_directions.len() >= *max_in_direction {
                let mut slice = self.prev_directions.iter().rev().take(*max_in_direction);
                if slice.all(|dir| dir == last_dir) {
                    if let Some(index) = direction.iter().position(|dir| dir == last_dir) {
                        direction.remove(index);
                    };
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
        heuristic_current_score: Some(0),
        heat_loss: 1,
        is_target: false,
        prev_directions: vec![],
        coord: Coordinate { x: 0, y: 0 },
        allowed_visits_from: Direction::n_full_sets(&3),
    };
    assert_eq!(
        node.find_directions(&3),
        Vec::from([
            Direction::Right,
            Direction::Down,
            Direction::Left,
            Direction::Up,
        ])
    );

    let node = Node {
        current_score: Some(0),
        heuristic_current_score: Some(0),
        heat_loss: 1,
        is_target: false,
        prev_directions: vec![Direction::Right],
        coord: Coordinate { x: 0, y: 0 },
        allowed_visits_from: Direction::n_full_sets(&3),
    };
    assert_eq!(
        node.find_directions(&3),
        Vec::from([Direction::Right, Direction::Down, Direction::Up,])
    );

    let node = Node {
        current_score: Some(0),
        heuristic_current_score: Some(0),
        heat_loss: 1,
        is_target: false,
        prev_directions: vec![Direction::Right, Direction::Right, Direction::Right],
        coord: Coordinate { x: 0, y: 0 },
        allowed_visits_from: Direction::n_full_sets(&3),
    };
    assert_eq!(
        node.find_directions(&3),
        Vec::from([Direction::Down, Direction::Up,])
    );
}

impl Grid {
    pub fn calculate_neighbors(&mut self, node: &Node) -> (Vec<Node>, Option<u64>) {
        // println!("{:?}", node);
        let directions = node.find_directions(&(self.max_repeat as usize));

        let mut unvisited = Vec::new();

        for direction in directions {
            let mut current_node = node.clone();
            // for _ in 0..self.max_repeat {
            match self.set_next_node(&direction, &current_node, &mut unvisited) {
                (_, Some(value)) => return (Vec::new(), Some(value)),
                (Some(node), _) => current_node = node,
                (None, None) => (),
            };
            // }
        }
        let this_node_in_grid = &mut self.data[node.coord.y][node.coord.x];
        // if let Some(direction) = node.prev_directions.iter().last() {
        //     this_node_in_grid.allowed_visits_from.remove(direction);
        // }
        (unvisited, None)
    }

    fn set_next_node(
        &mut self,
        direction: &Direction,
        node: &Node,
        unvisited: &mut Vec<Node>,
    ) -> (Option<Node>, Option<u64>) {
        let allowed = node.find_directions(&(self.max_repeat as usize));

        if allowed.contains(direction) == false {
            return (None, None);
        }

        let (y_offset, x_offset) = match direction {
            Direction::Down => (1, 0),
            Direction::Up => (-1, 0),
            Direction::Right => (0, 1),
            Direction::Left => (0, -1),
            _ => panic!(),
        };
        let new_x = get_coords(node.coord.x, x_offset, self.data[0].len());
        let new_y = get_coords(node.coord.y, y_offset, self.data.len());
        let (x_len, y_len) = { (self.data[0].len(), self.data.len()) };

        match (new_x, new_y) {
            (Some(x), Some(y)) => {
                let next_node = &mut self.data[y][x];
                // if next_node.allowed_visits_from.contains(&direction) {
                // if (node.coord.x == 11 && node.coord.y == 7) {
                //     println!(
                //         "This should be able to reach {},{}",
                //         node.coord.y, node.coord.x
                //     );
                // }
                // if (x == 11 && y == 7) {
                //     println!("Evaluating my problematic node");
                // }
                let heuristic = (x_len - (x * 1).min(x_len) + y_len - (y * 1).min(y_len)) as u64;
                let potential_score = node.current_score.unwrap() + next_node.heat_loss as u64;
                if next_node.is_target == true {
                    println!("{:?}", node.prev_directions);
                    return (None, Some(potential_score));
                }

                let prev_directions = [node.prev_directions.as_slice(), &[*direction]].concat();
                let direction_based_y: isize = prev_directions
                    .iter()
                    .map(|dir| {
                        if *dir == Direction::Down {
                            1
                        } else if *dir == Direction::Up {
                            -1
                        } else {
                            0
                        }
                    })
                    .sum();
                assert_eq!(direction_based_y, y as isize);

                let direction_based_x: isize = prev_directions
                    .iter()
                    .map(|dir| {
                        if *dir == Direction::Right {
                            1
                        } else if *dir == Direction::Left {
                            -1
                        } else {
                            0
                        }
                    })
                    .sum();
                assert_eq!(direction_based_x, x as isize);
                // Update grid and push to queue, but only if score is better
                let last_dir_count =
                    get_dir_count(&prev_directions, direction, &self.max_repeat.into());

                let new_node = Node {
                    current_score: Some(potential_score),
                    heuristic_current_score: Some(potential_score + heuristic),
                    heat_loss: next_node.heat_loss,
                    is_target: false,
                    prev_directions,
                    coord: Coordinate { x, y },
                    allowed_visits_from: next_node.allowed_visits_from.clone(),
                };

                if next_node
                    .allowed_visits_from
                    .get(&last_dir_count)
                    .unwrap()
                    .contains(direction)
                {
                    if (next_node.heuristic_current_score.is_some()
                        && potential_score + heuristic
                            <= next_node.heuristic_current_score.unwrap()
                        || next_node.current_score.is_none())
                    {
                        next_node.current_score = Some(potential_score);
                        next_node.heuristic_current_score = Some(potential_score + heuristic);
                    }
                    next_node
                        .allowed_visits_from
                        .get_mut(&last_dir_count)
                        .unwrap()
                        .remove(direction);
                    unvisited.push(new_node.clone());
                }

                return (Some(new_node), None);
            }
            _ => (),
        }
        (None, None)
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

fn get_dir_count(
    prev_directions: &Vec<Direction>,
    direction: &Direction,
    max_repeat: &usize,
) -> u8 {
    let mut iter = prev_directions
        .iter()
        .rev()
        .take_while(|dir| *dir == direction);
    let mut current = 0;
    for i in 1..*max_repeat {
        if let Some(dir) = iter.next() {
            current = i as u8;
        }
    }
    current
}

#[test]
fn get_right_dir_counts() {
    assert_eq!(get_dir_count(&vec![Direction::Up], &Direction::Up, &3), 1);
}

#[test]
fn calculates_neighbors() {
    let mut grid = Grid {
        data: vec![
            vec![
                Node {
                    current_score: Some(0),
                    heuristic_current_score: Some(0),
                    heat_loss: 1,
                    is_target: false,
                    prev_directions: vec![],
                    coord: Coordinate { x: 0, y: 0 },
                    allowed_visits_from: Direction::n_full_sets(&3),
                },
                Node {
                    current_score: None,
                    heuristic_current_score: None,
                    heat_loss: 2,
                    is_target: false,
                    prev_directions: vec![],
                    coord: Coordinate { x: 1, y: 0 },
                    allowed_visits_from: Direction::n_full_sets(&3),
                },
                Node {
                    current_score: None,
                    heuristic_current_score: None,
                    heat_loss: 2,
                    is_target: false,
                    prev_directions: vec![],
                    coord: Coordinate { x: 2, y: 0 },
                    allowed_visits_from: Direction::n_full_sets(&3),
                },
            ],
            vec![
                Node {
                    current_score: None,
                    heuristic_current_score: None,
                    heat_loss: 4,
                    is_target: false,
                    prev_directions: vec![],
                    coord: Coordinate { x: 0, y: 1 },
                    allowed_visits_from: Direction::n_full_sets(&3),
                },
                Node {
                    current_score: None,
                    heuristic_current_score: None,
                    heat_loss: 5,
                    is_target: true,
                    prev_directions: vec![],
                    coord: Coordinate { x: 1, y: 1 },
                    allowed_visits_from: Direction::n_full_sets(&3),
                },
                Node {
                    current_score: None,
                    heuristic_current_score: None,
                    heat_loss: 2,
                    is_target: true,
                    prev_directions: vec![],
                    coord: Coordinate { x: 2, y: 1 },
                    allowed_visits_from: Direction::n_full_sets(&3),
                },
            ],
        ],
        min_repeat: 1,
        max_repeat: 3,
    };

    let mut result = grid.calculate_neighbors(&grid.data[0][0].clone());

    Node::sort_by_score(&mut result.0);

    assert_eq!(
        result,
        (
            vec![
                Node {
                    current_score: Some(4),
                    heuristic_current_score: Some(8),
                    heat_loss: 4,
                    is_target: false,
                    prev_directions: vec![Direction::Down],
                    coord: Coordinate { x: 0, y: 1 },
                    allowed_visits_from: Direction::n_full_sets(&3),
                },
                Node {
                    current_score: Some(2),
                    heuristic_current_score: Some(6),
                    heat_loss: 2,
                    is_target: false,
                    prev_directions: vec![Direction::Right],
                    coord: Coordinate { x: 1, y: 0 },
                    allowed_visits_from: Direction::n_full_sets(&3),
                },
            ],
            None
        )
    );

    let result = grid.calculate_neighbors(&result.0[1]);
    assert_eq!(result, (vec![], Some(7)));
}
