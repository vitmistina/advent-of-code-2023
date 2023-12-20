use crate::directions::get_dir_count;

use super::*;

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

impl Grid {
    pub fn calculate_neighbors(&mut self, node: &Node) -> (Vec<Node>, Option<u64>) {
        let directions = node.find_directions(&self.max_repeat, &self.min_repeat);
        let mut unvisited = Vec::new();
        for direction in directions {
            let (new_x, new_y, x_len, y_len) =
                self.get_coords_for_direction(&direction, &node.coord);

            match (new_x, new_y) {
                (Some(x), Some(y)) => {
                    let next_directions = [node.prev_directions.as_slice(), &[direction]].concat();
                    let next_dir_count =
                        get_dir_count(&next_directions, &direction, &self.max_repeat.into());
                    let next_node = &mut self
                        .data
                        .get_mut(&(next_dir_count.into(), direction))
                        .unwrap()[y][x];
                    let h = (x_len - (x * 1).min(x_len) + y_len - (y * 1).min(y_len)) as u64;
                    let g = node.current_score.unwrap() + next_node.heat_loss as u64;
                    let f = g + h;

                    next_node.prev_directions = next_directions;

                    if next_node.is_target == true && next_dir_count >= self.min_repeat {
                        return (Vec::new(), Some(g));
                    }

                    if next_node.is_visited == false
                        && (next_node.heuristic_current_score.is_none()
                            || next_node
                                .heuristic_current_score
                                .is_some_and(|current_f| f < current_f))
                    {
                        next_node.current_score = Some(g);
                        next_node.heuristic_current_score = Some(f);

                        unvisited.push(next_node.clone());
                        // if next_dir_count >= self.min_repeat {

                        // }
                    }
                }
                _ => (),
            };
        }
        let prev_dir = node
            .prev_directions
            .iter()
            .cloned()
            .last()
            .unwrap_or_default();
        let prev_dir_count =
            get_dir_count(&node.prev_directions, &prev_dir, &self.max_repeat.into()).max(1);

        let node = &mut self
            .data
            .get_mut(&(prev_dir_count.into(), prev_dir))
            .unwrap()[node.coord.y][node.coord.x];
        node.is_visited = true;
        (unvisited, None)
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
        let (x_len, y_len) = {
            (
                self.data.get(&(1, Direction::Right)).unwrap()[0].len(),
                self.data.get(&(1, Direction::Right)).unwrap().len(),
            )
        };
        let new_x = get_coords(coord.x, x_offset, x_len);
        let new_y = get_coords(coord.y, y_offset, y_len);
        (new_x, new_y, x_len, y_len)
    }
}

#[test]
fn calculates_neighbors() {
    let input = "123";
    let mut grid = Grid::parse(input, &1, &3);
    let mut node = &mut grid.data.get_mut(&(1, Direction::Right)).unwrap()[0][0].clone();
    assert_eq!(node.is_visited, false);
    let result = grid.calculate_neighbors(node);
    assert_eq!(result.0.len(), 1);
    let node = &grid.data.get(&(1, Direction::Right)).unwrap()[0][0].clone();
    assert_eq!(node.is_visited, true);

    let mut node = &mut grid.data.get_mut(&(1, Direction::Right)).unwrap()[0][1].clone();
    let result = grid.calculate_neighbors(node);
    assert_eq!(result.1, Some(5));
}
