use crate::{Coordinate, Location, Observation, SpaceMap};

pub trait ExpandsSpace {
    fn expand(&mut self);
}

impl ExpandsSpace for SpaceMap {
    fn expand(&mut self) {
        let empties = self.counts_empty_rows_and_columns();

        let location_template = Location {
            coordinate: Coordinate { x: 0, y: 0 },
            observation: Observation::Space,
            is_expanded: true,
        };

        let mut expanded_grid: Vec<Vec<Location>> = Vec::new();

        for (y, row) in self.grid.iter().enumerate() {
            if empties.0.contains(&y) {
                let extra_row = (0..self.grid[0].len())
                    .map(|_| location_template.clone())
                    .collect::<Vec<_>>();
                expanded_grid.push(extra_row);
            }

            expanded_grid.push(row.clone());
        }

        let mut x_offset = 0;
        for x in 0..expanded_grid[0].len() {
            if empties.1.contains(&x) {
                for row in expanded_grid.iter_mut() {
                    row.insert(x + x_offset, location_template.clone());
                }
                x_offset += 1;
            }
        }

        expanded_grid.iter_mut().enumerate().for_each(|(y, row)| {
            row.iter_mut().enumerate().for_each(|(x, loc)| {
                loc.coordinate.x = x;
                loc.coordinate.y = y;
            })
        });

        self.grid = expanded_grid;
    }
}

impl SpaceMap {
    fn counts_empty_rows_and_columns(&self) -> (Vec<usize>, Vec<usize>) {
        let empty_rows = self
            .grid
            .iter()
            .enumerate()
            .filter_map(|(y, row)| {
                if row.iter().all(|loc| loc.observation == Observation::Space) {
                    Some(y)
                } else {
                    None
                }
            })
            .collect();

        let empty_columns = (0..self.grid[0].len())
            .filter_map(|x| {
                if self
                    .grid
                    .iter()
                    .all(|row| row[x].observation == Observation::Space)
                {
                    Some(x)
                } else {
                    None
                }
            })
            .collect();

        (empty_rows, empty_columns)
    }
}

#[cfg(test)]
mod t {
    use crate::parsing::ParseGrid;

    use super::*;

    #[test]
    fn counts_empty_rows_and_columns() {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

        let space_map = SpaceMap::parse_grid(input);

        assert_eq!(
            space_map.counts_empty_rows_and_columns(),
            (vec![3, 7], vec![2, 5, 8])
        );

        let input = ".#.
...";
        let space_map = SpaceMap::parse_grid(input);

        assert_eq!(
            space_map.counts_empty_rows_and_columns(),
            (vec![1], vec![0, 2])
        );
    }

    #[test]
    fn expands_space() {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
        let mut space_map = SpaceMap::parse_grid(input);
        space_map.expand();
        assert_eq!(space_map.grid.len(), 12);
        assert_eq!(space_map.grid[0].len(), 13);
        assert_eq!(space_map.grid.iter().map(|row| row.len()).max(), Some(13));

        let input = ".#.
...";
        let mut space_map = SpaceMap::parse_grid(input);
        space_map.expand();
        let expected = vec![
            vec![
                Location {
                    coordinate: Coordinate { x: 0, y: 0 },
                    observation: Observation::Space,
                    is_expanded: true,
                },
                Location {
                    coordinate: Coordinate { x: 1, y: 0 },
                    observation: Observation::Space,
                    is_expanded: false,
                },
                Location {
                    coordinate: Coordinate { x: 2, y: 0 },
                    observation: Observation::Galaxy,
                    is_expanded: false,
                },
                Location {
                    coordinate: Coordinate { x: 3, y: 0 },
                    observation: Observation::Space,
                    is_expanded: true,
                },
                Location {
                    coordinate: Coordinate { x: 4, y: 0 },
                    observation: Observation::Space,
                    is_expanded: false,
                },
            ],
            vec![
                Location {
                    coordinate: Coordinate { x: 0, y: 1 },
                    observation: Observation::Space,
                    is_expanded: true,
                },
                Location {
                    coordinate: Coordinate { x: 1, y: 1 },
                    observation: Observation::Space,
                    is_expanded: true,
                },
                Location {
                    coordinate: Coordinate { x: 2, y: 1 },
                    observation: Observation::Space,
                    is_expanded: true,
                },
                Location {
                    coordinate: Coordinate { x: 3, y: 1 },
                    observation: Observation::Space,
                    is_expanded: true,
                },
                Location {
                    coordinate: Coordinate { x: 4, y: 1 },
                    observation: Observation::Space,
                    is_expanded: true,
                },
            ],
            vec![
                Location {
                    coordinate: Coordinate { x: 0, y: 2 },
                    observation: Observation::Space,
                    is_expanded: true,
                },
                Location {
                    coordinate: Coordinate { x: 1, y: 2 },
                    observation: Observation::Space,
                    is_expanded: false,
                },
                Location {
                    coordinate: Coordinate { x: 2, y: 2 },
                    observation: Observation::Space,
                    is_expanded: false,
                },
                Location {
                    coordinate: Coordinate { x: 3, y: 2 },
                    observation: Observation::Space,
                    is_expanded: true,
                },
                Location {
                    coordinate: Coordinate { x: 4, y: 2 },
                    observation: Observation::Space,
                    is_expanded: false,
                },
            ],
        ];
        assert_eq!(space_map.grid, expected);
    }
}
