use std::collections::HashSet;

mod communities;
mod export;
mod parsing;
mod removal;

pub(super) struct Graph {
    edges: HashSet<(String, String)>,
}

impl Graph {
    pub fn export(input: &str) {
        let graph = Graph::from_input(input);
        graph.export_to_graph_ml();
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
}
