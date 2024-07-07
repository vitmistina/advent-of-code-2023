use std::collections::VecDeque;

use super::*;

// there is only one node of type Crossroad which is before the Finish node, find it and mark it as type PreFinish
impl Maze {
    fn find_prefinish(&mut self) -> () {
        let finish_node_coord = self
            .nodes
            .iter()
            .find(|(_, node)| node.node_type == NodeType::Finish)
            .map(|(coord, _)| coord)
            .unwrap();

        let prefinish_node_coord = self
            .edges
            .iter()
            .find_map(|edge| {
                if edge.ending_node_loc == *finish_node_coord {
                    Some(edge.starting_node_loc.clone())
                } else {
                    None
                }
            })
            .unwrap();

        let prefinish_node = self.nodes.get_mut(&prefinish_node_coord).unwrap();
        prefinish_node.node_type = NodeType::PreFinish;
    }

    fn find_start_coord(&self) -> Coordinate {
        self.nodes
            .iter()
            .find(|(_, node)| node.node_type == NodeType::Start)
            .map(|(coord, _)| coord.clone())
            .unwrap()
    }

    fn find_edges_from_coord(&self, start: &Coordinate) -> Vec<Edge> {
        self.edges
            .iter()
            .filter(|edge| edge.starting_node_loc == *start)
            .cloned()
            .collect()
    }
}

pub(crate) struct Pathfinder {
    maze: Maze,
    queue: VecDeque<Path>,
    longest_path: Option<Path>,
}

impl Pathfinder {
    pub(crate) fn new(maze: &Maze) -> Pathfinder {
        Pathfinder {
            maze: maze.clone(),
            queue: VecDeque::new(),
            longest_path: None,
        }
    }

    fn process_node(&mut self, path: &Path) -> () {
        let coord = path.next_node.unwrap();
        let edges = self.maze.find_edges_from_coord(&coord);
        for edge in edges {
            let next_node = self.maze.nodes.get(&edge.ending_node_loc).unwrap();
            if path.visited_nodes.contains(&edge.ending_node_loc) {
                continue;
            }
            if next_node.node_type == NodeType::PreFinish
                && path.visited_nodes.len() < self.maze.nodes.len() - 3
            {
                continue;
            }
            let mut new_path = path.clone();
            new_path.visited_nodes.push(coord);
            new_path.length += edge.length;
            new_path.next_node = Some(edge.ending_node_loc);

            if next_node.node_type == NodeType::Finish {
                if self.longest_path.is_none() {
                    self.longest_path = Some(new_path.clone());
                } else if new_path.length > self.longest_path.as_ref().unwrap().length {
                    self.longest_path = Some(new_path.clone());
                }
            }

            self.queue.push_back(new_path);
        }
    }

    pub(crate) fn find_longest_path(&mut self) -> usize {
        let start = self.maze.find_start_coord();
        let path = Path {
            visited_nodes: vec![],
            length: 0,
            next_node: Some(start),
        };
        self.queue.push_back(path);

        let mut counter = 0;
        while !self.queue.is_empty() {
            let path = self.queue.pop_front().unwrap();
            self.process_node(&path);
            counter += 1;
            if (counter % 100000) == 0 {
                println!("Processed {} paths", counter);
            }
        }

        self.longest_path.as_ref().unwrap().length
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Path {
    visited_nodes: Vec<Coordinate>,
    length: usize,
    next_node: Option<Coordinate>,
}

// start at node type Start
// find all edges that start at Start Coordinate
// for each edge, find the next node and put it in a queue including the total edge length
// skip adding a node to the queue if the node is already visited
// skip adding a node to the queue if the node is of type PreFinish AND the count of visited nodes is less than the total number of nodes minus 1 (all Crossroads must be visited before the PreFinish node)
// take some node from the queue in a way which is most efficient for the execution
// repeat until the queue is empty

#[test]
fn finds_prefinish() {
    let mut maze = create_test_maze();
    maze.find_prefinish();
    let prefinish = maze.nodes.get(&Coordinate { x: 2, y: 3 }).unwrap();
    assert_eq!(prefinish.node_type, NodeType::PreFinish);
}

#[test]
fn find_edges_from_coord() {
    let maze = create_test_maze();
    let start = Coordinate { x: 1, y: 0 };
    let edges = maze.find_edges_from_coord(&start);
    assert_eq!(edges.len(), 3);
}

#[test]
fn finds_start() {
    let maze = create_test_maze();
    let start = maze.find_start_coord();
    assert_eq!(start, Coordinate { x: 1, y: 0 });
}

fn create_test_maze() -> Maze {
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
                Coordinate { x: 1, y: 3 },
                Node {
                    id: 2,
                    is_visited: false,
                    node_type: NodeType::Crossroad,
                    exits: vec![Direction::Left, Direction::Right],
                },
            ),
            (
                Coordinate { x: 4, y: 4 },
                Node {
                    id: 3,
                    is_visited: false,
                    node_type: NodeType::Finish,
                    exits: vec![],
                },
            ),
            (
                Coordinate { x: 0, y: 1 },
                Node {
                    id: 4,
                    is_visited: false,
                    node_type: NodeType::Crossroad,
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
                ending_node_id: 2,
                starting_node_loc: Coordinate { x: 1, y: 0 },
                ending_node_loc: Coordinate { x: 1, y: 3 },
                length: 3,
            },
            Edge {
                starting_node_id: 2,
                ending_node_id: 1,
                starting_node_loc: Coordinate { x: 1, y: 3 },
                ending_node_loc: Coordinate { x: 2, y: 3 },
                length: 4,
            },
            Edge {
                starting_node_id: 1,
                ending_node_id: 3,
                starting_node_loc: Coordinate { x: 2, y: 3 },
                ending_node_loc: Coordinate { x: 4, y: 4 },
                length: 3,
            },
            Edge {
                starting_node_id: 0,
                ending_node_id: 4,
                starting_node_loc: Coordinate { x: 1, y: 0 },
                ending_node_loc: Coordinate { x: 0, y: 1 },
                length: 1,
            },
        ],
        sorted_nodes: vec![
            Coordinate { x: 1, y: 0 },
            Coordinate { x: 1, y: 3 },
            Coordinate { x: 2, y: 3 },
            Coordinate { x: 4, y: 4 },
        ],
    };
    maze
}

