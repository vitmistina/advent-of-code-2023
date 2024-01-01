use super::*;

impl Stack {
    pub(super) fn fall_next_brick(&mut self) -> Result<(), String> {
        if let Some(mut brick) = self.snapshot.pop() {
            let mut highest_point = 0;
            for x in brick.min.x..=brick.max.x {
                for y in brick.min.y..=brick.max.y {
                    let terrain_height = self
                        .terrain
                        .get(y)
                        .and_then(|column| column.get(x))
                        .cloned()
                        .unwrap_or(0);
                    highest_point = highest_point.max(terrain_height);
                }
            }

            let brick_height = brick.max.z - brick.min.z;

            brick.min.z = 1 + highest_point;
            brick.max.z = 1 + highest_point + brick_height;

            self.update_terrain(&brick);
            self.landed.push(brick.clone());
            Ok(())
        } else {
            Err("No more bricks to fall".to_string())
        }
    }

    fn update_terrain(&mut self, brick: &Brick) {
        for x in brick.min.x..=brick.max.x {
            for y in brick.min.y..=brick.max.y {
                self.terrain[y][x] = self.terrain[y][x].max(brick.max.z);
            }
        }
    }
}

#[test]
fn calculates_fallen_brick() {
    let mut stack = Stack {
        terrain: vec![vec![0, 1, 2]],
        snapshot: vec![
            Brick {
                min: Coordinate { x: 0, y: 0, z: 8 },
                max: Coordinate { x: 0, y: 0, z: 9 },
                id: 2,
            },
            Brick {
                min: Coordinate { x: 1, y: 0, z: 5 },
                max: Coordinate { x: 2, y: 0, z: 5 },
                id: 1,
            },
        ],
        landed: vec![],
        relations: HashMap::new(),
    };
    let original_snapshot_len = stack.snapshot.len();

    assert_eq!(stack.fall_next_brick(), Ok(()));
    assert_eq!(stack.landed.len(), 1);
    assert_eq!(
        stack.landed[0],
        Brick {
            min: Coordinate { x: 1, y: 0, z: 3 },
            max: Coordinate { x: 2, y: 0, z: 3 },
            id: 1,
        }
    );
    assert_eq!(stack.snapshot.len(), original_snapshot_len - 1);

    assert_eq!(stack.fall_next_brick(), Ok(()));
    assert_eq!(stack.landed.len(), 2);
    assert_eq!(
        stack.landed[1],
        Brick {
            min: Coordinate { x: 0, y: 0, z: 1 },
            max: Coordinate { x: 0, y: 0, z: 2 },
            id: 2,
        }
    );
    assert_eq!(stack.snapshot.len(), original_snapshot_len - 2);
    assert_eq!(stack.snapshot.len(), 0);

    let one_too_long = stack.fall_next_brick();

    assert!(one_too_long.is_err());
}

#[test]
fn updates_terrain() {
    let mut stack = Stack {
        terrain: vec![vec![0, 1, 2]],
        snapshot: vec![],
        landed: vec![],
        relations: HashMap::new(),
    };

    let brick = Brick {
        min: Coordinate { x: 1, y: 0, z: 3 },
        max: Coordinate { x: 2, y: 0, z: 3 },
        id: 1,
    };

    stack.update_terrain(&brick);

    assert_eq!(stack.terrain, vec![vec![0, 3, 3]]);

    let second_brick = Brick {
        min: Coordinate { x: 0, y: 0, z: 1 },
        max: Coordinate { x: 0, y: 0, z: 2 },
        id: 1,
    };

    stack.update_terrain(&second_brick);

    assert_eq!(stack.terrain, vec![vec![2, 3, 3]]);
}
