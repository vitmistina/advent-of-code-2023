use super::*;

impl Stack {
    pub(super) fn sort_bricks(&mut self) {
        self.snapshot.sort_by(|a, b| b.min.z.cmp(&a.min.z));
        self.landed.sort_by(|a, b| b.min.z.cmp(&a.min.z));
    }
}

#[test]
fn orders_bricks() {
    let unordered = Vec::from([
        Brick {
            min: Coordinate { x: 1, y: 1, z: 8 },
            max: Coordinate { x: 1, y: 1, z: 9 },
            id: 7,
        },
        Brick {
            min: Coordinate { x: 0, y: 0, z: 2 },
            max: Coordinate { x: 2, y: 0, z: 2 },
            id: 2,
        },
        Brick {
            min: Coordinate { x: 0, y: 0, z: 4 },
            max: Coordinate { x: 0, y: 2, z: 4 },
            id: 4,
        },
        Brick {
            min: Coordinate { x: 1, y: 0, z: 1 },
            max: Coordinate { x: 1, y: 2, z: 1 },
            id: 1,
        },
        Brick {
            min: Coordinate { x: 2, y: 0, z: 5 },
            max: Coordinate { x: 2, y: 2, z: 5 },
            id: 5,
        },
        Brick {
            min: Coordinate { x: 0, y: 2, z: 3 },
            max: Coordinate { x: 2, y: 2, z: 3 },
            id: 3,
        },
        Brick {
            min: Coordinate { x: 0, y: 1, z: 6 },
            max: Coordinate { x: 2, y: 1, z: 6 },
            id: 6,
        },
    ]);

    let mut stack = Stack {
        terrain: vec![],
        snapshot: unordered,
        landed: vec![],
        relations: HashMap::new(),
    };

    stack.sort_bricks();

    assert_eq!(
        stack.snapshot,
        Vec::from([
            Brick {
                min: Coordinate { x: 1, y: 1, z: 8 },
                max: Coordinate { x: 1, y: 1, z: 9 },
                id: 7
            },
            Brick {
                min: Coordinate { x: 0, y: 1, z: 6 },
                max: Coordinate { x: 2, y: 1, z: 6 },
                id: 6
            },
            Brick {
                min: Coordinate { x: 2, y: 0, z: 5 },
                max: Coordinate { x: 2, y: 2, z: 5 },
                id: 5
            },
            Brick {
                min: Coordinate { x: 0, y: 0, z: 4 },
                max: Coordinate { x: 0, y: 2, z: 4 },
                id: 4
            },
            Brick {
                min: Coordinate { x: 0, y: 2, z: 3 },
                max: Coordinate { x: 2, y: 2, z: 3 },
                id: 3
            },
            Brick {
                min: Coordinate { x: 0, y: 0, z: 2 },
                max: Coordinate { x: 2, y: 0, z: 2 },
                id: 2
            },
            Brick {
                min: Coordinate { x: 1, y: 0, z: 1 },
                max: Coordinate { x: 1, y: 2, z: 1 },
                id: 1
            },
        ])
    );
}
