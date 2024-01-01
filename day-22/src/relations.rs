use super::*;

impl Stack {
    fn find_supported_bricks(&self, brick: &Brick) -> Vec<usize> {
        self.landed
            .iter()
            .filter(|&landed_brick| landed_brick.min.z == brick.max.z + 1)
            .filter_map(|landed_brick| {
                let x_overlap =
                    landed_brick.min.x <= brick.max.x && landed_brick.max.x >= brick.min.x;
                let y_overlap =
                    landed_brick.min.y <= brick.max.y && landed_brick.max.y >= brick.min.y;
                if x_overlap && y_overlap {
                    Some(landed_brick.id)
                } else {
                    None
                }
            })
            .collect()
    }

    pub(super) fn find_relations(&mut self) {
        self.relations = self
            .landed
            .iter()
            .map(|brick| {
                (
                    brick.id,
                    Relation {
                        supports: Vec::new(),
                        stands_on: Vec::new(),
                    },
                )
            })
            .collect();

        for brick in &self.landed {
            let supported_bricks = self.find_supported_bricks(brick);

            if let Some(relation) = self.relations.get_mut(&brick.id) {
                relation.supports = supported_bricks.clone();
            }

            for supported_id in supported_bricks {
                if let Some(relation) = self.relations.get_mut(&supported_id) {
                    relation.stands_on.push(brick.id);
                }
            }
        }
    }

    pub(crate) fn is_safe_to_remove(&self, relation: &Relation) -> bool {
        if relation.supports.is_empty() {
            return true;
        }

        relation.supports.iter().all(|supported_id| {
            self.relations
                .get(supported_id)
                .map_or(false, |supported_by| supported_by.stands_on.len() > 1)
        })
    }
}

#[test]
fn check_floor_above() {
    let mut stack = Stack {
        terrain: vec![vec![]],
        snapshot: vec![],
        landed: vec![
            Brick {
                min: Coordinate { x: 1, y: 0, z: 1 },
                max: Coordinate { x: 2, y: 0, z: 1 },
                id: 1,
            },
            Brick {
                min: Coordinate { x: 1, y: 0, z: 2 },
                max: Coordinate { x: 1, y: 2, z: 2 },
                id: 3,
            },
            Brick {
                min: Coordinate { x: 1, y: 0, z: 3 },
                max: Coordinate { x: 2, y: 0, z: 3 },
                id: 2,
            },
            Brick {
                min: Coordinate { x: 0, y: 0, z: 2 },
                max: Coordinate { x: 0, y: 2, z: 2 },
                id: 4,
            },
            Brick {
                min: Coordinate { x: 2, y: 0, z: 2 },
                max: Coordinate { x: 2, y: 0, z: 10 },
                id: 5,
            },
        ],
        relations: HashMap::new(),
    };

    let found_ids: Vec<usize> = stack.find_supported_bricks(&Brick {
        min: Coordinate { x: 1, y: 0, z: 1 },
        max: Coordinate { x: 2, y: 0, z: 1 },
        id: 1,
    });

    assert_eq!(found_ids, vec![3, 5]);
}

#[test]
fn creates_support_relations() {
    let mut stack = Stack {
        terrain: vec![vec![]],
        snapshot: vec![],
        landed: vec![
            Brick {
                min: Coordinate { x: 1, y: 0, z: 1 },
                max: Coordinate { x: 2, y: 0, z: 1 },
                id: 1,
            },
            Brick {
                min: Coordinate { x: 1, y: 0, z: 2 },
                max: Coordinate { x: 1, y: 2, z: 2 },
                id: 3,
            },
            Brick {
                min: Coordinate { x: 1, y: 0, z: 3 },
                max: Coordinate { x: 1, y: 0, z: 3 },
                id: 2,
            },
            Brick {
                min: Coordinate { x: 0, y: 0, z: 2 },
                max: Coordinate { x: 0, y: 2, z: 2 },
                id: 4,
            },
            Brick {
                min: Coordinate { x: 2, y: 0, z: 2 },
                max: Coordinate { x: 2, y: 0, z: 10 },
                id: 5,
            },
        ],
        relations: HashMap::new(),
    };

    stack.find_relations();

    assert_eq!(
        stack.relations,
        HashMap::from([
            (
                1,
                Relation {
                    supports: vec![3, 5],
                    stands_on: vec![]
                }
            ),
            (
                2,
                Relation {
                    supports: vec![],
                    stands_on: vec![3]
                }
            ),
            (
                3,
                Relation {
                    supports: vec![2],
                    stands_on: vec![1]
                }
            ),
            (
                4,
                Relation {
                    supports: vec![],
                    stands_on: vec![]
                }
            ),
            (
                5,
                Relation {
                    supports: vec![],
                    stands_on: vec![1]
                }
            ),
        ])
    );
}
