use super::*;

impl Stack {
    pub(super) fn count_safe_bricks(&self) -> usize {
        self.relations
            .iter()
            .filter(|&(_id, current)| self.is_safe_to_remove(current))
            .count()
    }
}

#[test]
fn counts_safe_bricks() {
    let stack = Stack {
        terrain: vec![],
        snapshot: vec![],
        landed: vec![],
        relations: HashMap::from([
            // 1 supports two bricks, but 5 stands just on 1. NOT SAFE
            (
                1,
                Relation {
                    supports: vec![3, 5],
                    stands_on: vec![],
                },
            ),
            // 2 doesn't support any bricks, SAFE
            (
                2,
                Relation {
                    supports: vec![],
                    stands_on: vec![3],
                },
            ),
            // 3 supports 2, and 2 stands only on 3, NOT SAFE
            (
                3,
                Relation {
                    supports: vec![2],
                    stands_on: vec![1, 4],
                },
            ),
            // 4 supports 3, but 3 is also supported by 1, SAFE
            (
                4,
                Relation {
                    supports: vec![3],
                    stands_on: vec![],
                },
            ),
            // 5 doesn't support any bricks, SAFE
            (
                5,
                Relation {
                    supports: vec![],
                    stands_on: vec![1],
                },
            ),
        ]),
    };

    let result: usize = stack.count_safe_bricks();
    assert_eq!(result, 3);
}
