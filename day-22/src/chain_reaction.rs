use super::*;

impl Stack {
    pub(super) fn get_chain_lengths(&self) -> Vec<usize> {
        let mut memory: HashMap<usize, usize> = HashMap::new();
        self.relations
            .keys()
            .map(|&id| self.compute_chain_length(id))
            .collect()
    }

    fn compute_chain_length(&self, id: usize) -> usize {
        let mut relations = self.relations.clone();
        let mut buffer = vec![id];
        let mut chain_count = 0;

        while let Some(current_id) = buffer.pop() {
            let supported_ids = if let Some(current_relation) = relations.get(&current_id) {
                current_relation.supports.clone()
            } else {
                panic!()
            };

            for &supported_id in &supported_ids {
                if let Some(supported_relation) = relations.get_mut(&supported_id) {
                    supported_relation.stands_on.retain(|&x| x != current_id);

                    if supported_relation.stands_on.is_empty() {
                        buffer.push(supported_id);
                        chain_count += 1;
                    }
                }
            }
        }

        chain_count
    }
}

#[test]
fn computes_chained_relations() {
    let stack = Stack {
        terrain: vec![],
        snapshot: vec![],
        landed: vec![],
        relations: HashMap::from([
            // 1 supports 3, 5 and 7. 3 is supported by other relation, but 5 and 7 stands just on 1.
            // Furthermore, 5 takes down itself and 6.
            // expect 3
            (
                1,
                Relation {
                    supports: vec![3, 5, 7],
                    stands_on: vec![],
                },
            ),
            // 2 doesn't support other, expect 0
            (
                2,
                Relation {
                    supports: vec![],
                    stands_on: vec![3],
                },
            ),
            // 3 supports 2, and 2 stands only on 3, Chains to 2, which takes down itself. expect 1
            (
                3,
                Relation {
                    supports: vec![2],
                    stands_on: vec![1, 4],
                },
            ),
            // 4 doesn't support other, expect 0
            (
                4,
                Relation {
                    supports: vec![3],
                    stands_on: vec![],
                },
            ),
            // 5 supports 6, 6 stands just on 5. 6 takes down itself. expect 1
            (
                5,
                Relation {
                    supports: vec![6],
                    stands_on: vec![1],
                },
            ),
            // 6 doesn't support other, expect 0
            (
                6,
                Relation {
                    supports: vec![],
                    stands_on: vec![5],
                },
            ),
            // 7 doesn't support other, expect 0
            (
                7,
                Relation {
                    supports: vec![],
                    stands_on: vec![1],
                },
            ),
        ]),
    };

    let result: usize = stack.compute_chain_length(1);
    assert_eq!(result, 3);

    let result = stack.compute_chain_length(2);
    assert_eq!(result, 0);

    let result = stack.compute_chain_length(3);
    assert_eq!(result, 1);

    let result = stack.compute_chain_length(4);
    assert_eq!(result, 0);

    let result = stack.compute_chain_length(5);
    assert_eq!(result, 1);

    let result = stack.compute_chain_length(6);
    assert_eq!(result, 0);

    let result = stack.compute_chain_length(7);
    assert_eq!(result, 0);
}

#[test]
fn computes_all() {
    let stack = Stack {
        terrain: vec![],
        snapshot: vec![],
        landed: vec![],
        relations: HashMap::from([
            // 1 supports 3, 5 and 7. 3 is supported by other relation, but 5 and 7 stands just on 1.
            // Furthermore, 5 takes down itself and 6.
            // expect 3
            (
                1,
                Relation {
                    supports: vec![3, 5, 7],
                    stands_on: vec![],
                },
            ),
            // 2 doesn't support other, expect 0
            (
                2,
                Relation {
                    supports: vec![],
                    stands_on: vec![3],
                },
            ),
            // 3 supports 2, and 2 stands only on 3, Chains to 2, which takes down itself. expect 1
            (
                3,
                Relation {
                    supports: vec![2],
                    stands_on: vec![1, 4],
                },
            ),
            // 4 doesn't support other, expect 0
            (
                4,
                Relation {
                    supports: vec![3],
                    stands_on: vec![],
                },
            ),
            // 5 supports 6, 6 stands just on 5. 6 takes down itself. expect 1
            (
                5,
                Relation {
                    supports: vec![6],
                    stands_on: vec![1],
                },
            ),
            // 6 doesn't support other, expect 0
            (
                6,
                Relation {
                    supports: vec![],
                    stands_on: vec![5],
                },
            ),
            // 7 doesn't support other, expect 0
            (
                7,
                Relation {
                    supports: vec![],
                    stands_on: vec![1],
                },
            ),
        ]),
    };

    assert_eq!(
        stack.get_chain_lengths().sort(),
        [3, 0, 1, 0, 1, 0, 0].sort()
    );
}
