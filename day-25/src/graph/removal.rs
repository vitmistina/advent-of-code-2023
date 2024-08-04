use super::Graph;

impl Graph {
    pub fn remove_edge(&mut self, node1: &str, node2: &str) {
        self.edges.remove(&(node1.to_string(), node2.to_string()));
        self.edges.remove(&(node2.to_string(), node1.to_string()));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn removes_edge() {
        let mut graph = Graph {
            edges: vec![("a", "b"), ("b", "c"), ("c", "d")]
                .into_iter()
                .map(|e| (e.0.to_string(), e.1.to_string()))
                .collect(),
        };
        graph.remove_edge("b", "c");
        assert_eq!(graph.edges.len(), 2);

        // reverse order still works
        graph.remove_edge("d", "c");
        assert_eq!(graph.edges.len(), 1);
    }
}
