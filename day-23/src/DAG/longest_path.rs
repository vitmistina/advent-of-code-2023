use super::*;

impl Maze {
    pub(crate) fn find_longest_path(&self) -> usize {
        let mut distances = HashMap::new();

        // Initialize distances for all nodes
        for node in self.nodes.keys() {
            distances.insert(
                node,
                if node == &self.sorted_nodes[0] {
                    0
                } else {
                    isize::MIN
                },
            );
        }

        // Relax edges along the topologically sorted nodes
        for current_node in &self.sorted_nodes {
            for edge in &self.edges {
                if &edge.starting_node_loc == current_node {
                    let distance_through_current =
                        distances[current_node].saturating_add(edge.length as isize);
                    let distance_to_target = distances.get_mut(&edge.ending_node_loc).unwrap();

                    if *distance_to_target < distance_through_current {
                        *distance_to_target = distance_through_current;
                    }
                }
            }
        }

        // Find the maximum distance
        *distances.values().max().unwrap() as usize
    }
}

#[test]
fn finds_longest_path() {
    let maze = Maze {
        grid: vec![],
        nodes: HashMap::from([
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
        ]),
        edges: vec![
            Edge {
                starting_node_id: 0,
                ending_node_id: 1,
                starting_node_loc: Coordinate { x: 1, y: 0 },
                ending_node_loc: Coordinate { x: 2, y: 3 },
                length: 4,
            },
            Edge {
                starting_node_id: 0,
                ending_node_id: 1,
                starting_node_loc: Coordinate { x: 2, y: 3 },
                ending_node_loc: Coordinate { x: 4, y: 4 },
                length: 3,
            },
            Edge {
                starting_node_id: 0,
                ending_node_id: 1,
                starting_node_loc: Coordinate { x: 2, y: 3 },
                ending_node_loc: Coordinate { x: 4, y: 4 },
                length: 7,
            },
        ],
        sorted_nodes: vec![
            Coordinate { x: 1, y: 0 },
            Coordinate { x: 2, y: 3 },
            Coordinate { x: 4, y: 4 },
        ],
    };

    let result = maze.find_longest_path();

    assert_eq!(result, 4 + 7);
}
