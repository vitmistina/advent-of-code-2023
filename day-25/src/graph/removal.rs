use super::Graph;

impl Graph {
    pub fn remove_edge(&mut self, node1: &str, node2: &str) {
        let mut nodes = vec![node1.to_string(), node2.to_string()];
        nodes.sort();

        self.edges.retain(|edge| edge.nodes != nodes);
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::graph::Edge;

    use super::*;

    #[test]
    fn removes_edge() {
        let mut graph = Graph {
            edges: vec![("a", "b"), ("b", "c"), ("c", "d"), ("x", "y")]
                .into_iter()
                .map(|e| Edge {
                    nodes: vec![e.0.to_string(), e.1.to_string()],
                    weight: 1,
                })
                .collect(),
            nodes: HashSet::new(),
        };
        graph.remove_edge("b", "c");
        assert_eq!(graph.edges.len(), 3);

        graph.remove_edge("b", "c");
        assert_eq!(graph.edges.len(), 3);

        // reverse order still works
        graph.remove_edge("d", "c");
        assert_eq!(graph.edges.len(), 2);
    }
}
