use super::*;

impl Grid {
    pub fn find_lowest_unvisited(&self) -> Option<Coordinate> {
        let mut min = u64::MAX;
        let mut min_coord = None;

        for (y, row) in self.data.iter().enumerate() {
            for (x, node) in row.iter().enumerate() {
                if node.is_visited == false {
                    if let Some(score) = node.current_score {
                        if score < min {
                            min = score;
                            min_coord = Some(Coordinate { x, y });
                        }
                    }
                }
            }
        }
        if min == u64::MAX {
            panic!();
        }
        min_coord
    }
}

#[test]
fn finds_lowest_current_score() {
    let grid = Grid {
        data: vec![vec![
            Node {
                current_score: Some(0),
                is_visited: false,
                heat_loss: 1,
                is_target: false,
                prev_directions: vec![],
            },
            Node {
                current_score: None,
                is_visited: false,
                heat_loss: 2,
                is_target: true,
                prev_directions: vec![],
            },
        ]],
    };

    assert_eq!(
        grid.find_lowest_unvisited(),
        Some(Coordinate { x: 0, y: 0 })
    );

    let grid = Grid {
        data: vec![
            vec![
                Node {
                    current_score: Some(0),
                    is_visited: true,
                    heat_loss: 1,
                    is_target: false,
                    prev_directions: vec![],
                },
                Node {
                    current_score: Some(5),
                    is_visited: false,
                    heat_loss: 5,
                    is_target: false,
                    prev_directions: vec![],
                },
                Node {
                    current_score: None,
                    is_visited: false,
                    heat_loss: 2,
                    is_target: false,
                    prev_directions: vec![],
                },
            ],
            vec![
                Node {
                    current_score: Some(4),
                    is_visited: false,
                    heat_loss: 4,
                    is_target: false,
                    prev_directions: vec![],
                },
                Node {
                    current_score: None,
                    is_visited: false,
                    heat_loss: 5,
                    is_target: false,
                    prev_directions: vec![],
                },
                Node {
                    current_score: None,
                    is_visited: false,
                    heat_loss: 2,
                    is_target: true,
                    prev_directions: vec![],
                },
            ],
        ],
    };

    assert_eq!(
        grid.find_lowest_unvisited(),
        Some(Coordinate { x: 0, y: 1 })
    );
}
