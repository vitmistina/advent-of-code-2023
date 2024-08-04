use std::{fs::File, io::Write};

use super::Graph;

impl Graph {
    fn write_node(id: &str) -> String {
        format!(
            "<node id=\"{}\">\n<data>\n
            <y:Label>\n<y:Label.Text>{}</y:Label.Text></y:Label>\n
            </data></node>\n",
            id, id
        )
    }

    pub fn export_to_graph_ml(&self) {
        let path = "graph.graphml";
        let mut file = File::create(path).unwrap();
        file.write_all(b"<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n")
            .unwrap();
        file.write_all(b"<graphml xmlns=\"http://graphml.graphdrawing.org/xmlns\" xmlns:xsi=\"http://www.w3.org/2001/XMLSchema-instance\" xsi:schemaLocation=\"http://graphml.graphdrawing.org/xmlns http://graphml.graphdrawing.org/xmlns/1.0/graphml.xsd\">\n").unwrap();
        file.write_all(b"<graph id=\"G\" edgedefault=\"undirected\">\n")
            .unwrap();

        let mut nodes = Vec::new();
        for (node1, node2) in &self.edges {
            if !nodes.contains(node1) {
                nodes.push(node1.clone());
                file.write_all(Self::write_node(node1).as_bytes()).unwrap();
            }
            if !nodes.contains(node2) {
                nodes.push(node2.clone());
                file.write_all(Self::write_node(node2).as_bytes()).unwrap();
            }
        }

        for (node1, node2) in &self.edges {
            file.write_all(
                format!("<edge source=\"{}\" target=\"{}\"/>\n", node1, node2).as_bytes(),
            )
            .unwrap();
        }

        file.write_all(b"</graph>\n").unwrap();

        file.write_all(b"</graphml>\n").unwrap();

        println!("Graph exported to {}", path);
    }
}
