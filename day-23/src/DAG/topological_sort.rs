use super::*;

impl Maze {
    pub(crate) fn topological_sort(&mut self) {
        let mut sorted_nodes = Vec::new();
        let mut no_incoming_edges = self.nodes.keys().cloned().collect::<HashSet<Coordinate>>();
        let mut incoming_edges_count = HashMap::new();

        let mut edges = self.edges.clone();

        // Initialize incoming edges count
        for edge in &self.edges {
            *incoming_edges_count
                .entry(edge.ending_node_loc)
                .or_insert(0) += 1;
            no_incoming_edges.remove(&edge.ending_node_loc);
        }

        while let Some(node) = no_incoming_edges.iter().cloned().next() {
            no_incoming_edges.remove(&node);
            sorted_nodes.push(node);

            let mut edges_to_remove = Vec::new();
            for edge in self.edges.clone() {
                if edge.starting_node_loc == node {
                    *incoming_edges_count
                        .entry(edge.ending_node_loc)
                        .or_insert(0) -= 1;
                    if incoming_edges_count[&edge.ending_node_loc] == 0 {
                        no_incoming_edges.insert(edge.ending_node_loc);
                    }
                    edges_to_remove.push(edge.clone());
                }
            }
            edges.retain(|e| !edges_to_remove.contains(e));
        }

        if !edges.is_empty() {
            // Graph has at least one cycle, topological sorting not possible
            panic!("Graph has a cycle, topological sorting not possible");
        }

        self.sorted_nodes = sorted_nodes;
    }
}

#[test]
fn sorts_in_topological_order() {
    let mut maze = Maze {
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
        sorted_nodes: vec![],
    };

    maze.topological_sort();

    assert_eq!(
        maze.sorted_nodes,
        vec![
            Coordinate { x: 1, y: 0 },
            Coordinate { x: 2, y: 3 },
            Coordinate { x: 4, y: 4 }
        ]
    );
}
