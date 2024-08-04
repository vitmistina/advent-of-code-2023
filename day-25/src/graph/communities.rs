use std::collections::HashSet;

use super::Graph;

impl Graph {
    pub fn find_communities(&self) -> Vec<HashSet<String>> {
        let mut communities = Vec::new();
        let mut visited = HashSet::new();

        for (node1, node2) in &self.edges {
            if !visited.contains(node1) {
                let community = self.depth_first_search(node1);
                visited.extend(community.iter().cloned());
                communities.push(community);
            }
            if !visited.contains(node2) {
                let community = self.depth_first_search(node2);
                visited.extend(community.iter().cloned());
                communities.push(community);
            }
        }

        communities
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

    fn neighbors(&self, node: &String) -> Vec<String> {
        self.edges
            .iter()
            .filter_map(|(n1, n2)| {
                if n1 == node {
                    Some(n2.clone())
                } else if n2 == node {
                    Some(n1.clone())
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
}
