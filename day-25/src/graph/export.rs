use std::{fs::File, io::Write};

use super::Graph;

impl Graph {
    fn write_node(id: &str) -> String {
        format!(
            "<node id=\"{}\">
            <data key=\"d3\"><![CDATA[{}]]></data>
            <data key=\"d4\">
				<x:List>
					<y:Label>
						<y:Label.Text>{}</y:Label.Text>
						<y:Label.LayoutParameter>
							<y:CompositeLabelModelParameter>
								<y:CompositeLabelModelParameter.Parameter>
									<y:InteriorLabelModelParameter Position=\"Center\" Model=\"{{y:GraphMLReference 2}}\"/>
								</y:CompositeLabelModelParameter.Parameter>
								<y:CompositeLabelModelParameter.Model>
									<y:CompositeLabelModel>
										<y:CompositeLabelModel.LabelModels>
											<y:ExteriorLabelModel Insets=\"5\"/>
											<y:GraphMLReference ResourceKey=\"2\"/>
											<x:Static Member=\"y:FreeNodeLabelModel.Instance\"/>
										</y:CompositeLabelModel.LabelModels>
									</y:CompositeLabelModel>
								</y:CompositeLabelModelParameter.Model>
							</y:CompositeLabelModelParameter>
						</y:Label.LayoutParameter>
						<y:Label.Style>
							<yjs:DefaultLabelStyle verticalTextAlignment=\"CENTER\" horizontalTextAlignment=\"CENTER\" textFill=\"BLACK\">
								<yjs:DefaultLabelStyle.font>
									<yjs:Font fontSize=\"12\"/>
								</yjs:DefaultLabelStyle.font>
							</yjs:DefaultLabelStyle>
						</y:Label.Style>
					</y:Label>
				</x:List>
			</data>
            </node>\n",
            id, id, id
        )
    }

    pub fn export_to_graph_ml(&self) {
        let path = "graph.graphml";
        let mut file = File::create(path).unwrap();
        file.write_all(b"<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n")
            .unwrap();
        file.write_all(b"<graphml xsi:schemaLocation=\"http://graphml.graphdrawing.org/xmlns http://www.yworks.com/xml/schema/graphml.html/2.0/ygraphml.xsd \" xmlns=\"http://graphml.graphdrawing.org/xmlns\" xmlns:demostyle2=\"http://www.yworks.com/yFilesHTML/demos/FlatDemoStyle/2.0\" xmlns:demostyle=\"http://www.yworks.com/yFilesHTML/demos/FlatDemoStyle/1.0\" xmlns:icon-style=\"http://www.yworks.com/yed-live/icon-style/1.0\" xmlns:bpmn=\"http://www.yworks.com/xml/yfiles-bpmn/2.0\" xmlns:demotablestyle=\"http://www.yworks.com/yFilesHTML/demos/FlatDemoTableStyle/1.0\" xmlns:uml=\"http://www.yworks.com/yFilesHTML/demos/UMLDemoStyle/1.0\" xmlns:GraphvizNodeStyle=\"http://www.yworks.com/yFilesHTML/graphviz-node-style/1.0\" xmlns:Vue2jsNodeStyle=\"http://www.yworks.com/demos/yfiles-vuejs-node-style/1.0\" xmlns:Vue3jsNodeStyle=\"http://www.yworks.com/demos/yfiles-vue-node-style/3.0\" xmlns:explorer-style=\"http://www.yworks.com/data-explorer/1.0\" xmlns:y=\"http://www.yworks.com/xml/yfiles-common/3.0\" xmlns:x=\"http://www.yworks.com/xml/yfiles-common/markup/3.0\" xmlns:yjs=\"http://www.yworks.com/xml/yfiles-for-html/2.0/xaml\" xmlns:xsi=\"http://www.w3.org/2001/XMLSchema-instance\">\n").unwrap();
        file.write_all(b"<graph id=\"G\" edgedefault=\"undirected\">\n")
            .unwrap();

        let mut nodes = Vec::new();
        for edge in &self.edges {
            let node1 = &edge.nodes[0];
            let node2 = &edge.nodes[1];
            if !nodes.contains(node1) {
                nodes.push(node1.clone());
                file.write_all(Self::write_node(node1).as_bytes()).unwrap();
            }
            if !nodes.contains(node2) {
                nodes.push(node2.clone());
                file.write_all(Self::write_node(node2).as_bytes()).unwrap();
            }
        }

        for edge in &self.edges {
            let node1 = &edge.nodes[0];
            let node2 = &edge.nodes[1];
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