#[test]
fn processes_node() {
    let mut maze = create_test_maze();

    // set Coordinate { x: 2, y: 3 } to PreFinish
    maze.nodes
        .get_mut(&Coordinate { x: 2, y: 3 })
        .unwrap()
        .node_type = NodeType::PreFinish;

    let start = Coordinate { x: 1, y: 0 };
    let mut pathfinder = Pathfinder::new(&maze);
    let path = Path {
        visited_nodes: vec![Coordinate { x: 0, y: 1 }],
        length: 0,
        next_node: Some(start),
    };

    pathfinder.process_node(&path.clone());

    assert_eq!(pathfinder.queue.len(), 1);
    let path = pathfinder.queue.pop_front().unwrap();
    assert_eq!(path.length, 3);
    assert_eq!(path.visited_nodes.len(), 2);
    assert_eq!(path.next_node, Some(Coordinate { x: 1, y: 3 }));
}

#[test]
fn processes_node_and_accepts_prefinish() {
    let mut maze = create_test_maze();

    // set Coordinate { x: 2, y: 3 } to PreFinish
    maze.nodes
        .get_mut(&Coordinate { x: 2, y: 3 })
        .unwrap()
        .node_type = NodeType::PreFinish;

    let start = Coordinate { x: 1, y: 0 };
    let mut pathfinder = Pathfinder::new(&maze);
    let path = Path {
        visited_nodes: vec![
            Coordinate { x: 0, y: 1 },
            Coordinate { x: 1, y: 0 },
            Coordinate { x: 1, y: 3 },
        ],
        length: 0,
        next_node: Some(start),
    };

    pathfinder.process_node(&path.clone());

    assert_eq!(pathfinder.queue.len(), 1);
    let path = pathfinder.queue.pop_front().unwrap();
    assert_eq!(path.length, 4);
    assert_eq!(path.visited_nodes.len(), 4);
    assert_eq!(path.next_node, Some(Coordinate { x: 2, y: 3 }));
}

#[test]
fn finds_longest_path() {
    let mut maze = Maze {
        grid: vec![],
        nodes: HashMap::from([
            (
                Coordinate { x: 1, y: 0 },
                Node {
                    id: 0,
                    is_visited: false,
                    node_type: NodeType::Start,
                    exits: vec![],
                },
            ),
            (
                Coordinate { x: 2, y: 3 },
                Node {
                    id: 1,
                    is_visited: false,
                    node_type: NodeType::Crossroad,
                    exits: vec![],
                },
            ),
            (
                Coordinate { x: 1, y: 3 },
                Node {
                    id: 2,
                    is_visited: false,
                    node_type: NodeType::Crossroad,
                    exits: vec![],
                },
            ),
            (
                Coordinate { x: 4, y: 4 },
                Node {
                    id: 3,
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
                ending_node_id: 2,
                starting_node_loc: Coordinate { x: 1, y: 0 },
                ending_node_loc: Coordinate { x: 1, y: 3 },
                length: 3,
            },
            Edge {
                starting_node_id: 2,
                ending_node_id: 1,
                starting_node_loc: Coordinate { x: 1, y: 3 },
                ending_node_loc: Coordinate { x: 2, y: 3 },
                length: 4,
            },
            Edge {
                starting_node_id: 1,
                ending_node_id: 3,
                starting_node_loc: Coordinate { x: 2, y: 3 },
                ending_node_loc: Coordinate { x: 4, y: 4 },
                length: 7,
            },
        ],
        sorted_nodes: vec![],
    };
    maze.find_prefinish();
    let mut pathfinder = Pathfinder::new(&maze);
    let longest_path = pathfinder.find_longest_path();
    assert_eq!(longest_path, 14);
}
