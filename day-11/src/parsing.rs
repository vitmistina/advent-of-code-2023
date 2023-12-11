use crate::{Coordinate, Location, Observation, SpaceMap};

pub trait ParseGrid {
    fn parse_grid(input: &str) -> Self;
}

impl ParseGrid for SpaceMap {
    fn parse_grid(input: &str) -> Self {
        Self {
            grid: input
                .lines()
                .enumerate()
                .map(|(y, line)| {
                    line.chars()
                        .enumerate()
                        .map(|(x, ch)| Location {
                            coordinate: Coordinate { x, y },
                            observation: if ch == '#' {
                                Observation::Galaxy
                            } else {
                                Observation::Space
                            },
                            is_expanded: false,
                        })
                        .collect()
                })
                .collect(),
        }
    }
}

#[cfg(test)]
mod t {
    use super::*;
    #[test]
    fn parses_grid() {
        let input = "...#......
.......#..";
        let expected = vec![
            vec![
                Location {
                    coordinate: Coordinate { x: 0, y: 0 },
                    observation: Observation::Space,
                    is_expanded: false,
                },
                Location {
                    coordinate: Coordinate { x: 1, y: 0 },
                    observation: Observation::Space,
                    is_expanded: false,
                },
                Location {
                    coordinate: Coordinate { x: 2, y: 0 },
                    observation: Observation::Space,
                    is_expanded: false,
                },
                Location {
                    coordinate: Coordinate { x: 3, y: 0 },
                    observation: Observation::Galaxy,
                    is_expanded: false,
                },
                Location {
                    coordinate: Coordinate { x: 4, y: 0 },
                    observation: Observation::Space,
                    is_expanded: false,
                },
                Location {
                    coordinate: Coordinate { x: 5, y: 0 },
                    observation: Observation::Space,
                    is_expanded: false,
                },
                Location {
                    coordinate: Coordinate { x: 6, y: 0 },
                    observation: Observation::Space,
                    is_expanded: false,
                },
                Location {
                    coordinate: Coordinate { x: 7, y: 0 },
                    observation: Observation::Space,
                    is_expanded: false,
                },
                Location {
                    coordinate: Coordinate { x: 8, y: 0 },
                    observation: Observation::Space,
                    is_expanded: false,
                },
                Location {
                    coordinate: Coordinate { x: 9, y: 0 },
                    observation: Observation::Space,
                    is_expanded: false,
                },
            ],
            vec![
                Location {
                    coordinate: Coordinate { x: 0, y: 1 },
                    observation: Observation::Space,
                    is_expanded: false,
                },
                Location {
                    coordinate: Coordinate { x: 1, y: 1 },
                    observation: Observation::Space,
                    is_expanded: false,
                },
                Location {
                    coordinate: Coordinate { x: 2, y: 1 },
                    observation: Observation::Space,
                    is_expanded: false,
                },
                Location {
                    coordinate: Coordinate { x: 3, y: 1 },
                    observation: Observation::Space,
                    is_expanded: false,
                },
                Location {
                    coordinate: Coordinate { x: 4, y: 1 },
                    observation: Observation::Space,
                    is_expanded: false,
                },
                Location {
                    coordinate: Coordinate { x: 5, y: 1 },
                    observation: Observation::Space,
                    is_expanded: false,
                },
                Location {
                    coordinate: Coordinate { x: 6, y: 1 },
                    observation: Observation::Space,
                    is_expanded: false,
                },
                Location {
                    coordinate: Coordinate { x: 7, y: 1 },
                    observation: Observation::Galaxy,
                    is_expanded: false,
                },
                Location {
                    coordinate: Coordinate { x: 8, y: 1 },
                    observation: Observation::Space,
                    is_expanded: false,
                },
                Location {
                    coordinate: Coordinate { x: 9, y: 1 },
                    observation: Observation::Space,
                    is_expanded: false,
                },
            ],
        ];

        assert_eq!(SpaceMap::parse_grid(input).grid, expected);
    }
}
