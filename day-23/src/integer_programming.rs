use super::*;

fn create_connection_matrix(edges: &[Edge]) -> Vec<Vec<usize>> {
    // Step 1: Determine Matrix Size
    let max_index = edges.iter().fold(0, |max, edge| {
        max.max(edge.starting_node.x.max(edge.ending_node.x))
    }) + 1; // +1 because nodes are zero-indexed

    // Step 2: Initialize Matrix
    let mut matrix = vec![vec![0; max_index]; max_index];

    // Step 3: Set Connections
    for edge in edges {
        matrix[edge.starting_node.x][edge.ending_node.x] = 1;
        matrix[edge.ending_node.x][edge.starting_node.x] = 1; // Because the graph is undirected
    }

    matrix
}

fn create_distance_matrix(edges: &[Edge]) -> Vec<Vec<usize>> {
    // Step 1: Determine Matrix Size
    let max_index = edges.iter().fold(0, |max, edge| {
        max.max(edge.starting_node.x.max(edge.ending_node.x))
    }) + 1; // +1 because nodes are zero-indexed

    // Step 2: Initialize Matrix
    let mut matrix = vec![vec![0; max_index]; max_index];

    // Step 3: Set Connections
    for edge in edges {
        matrix[edge.starting_node.x][edge.ending_node.x] = edge.length;
        matrix[edge.ending_node.x][edge.starting_node.x] = edge.length; // Because the graph is undirected
    }

    matrix
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix_of_connections() {
        let edges = vec![
            Edge {
                starting_node: Coordinate { x: 0, y: 0 },
                ending_node: Coordinate { x: 1, y: 1 },
                length: 50,
            },
            Edge {
                starting_node: Coordinate { x: 0, y: 0 },
                ending_node: Coordinate { x: 2, y: 2 },
                length: 60,
            },
        ];

        let expected_matrix = vec![vec![0, 1, 1], vec![1, 0, 0], vec![1, 0, 0]];

        let actual_matrix = create_connection_matrix(&edges);

        assert_eq!(
            actual_matrix, expected_matrix,
            "The generated matrix does not match the expected matrix."
        );
    }

    #[test]
    fn test_matrix_of_distances() {
        let edges = vec![
            Edge {
                starting_node: Coordinate { x: 0, y: 0 },
                ending_node: Coordinate { x: 1, y: 1 },
                length: 5,
            },
            Edge {
                starting_node: Coordinate { x: 0, y: 0 },
                ending_node: Coordinate { x: 2, y: 2 },
                length: 9,
            },
        ];

        let expected_matrix = vec![vec![0, 5, 9], vec![5, 0, 0], vec![9, 0, 0]];

        let actual_matrix = create_distance_matrix(&edges);

        assert_eq!(
            actual_matrix, expected_matrix,
            "The generated matrix does not match the expected matrix."
        );
    }
}
