use crate::{Coordinate, Observation, SpaceMap};

pub trait ExpandsSpace {
    fn expand(&mut self, space_age_multiplier: &usize);
    fn collect_galaxies(&self) -> Vec<Coordinate>;
}

impl ExpandsSpace for SpaceMap {
    fn collect_galaxies(&self) -> Vec<Coordinate> {
        self.grid
            .iter()
            .flat_map(|row| {
                row.iter().filter_map(|loc| {
                    if loc.observation == Observation::Galaxy {
                        Some(loc.coordinate.clone())
                    } else {
                        None
                    }
                })
            })
            .collect()
    }
    fn expand(&mut self, space_age_multiplier: &usize) {
        let empties = self.counts_empty_rows_and_columns();
        let galaxies = self
            .collect_galaxies()
            .iter()
            .map(|galaxy| Coordinate {
                x: galaxy.x
                    + empties.1.iter().filter(|x| galaxy.x > **x).count()
                        * (space_age_multiplier - 1),
                y: galaxy.y
                    + empties.0.iter().filter(|y| galaxy.y > **y).count()
                        * (space_age_multiplier - 1),
            })
            .collect();
        self.expanded_galaxies = Some(galaxies);
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
    fn collects_galaxies() {
        let input = "....#........
.........#...
#............
.............
.............
........#....
.#...........
............#
.............
.............
.........#...
#....#.......";

        let space_map = SpaceMap::parse_grid(input);

        let galaxies: Vec<Coordinate> = space_map.collect_galaxies();

        assert_eq!(galaxies.len(), 9);
        assert_eq!(
            galaxies,
            vec![
                Coordinate { x: 4, y: 0 },
                Coordinate { x: 9, y: 1 },
                Coordinate { x: 0, y: 2 },
                Coordinate { x: 8, y: 5 },
                Coordinate { x: 1, y: 6 },
                Coordinate { x: 12, y: 7 },
                Coordinate { x: 9, y: 10 },
                Coordinate { x: 0, y: 11 },
                Coordinate { x: 5, y: 11 }
            ]
        )
    }

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
        let input = ".#..
...#";
        let mut space_map = SpaceMap::parse_grid(input);
        space_map.expand(&2);
        let expected = vec![Coordinate { x: 2, y: 0 }, Coordinate { x: 5, y: 1 }];
        assert_eq!(space_map.expanded_galaxies, Some(expected));
    }
}
