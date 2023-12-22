use super::*;

impl Grid {
    pub fn floodfill(&mut self) {
        let (x_len, y_len) = { (self.data[0].len(), self.data.len()) };
        let mut buffer = vec![Coordinate {
            x: x_len / 2 + 1,
            y: y_len / 2 + 1,
        }];
        while let Some(coord) = buffer.pop() {
            self.data[coord.y][coord.x].is_dug = true;

            for dir in vec![
                Direction::Up,
                Direction::Down,
                Direction::Left,
                Direction::Right,
            ] {
                let (new_x, new_y, x_len, y_len) = self.get_coords_for_direction(&dir, &coord);
                match (new_x, new_y) {
                    (Some(x), Some(y)) => {
                        if self.data[y][x].is_dug == false {
                            buffer.push(Coordinate { x, y });
                        };
                    }
                    _ => panic!("out of bounds"),
                }
            }
        }
    }
}

#[test]
fn flood_fills() {
    let mut grid = Grid::new();
    grid.data[500][500].is_dug = true;
    grid.data[500][501].is_dug = true;
    grid.data[500][502].is_dug = true;
    grid.data[500][503].is_dug = false;

    grid.data[501][500].is_dug = true;
    grid.data[501][501].is_dug = false;
    grid.data[501][502].is_dug = true;
    grid.data[501][503].is_dug = true;

    grid.data[502][500].is_dug = true;
    grid.data[502][501].is_dug = false;
    grid.data[502][502].is_dug = false;
    grid.data[502][503].is_dug = true;

    grid.data[503][500].is_dug = true;
    grid.data[503][501].is_dug = true;
    grid.data[503][502].is_dug = true;
    grid.data[503][503].is_dug = true;

    grid.floodfill();

    assert_eq!(grid.data[500][503].is_dug, false);
    assert_eq!(grid.data[500].iter().filter(|node| node.is_dug).count(), 3);
    assert_eq!(grid.data[501].iter().filter(|node| node.is_dug).count(), 4);
    assert_eq!(grid.data[502].iter().filter(|node| node.is_dug).count(), 4);
    assert_eq!(grid.data[503].iter().filter(|node| node.is_dug).count(), 4);
}
