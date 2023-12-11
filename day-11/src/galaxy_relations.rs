use crate::{Coordinate, SpaceMap};

pub trait PairsGalaxies {
    fn establish_pairs(&self) -> Vec<(Coordinate, Coordinate)>;
}

impl PairsGalaxies for SpaceMap {
    fn establish_pairs(&self) -> Vec<(Coordinate, Coordinate)> {
        let galaxies = self.expanded_galaxies.as_ref().unwrap();
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
    use super::*;

    #[test]
    fn establishes_pairs() {
        let space_map = SpaceMap {
            grid: vec![],
            expanded_galaxies: Some(vec![
                Coordinate { x: 4, y: 0 },
                Coordinate { x: 9, y: 1 },
                Coordinate { x: 0, y: 2 },
                Coordinate { x: 8, y: 5 },
                Coordinate { x: 1, y: 6 },
                Coordinate { x: 12, y: 7 },
                Coordinate { x: 9, y: 10 },
                Coordinate { x: 0, y: 11 },
                Coordinate { x: 5, y: 11 },
            ]),
            shortest_path_between_pairs: None,
        };

        let output: Vec<(Coordinate, Coordinate)> = space_map.establish_pairs();

        assert_eq!(output.len(), 36);

        let space_map = SpaceMap {
            grid: vec![],
            expanded_galaxies: Some(vec![
                Coordinate { x: 4, y: 0 },
                Coordinate { x: 9, y: 1 },
                Coordinate { x: 0, y: 2 },
            ]),
            shortest_path_between_pairs: None,
        };

        let output: Vec<(Coordinate, Coordinate)> = space_map.establish_pairs();

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
