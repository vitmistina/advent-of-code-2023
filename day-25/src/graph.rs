use std::collections::{HashMap, HashSet};

use std::fs::File;
use std::io::{self, Write};

mod communities;
mod export;
mod parsing;
mod removal;

#[derive(Debug, Clone)]
pub(super) struct Graph {
    edges: HashSet<Edge>,
    nodes: HashSet<Node>,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub(super) struct Edge {
    nodes: Vec<String>,
    weight: usize,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct Node {
    id: String,
    contains: Vec<String>,
}
impl Node {
    fn get_previous_nodes(&self) -> Vec<String> {
        if self.contains.is_empty() {
            return vec![self.id.clone()];
        } else {
            return self.contains.clone();
        }
    }
}

impl Graph {
    pub fn export(input: &str) {
        let graph = Graph::from_input(input);
        graph.export_to_graph_ml();
    }

    pub fn sort_edge_scores(&self) -> Vec<(Edge, usize)> {
        let mut edge_scores = Vec::new();

        for edge in &self.edges {
            let score = self.calculate_connectivity_score(&edge);
            edge_scores.push((edge.clone(), score));
        }

        edge_scores.sort_by(|a, b| a.1.cmp(&b.1));
        edge_scores
    }

    fn calculate_node_degrees(&self) -> HashMap<String, usize> {
        let mut node_degrees = HashMap::new();

        for edge in &self.edges {
            for node in &edge.nodes {
                *node_degrees.entry(node.clone()).or_insert(0) += edge.weight;
            }
        }

        node_degrees
    }

    pub fn find_cut_product(&mut self) -> Option<usize> {
        let mut generation = 0;
        // let mut file = File::create("output.txt").expect("Unable to create file");

        // while let Some(edge) = self.find_least_connected_edge(Some(&mut file)) {
        while let Some(edge) = self.find_least_connected_edge(None) {
            let new_id = generation.to_string();
            self.merge_nodes(&edge.nodes[0], &edge.nodes[1], &new_id);

            println!(
                "Generation {}, Nodes count {}",
                generation,
                self.nodes.len()
            );

            // writeln!(file, "Least connected edge: {:#?}", edge).expect("Unable to write to file");
            // writeln!(file, "Generation {}: {:#?}", generation, self)
            //     .expect("Unable to write to file");

            generation += 1;

            let node_degrees = self.calculate_node_degrees();

            if let Some((three_degree_node, _)) = node_degrees.iter().find(|&(_, v)| *v == 3) {
                let three_degree_node = self
                    .nodes
                    .iter()
                    .find(|n| n.id == *three_degree_node)
                    .unwrap();
                println!("Found a node with degree 3: {:?}", three_degree_node);
                return Some(
                    three_degree_node.get_previous_nodes().len()
                        * self
                            .nodes
                            .iter()
                            .filter(|n| n.id != three_degree_node.id)
                            .map(|n| n.get_previous_nodes().len())
                            .sum::<usize>(),
                );
            }
            // if self.edges.len() == 1 || node_degrees.values().any(|&v| v == 3) {
            //     println!("Found a node with degree 3 or only one edge left");
            //     println!("Degrees: {:#?}", node_degrees);
            //     return Some(
            //         self.nodes
            //             .iter()
            //             .map(|n| n.get_previous_nodes().len())
            //             .product(),
            //     );
            // }

            // if generation > 20 {
            //     break;đ
            // }
        }

        None
    }

    fn find_node_by_id(&self, id: &str) -> Option<Node> {
        self.nodes.iter().find(|n| n.id == id).cloned()
    }

    fn find_least_connected_edge(&self, mut file: Option<&mut File>) -> Option<Edge> {
        let mut current_min = usize::MAX;
        let mut current_max_weight = usize::MIN;
        let mut best_edge = None;

        let scores = self.sort_edge_scores();

        if let Some(file) = &mut file {
            writeln!(file, "Scores:").expect("Unable to write to file");
            for score in &scores {
                writeln!(file, "{} {:?}", score.1, score.0.nodes).expect("Unable to write to file");
            }
        }

        for score in scores {
            let (edge, connectivity_score) = score;
            if edge.weight > 3 {
                return Some(edge.clone());
            }

            if connectivity_score < current_min
                || connectivity_score == current_min && edge.weight > current_max_weight
            {
                if connectivity_score == current_min {
                    current_max_weight = edge.weight;
                }
                current_min = connectivity_score;
                best_edge = Some(edge.clone());
            }
        }

        best_edge
    }

    pub fn calculate_connectivity_score(&self, edge: &Edge) -> usize {
        let mut other_nodes = HashSet::new();

        for node in &edge.nodes {
            other_nodes.extend(self.neighbors(node));
        }

        let connectivity_score = other_nodes
            .iter()
            .filter(|other_node| !edge.nodes.contains(other_node))
            .map(|other_node| {
                self.edges
                    .iter()
                    .filter(|e| {
                        e.nodes.contains(other_node)
                            && (e.nodes.contains(&edge.nodes[0])
                                || e.nodes.contains(&edge.nodes[1]))
                    })
                    .map(|e| e.weight)
                    .sum::<usize>()
            })
            .sum::<usize>();
        // - self
        //     .find_node_by_id(&edge.nodes[0])
        //     .unwrap()
        //     .get_previous_nodes()
        //     .len()
        // - self
        //     .find_node_by_id(&edge.nodes[1])
        //     .unwrap()
        //     .get_previous_nodes()
        //     .len();
        // - edge.weight;
        connectivity_score
    }

    pub fn merge_nodes(&mut self, node1: &str, node2: &str, new_id: &str) {
        let node1 = self.nodes.iter().find(|n| n.id == node1).cloned();
        let node2 = self.nodes.iter().find(|n| n.id == node2).cloned();

        if let (Some(node1), Some(node2)) = (node1, node2) {
            let mut new_node = Node {
                id: new_id.to_string(),
                contains: vec![node1.get_previous_nodes(), node2.get_previous_nodes()].concat(),
            };

            let mut new_edges = HashSet::new();
            let mut edge_weights: HashMap<(String, String), usize> = HashMap::new();

            for edge in &self.edges {
                let mut new_edge = edge.clone();
                let mut nodes = new_edge.nodes.clone();
                let weight = new_edge.weight;

                if nodes.contains(&node1.id) || nodes.contains(&node2.id) {
                    nodes = nodes
                        .iter()
                        .map(|n| {
                            if n == &node1.id || n == &node2.id {
                                new_id.to_string()
                            } else {
                                n.clone()
                            }
                        })
                        .collect();

                    let key = if nodes[0] < nodes[1] {
                        (nodes[0].clone(), nodes[1].clone())
                    } else {
                        (nodes[1].clone(), nodes[0].clone())
                    };

                    if key.0 != key.1 {
                        // Ensure no self-loop is created
                        *edge_weights.entry(key).or_insert(0) += weight;
                    }
                } else {
                    new_edges.insert(new_edge);
                }
            }

            for ((node_a, node_b), weight) in edge_weights {
                new_edges.insert(Edge {
                    nodes: vec![node_a, node_b],
                    weight,
                });
            }

            self.nodes.retain(|n| n.id != node1.id && n.id != node2.id);

            self.nodes.insert(new_node);
            self.edges = new_edges;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn runs_sample() {
        let input = "jqt: rhn xhk nvd
        rsh: frs pzl lsr
        xhk: hfx
        cmg: qnr nvd lhk bvb
        rhn: xhk bvb hfx
        bvb: xhk hfx
        pzl: lsr hfx nvd
        qnr: nvd
        ntq: jqt hfx bvb xhk
        nvd: lhk
        lsr: lhk
        rzs: qnr cmg lsr rsh
        frs: qnr lhk lsr";
        let mut graph = Graph::from_input(input);
        graph.remove_edge("hfx", "pzl");
        graph.remove_edge("bvb", "cmg");
        graph.remove_edge("nvd", "jqt");
        assert_eq!(graph.find_two_community_product(), Some(54));
    }

    #[test]
    fn runs_merging_algorithm() {
        let input = "jqt: rhn xhk nvd
        rsh: frs pzl lsr
        xhk: hfx
        cmg: qnr nvd lhk bvb
        rhn: xhk bvb hfx
        bvb: xhk hfx
        pzl: lsr hfx nvd
        qnr: nvd
        ntq: jqt hfx bvb xhk
        nvd: lhk
        lsr: lhk
        rzs: qnr cmg lsr rsh
        frs: qnr lhk lsr";
        let mut graph = Graph::from_input(input);
        assert_eq!(graph.find_cut_product(), Some(54));
    }

    #[test]
    fn finds_least_connected_edge() {
        let input = "a: b e d c
        b: c d
        c: d e
        x: y";
        let graph = Graph::from_input(input);
        let result = graph.find_least_connected_edge(None).unwrap();
        assert_eq!(result.weight, 1);
    }

    #[test]
    fn finds_least_connected_edge_on_provided_sample() {
        let input = "jqt: rhn xhk nvd
        rsh: frs pzl lsr
        xhk: hfx
        cmg: qnr nvd lhk bvb
        rhn: xhk bvb hfx
        bvb: xhk hfx
        pzl: lsr hfx nvd
        qnr: nvd
        ntq: jqt hfx bvb xhk
        nvd: lhk
        lsr: lhk
        rzs: qnr cmg lsr rsh
        frs: qnr lhk lsr";
        let graph = Graph::from_input(input);
        let result = graph.find_least_connected_edge(None).unwrap();
        assert_eq!(result.weight, 1);
    }

    #[test]
    fn calculates_connectivity_score() {
        let graph = Graph {
            edges: vec![
                Edge {
                    nodes: vec!["a".to_string(), "b".to_string()],
                    weight: 1,
                },
                Edge {
                    nodes: vec!["a".to_string(), "c".to_string()],
                    weight: 10,
                },
                Edge {
                    nodes: vec!["b".to_string(), "c".to_string()],
                    weight: 1,
                },
                Edge {
                    nodes: vec!["b".to_string(), "d".to_string()],
                    weight: 1,
                },
                Edge {
                    nodes: vec!["c".to_string(), "d".to_string()],
                    weight: 1,
                },
                Edge {
                    nodes: vec!["d".to_string(), "e".to_string()],
                    weight: 1,
                },
            ]
            .into_iter()
            .collect(),
            nodes: HashSet::new(),
        };
        let edge = Edge {
            nodes: vec!["a".to_string(), "b".to_string()],
            weight: 1,
        };
        let result = graph.calculate_connectivity_score(&edge);
        assert_eq!(result, 12);
    }

    #[test]
    fn adds_previous_nodes() {
        let new_node = Node {
            id: "a".to_string(),
            contains: vec![],
        };
        let expected_output = vec!["a".to_string()];
        let result = new_node.get_previous_nodes();
        assert_eq!(result, expected_output);

        let full_node = Node {
            id: "1".to_string(),
            contains: vec!["b".to_string(), "c".to_string()],
        };
        let expected_output = vec!["b".to_string(), "c".to_string()];
        let result = full_node.get_previous_nodes();
        assert_eq!(result, expected_output);
    }

    #[test]
    fn merges_nodes() {
        let new_id = "becc7b18-66d3-481c-9f37-d112025ac25e".to_string();
        let input = "a: b
        b: c e
        c: d e
        x: y";
        let mut graph = Graph::from_input(input);
        assert_eq!(graph.nodes.len(), 7);

        graph.merge_nodes("b", "c", &new_id);
        let expected_edges = HashSet::from([
            Edge {
                nodes: vec!["a".to_string(), new_id.clone()],
                weight: 1,
            },
            Edge {
                nodes: vec![new_id.clone(), "e".to_string()],
                weight: 2,
            },
            Edge {
                nodes: vec![new_id.clone(), "d".to_string()],
                weight: 1,
            },
            Edge {
                nodes: vec!["x".to_string(), "y".to_string()],
                weight: 1,
            },
        ]);

        let expected_nodes = HashSet::from([
            Node {
                id: "a".to_string(),
                contains: vec![],
            },
            Node {
                id: new_id.clone(),
                contains: vec!["b".to_string(), "c".to_string()],
            },
            Node {
                id: "d".to_string(),
                contains: vec![],
            },
            Node {
                id: "e".to_string(),
                contains: vec![],
            },
            Node {
                id: "x".to_string(),
                contains: vec![],
            },
            Node {
                id: "y".to_string(),
                contains: vec![],
            },
        ]);

        assert_eq!(graph.edges.len(), 4);
        assert_eq!(graph.edges, expected_edges);
        assert_eq!(graph.nodes.len(), 6);
        assert_eq!(graph.nodes, expected_nodes);
    }
}
