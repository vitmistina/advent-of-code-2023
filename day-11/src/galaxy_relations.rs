use crate::{Coordinate, Observation, SpaceMap};

pub trait PairsGalaxies {
    fn collect_galaxies(&self) -> Vec<Coordinate>;
    fn establish_pairs(galaxies: Vec<Coordinate>) -> Vec<(Coordinate, Coordinate)>;
}

impl PairsGalaxies for SpaceMap {
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

    fn establish_pairs(galaxies: Vec<Coordinate>) -> Vec<(Coordinate, Coordinate)> {
        let total_galaxies = galaxies.len();
        galaxies
            .iter()
            .enumerate()
            .flat_map(|(index, galaxy)| {
                ((index + 1)..total_galaxies)
                    .map(|next_galaxy| (galaxy.clone(), galaxies[next_galaxy].clone()))
            })
            .collect()
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
    fn establishes_pairs() {
        let input = vec![
            Coordinate { x: 4, y: 0 },
            Coordinate { x: 9, y: 1 },
            Coordinate { x: 0, y: 2 },
            Coordinate { x: 8, y: 5 },
            Coordinate { x: 1, y: 6 },
            Coordinate { x: 12, y: 7 },
            Coordinate { x: 9, y: 10 },
            Coordinate { x: 0, y: 11 },
            Coordinate { x: 5, y: 11 },
        ];

        let output: Vec<(Coordinate, Coordinate)> = SpaceMap::establish_pairs(input);

        assert_eq!(output.len(), 36);

        let input = vec![
            Coordinate { x: 4, y: 0 },
            Coordinate { x: 9, y: 1 },
            Coordinate { x: 0, y: 2 },
        ];

        let output: Vec<(Coordinate, Coordinate)> = SpaceMap::establish_pairs(input);

        assert_eq!(output.len(), 3);
        assert_eq!(
            output,
            vec![
                (Coordinate { x: 4, y: 0 }, Coordinate { x: 9, y: 1 }),
                (Coordinate { x: 4, y: 0 }, Coordinate { x: 0, y: 2 }),
                (Coordinate { x: 9, y: 1 }, Coordinate { x: 0, y: 2 })
            ]
        );
    }
}
