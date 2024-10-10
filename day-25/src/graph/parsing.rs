use super::{Edge, Graph, Node};

impl Graph {
    pub fn from_input(input: &str) -> Self {
        let mut graph = Graph {
            edges: Default::default(),
            nodes: Default::default(),
        };

        for line in input.lines() {
            let mut parts = line.split(": ");
            let node = parts.next().unwrap().trim();
            let neighbors = parts.next().unwrap_or_default();

            for neighbor in neighbors.split(' ').map(str::trim) {
                let mut nodes = vec![node.to_string(), neighbor.to_string()];
                nodes.sort();

                let edge = Edge { nodes, weight: 1 };

                if !graph.edges.contains(&edge) {
                    graph.edges.insert(edge);
                };

                let node = Node {
                    id: neighbor.to_string(),
                    contains: vec![],
                };

                if !graph.nodes.contains(&node) {
                    graph.nodes.insert(node);
                };
            }

            let node = Node {
                id: node.to_string(),
                contains: vec![],
            };

            if !graph.nodes.contains(&node) {
                graph.nodes.insert(node);
            };
        }

        graph
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn parses_input_with_duplicate_edges() {
        // This input has particular two nodes, jqt and xhk, with an edge between them.
        // The edge is bidirectional, so there should be only one entry in the edges set.
        let input = "jqt: rhn xhk nvd
        xhk: jqt kqr\n";
        let graph = Graph::from_input(input);

        let expected_edges: HashSet<Edge> = [
            Edge {
                nodes: vec!["jqt".to_string(), "rhn".to_string()],
                weight: 1,
            },
            Edge {
                nodes: vec!["jqt".to_string(), "xhk".to_string()],
                weight: 1,
            },
            Edge {
                nodes: vec!["jqt".to_string(), "nvd".to_string()],
                weight: 1,
            },
            Edge {
                nodes: vec!["kqr".to_string(), "xhk".to_string()],
                weight: 1,
            },
        ]
        .iter()
        .cloned()
        .collect();

        let expected_nodes: HashSet<Node> = [
            Node {
                id: "jqt".to_string(),
                contains: vec![],
            },
            Node {
                id: "xhk".to_string(),
                contains: vec![],
            },
            Node {
                id: "kqr".to_string(),
                contains: vec![],
            },
            Node {
                id: "nvd".to_string(),
                contains: vec![],
            },
            Node {
                id: "rhn".to_string(),
                contains: vec![],
            },
        ]
        .iter()
        .cloned()
        .collect();

        assert_eq!(graph.edges, expected_edges);
        assert_eq!(graph.nodes, expected_nodes);
    }

    #[test]
    fn parses_test_input() {
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
        frs: qnr lhk lsr\n";
        let graph = Graph::from_input(input);
        assert_eq!(graph.edges.len(), 33);
        assert_eq!(graph.nodes.len(), 15);
    }
}
