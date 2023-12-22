use super::*;

impl Grid {
    pub fn execute(&mut self, command: &Command) {
        for _ in 0..command.len {
            let (new_x, new_y, x_len, y_len) =
                self.get_coords_for_direction(&command.dir, &self.current_coord);

            match (new_x, new_y) {
                (Some(x), Some(y)) => {
                    self.current_coord.x = x;
                    self.current_coord.y = y;
                    self.data[y][x].is_dug = true;
                }
                _ => panic!("out of bounds"),
            }
        }
    }

    pub fn get_coords_for_direction(
        &self,
        direction: &Direction,
        coord: &Coordinate,
    ) -> (Option<usize>, Option<usize>, usize, usize) {
        let (y_offset, x_offset) = match direction {
            Direction::Down => (1, 0),
            Direction::Up => (-1, 0),
            Direction::Right => (0, 1),
            Direction::Left => (0, -1),
            _ => panic!(),
        };
        let (x_len, y_len) = { (self.data[0].len(), self.data.len()) };
        let new_x = get_coords(coord.x, x_offset, x_len);
        let new_y = get_coords(coord.y, y_offset, y_len);
        (new_x, new_y, x_len, y_len)
    }
}

fn get_coords(index: usize, offset: i32, len: usize) -> Option<usize> {
    let adjusted_index = if offset.is_negative() {
        index.checked_sub(offset.wrapping_abs() as usize)
    } else {
        index.checked_add(offset as usize)
    };
    match adjusted_index {
        Some(new_index) if new_index < len => Some(new_index),
        _ => None,
    }
}

#[test]
fn command_marks_edge() {
    let mut grid = Grid::new();

    assert_eq!(grid.data[500][500].is_dug, true);

    grid.execute(&Command {
        dir: Direction::Right,
        len: 6,
    });

    assert_eq!(grid.data[500][501].is_dug, true);
    assert_eq!(grid.data[500][502].is_dug, true);
    assert_eq!(grid.data[500][503].is_dug, true);
    assert_eq!(grid.data[500][504].is_dug, true);
    assert_eq!(grid.data[500][505].is_dug, true);
    assert_eq!(grid.data[500][506].is_dug, true);

    grid.execute(&Command {
        dir: Direction::Down,
        len: 1,
    });
    assert_eq!(grid.data[501][506].is_dug, true);
    assert_eq!(grid.data[501][500].is_dug, false);
}
