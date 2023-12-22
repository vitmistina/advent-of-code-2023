use super::*;

impl Grid {
    pub fn count(&self) -> usize {
        self.data
            .iter()
            .map(|row| row.iter().filter(|node| node.is_dug).count())
            .sum()
    }
}

#[test]
fn counts_nodes() {
    let grid = Grid {
        data: vec![
            vec![Node { is_dug: true }, Node { is_dug: false }],
            vec![Node { is_dug: true }, Node { is_dug: true }],
        ],
        current_coord: Coordinate { x: 0, y: 0 },
    };

    assert_eq!(grid.count(), 3);
}

impl Vertices {
    pub fn count(&self) -> usize {
        let mut iter = self.data.iter().peekable();

        let mut sum: isize = 0;
        while let Some(vertex) = iter.next() {
            if let Some(next) = iter.peek() {
                sum += (next.y * vertex.x) as isize - (next.x * vertex.y) as isize;
            };
        }
        sum as usize / 2
    }
}

#[test]
fn count_polygon() {
    let vert = Vertices {
        data: vec![
            BigCoordinate { x: 0, y: 0 },
            BigCoordinate { x: 5, y: 0 },
            BigCoordinate { x: 5, y: 2 },
            BigCoordinate { x: 3, y: 2 },
            BigCoordinate { x: 3, y: 4 },
            BigCoordinate { x: 5, y: 4 },
            BigCoordinate { x: 5, y: 6 },
            BigCoordinate { x: 0, y: 6 },
        ],
        horizontal: Direction::Left,
        vertical: Direction::Up,
    };

    assert_eq!(vert.count(), 26);
}
