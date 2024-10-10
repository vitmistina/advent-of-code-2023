use std::collections::HashSet;

use super::Graph;

impl Graph {
    fn find_communities(&self) -> Vec<HashSet<String>> {
        let mut communities = Vec::new();
        let mut visited = HashSet::new();

        for node in self.nodes.iter() {
            if !visited.contains(&node.id) {
                let community = self.depth_first_search(&node.id);
                visited.extend(community.iter().cloned());
                communities.push(community);
            }
        }
        communities
    }

    fn multiply(communities: &[HashSet<String>]) -> usize {
        communities.iter().map(|c| c.len()).product()
    }

    pub fn find_two_community_product(&self) -> Option<usize> {
        let communities = self.find_communities();
        if communities.len() != 2 {
            return None;
        }
        Some(Self::multiply(&communities))
    }

    fn depth_first_search(&self, start: &String) -> HashSet<String> {
        let mut stack = Vec::new();
        let mut visited = HashSet::new();

        stack.push(start.clone());
        visited.insert(start.clone());

        while let Some(node) = stack.pop() {
            for neighbor in self.neighbors(&node) {
                if !visited.contains(&neighbor) {
                    visited.insert(neighbor.clone());
                    stack.push(neighbor);
                }
            }
        }

        visited
    }

    pub fn neighbors(&self, node: &String) -> Vec<String> {
        self.edges
            .iter()
            .filter_map(|edge| {
                if edge.nodes.contains(node) {
                    Some(edge.nodes.iter().find(|n| n != &node).unwrap().clone())
                } else {
                    None
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_one_community() {
        let input = "jqt: rhn xhk nvd";
        let graph = Graph::from_input(input);
        let communities = graph.find_communities();
        assert_eq!(communities.len(), 1);
    }

    #[test]
    fn finds_two_communities() {
        let input = "jqt: rhn xhk nvd
        jjj: aaa bbb ccc";
        let graph = Graph::from_input(input);
        let communities = graph.find_communities();
        assert_eq!(communities.len(), 2);
    }

    #[test]
    fn multiplies_community_sizes() {
        let input = "jqt: rhn xhk nvd
        jjj: aaa bbb ccc";
        let graph = Graph::from_input(input);
        let product = graph.find_two_community_product().unwrap();
        assert_eq!(product, 16);
    }
}
