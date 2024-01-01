use super::*;

impl Stack {
    pub(super) fn parse(input: &str) -> Self {
        Self {
            terrain: vec![vec![0; 10]; 10],
            snapshot: input
                .lines()
                .enumerate() // Provides index (for id) and line
                .map(|(index, line)| Brick::from(line, index + 1)) // Convert index to 1-based id
                .collect(),
            landed: vec![],
            relations: HashMap::new(),
        }
    }
}

impl Brick {
    fn from(line: &str, id: usize) -> Self {
        let parts: Vec<&str> = line.split('~').collect();
        let min_coords: Vec<usize> = parts[0].split(',').map(|s| s.parse().unwrap()).collect();
        let max_coords: Vec<usize> = parts[1].split(',').map(|s| s.parse().unwrap()).collect();

        Brick {
            min: Coordinate {
                x: min_coords[0],
                y: min_coords[1],
                z: min_coords[2],
            },
            max: Coordinate {
                x: max_coords[0],
                y: max_coords[1],
                z: max_coords[2],
            },
            id,
        }
    }
}

#[test]
fn parses_bricks() {
    let input = "1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9";
    let stack = Stack::parse(input);
    assert_eq!(stack.terrain.len(), 10);
    assert!(stack
        .terrain
        .iter()
        .all(|row| row.iter().all(|&elem| elem == 0)));
    assert_eq!(
        stack.snapshot,
        Vec::from([
            Brick {
                min: Coordinate { x: 1, y: 0, z: 1 },
                max: Coordinate { x: 1, y: 2, z: 1 },
                id: 1
            },
            Brick {
                min: Coordinate { x: 0, y: 0, z: 2 },
                max: Coordinate { x: 2, y: 0, z: 2 },
                id: 2
            },
            Brick {
                min: Coordinate { x: 0, y: 2, z: 3 },
                max: Coordinate { x: 2, y: 2, z: 3 },
                id: 3
            },
            Brick {
                min: Coordinate { x: 0, y: 0, z: 4 },
                max: Coordinate { x: 0, y: 2, z: 4 },
                id: 4
            },
            Brick {
                min: Coordinate { x: 2, y: 0, z: 5 },
                max: Coordinate { x: 2, y: 2, z: 5 },
                id: 5
            },
            Brick {
                min: Coordinate { x: 0, y: 1, z: 6 },
                max: Coordinate { x: 2, y: 1, z: 6 },
                id: 6
            },
            Brick {
                min: Coordinate { x: 1, y: 1, z: 8 },
                max: Coordinate { x: 1, y: 1, z: 9 },
                id: 7
            },
        ])
    );
}
