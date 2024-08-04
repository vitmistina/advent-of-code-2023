use super::Graph;

impl Graph {
    pub fn from_input(input: &str) -> Self {
        let mut graph = Graph {
            edges: Default::default(),
        };

        for line in input.lines() {
            let mut parts = line.split(": ");
            let node = parts.next().unwrap().trim();
            let neighbors = parts.next().unwrap_or_default();

            for neighbor in neighbors.split(' ').map(str::trim) {
                if graph
                    .edges
                    .get(&(neighbor.to_string(), node.to_string()))
                    .is_some()
                {
                    continue;
                }
                graph.edges.insert((node.to_string(), neighbor.to_string()));
            }
        }

        graph
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_input_with_duplicate_edges() {
        // This input has particular two nodes, jqt and xhk, with an edge between them.
        // The edge is bidirectional, so there should be only one entry in the edges set.
        let input = "jqt: rhn xhk nvd
        xhk: jqt kqr\n";
        let graph = Graph::from_input(input);
        assert_eq!(graph.edges.len(), 4);
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
    }
}
