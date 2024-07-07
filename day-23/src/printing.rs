use std::fs::File;
use std::io::Write;
use std::path::Path;

use super::*;

impl Maze {
    pub fn save_to_graphml(&self, file: &str) -> std::io::Result<()> {
        let file_path = Path::new(file);
        let mut file = File::create(file_path)?;

        // Write the header
        writeln!(
            file,
            "<graphml xmlns=\"http://graphml.graphdrawing.org/xmlns\" xmlns:y=\"http://www.yworks.com/xml/graphml\">"
        )?;
        // Define key for edge labels
        writeln!(
            file,
            "  <key id=\"d10\" for=\"edge\" yfiles.type=\"edgegraphics\"/>"
        )?;
        writeln!(file, "  <graph id=\"G\" edgedefault=\"undirected\">")?;

        // Write the nodes
        for coordinate in self.nodes.keys() {
            writeln!(file, "    <node id=\"{:?}\"/>", coordinate)?;
        }

        // Write the edges with labels
        for edge in &self.edges {
            writeln!(
                file,
                "    <edge source=\"{:?}\" target=\"{:?}\">",
                edge.starting_node_loc, edge.ending_node_loc
            )?;
            // Add edge length as label in the specified format
            write!(
                file,
                "      <data key=\"d10\">\n\
                 <y:PolyLineEdge>\n\
                 <y:EdgeLabel alignment=\"center\" backgroundColor=\"#C0C0C0\" configuration=\"AutoFlippingLabel\" distance=\"2.0\" fontFamily=\"Dialog\" fontSize=\"12\" fontStyle=\"plain\" hasLineColor=\"false\" horizontalTextPosition=\"center\" iconTextGap=\"4\" modelName=\"centered\" modelPosition=\"center\" preferredPlacement=\"center_on_edge\" ratio=\"0.5\" textColor=\"#000000\" verticalTextPosition=\"bottom\" visible=\"true\" xml:space=\"preserve\" >{}\n\
                 <y:PreferredPlacementDescriptor angle=\"0.0\" angleOffsetOnRightSide=\"0\" angleReference=\"absolute\" angleRotationOnRightSide=\"co\" distance=\"-1.0\" placement=\"center\" side=\"on_edge\" sideReference=\"relative_to_edge_flow\"/>\n\
                 </y:EdgeLabel>\n\
                 </y:PolyLineEdge>\n\
                 </data>\n",
                edge.length
            )?;
            writeln!(file, "    </edge>")?;
        }

        // Close the tags
        write!(file, "  </graph>\n</graphml>")?;

        Ok(())
    }
}
