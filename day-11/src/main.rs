mod parsing;
use std::{fs, future::IntoFuture};

use parsing::ParseGrid;

mod expand;
use expand::ExpandsSpace;

mod galaxy_relations;
use galaxy_relations::PairsGalaxies;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let space_map = SpaceMap::calculate_galaxies(&input, &2);
    println!(
        "Hello, world! {}",
        space_map.shortest_path_between_pairs.unwrap()
    );

    let input = fs::read_to_string("input.txt").unwrap();
    let space_map = SpaceMap::calculate_galaxies(&input, &1000000);
    println!(
        "Hello, very old world! {}",
        space_map.shortest_path_between_pairs.unwrap()
    );
}

#[derive(Debug, PartialEq, Clone)]
enum Observation {
    Space,
    Galaxy,
}

#[derive(Debug, PartialEq, Clone)]
struct Coordinate {
    x: usize,
    y: usize,
}

impl Coordinate {
    fn distance_from(&self, target: &Coordinate) -> usize {
        self.x.abs_diff(target.x) + self.y.abs_diff(target.y)
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Location {
    coordinate: Coordinate,
    observation: Observation,
    is_expanded: bool,
}

struct SpaceMap {
    grid: Vec<Vec<Location>>,
    shortest_path_between_pairs: Option<usize>,
}

impl SpaceMap {
    fn calculate_galaxies(input: &str, space_age_multiplier: &usize) -> Self {
        let mut space_map = SpaceMap::parse_grid(input);
        space_map.expand(space_age_multiplier);
        let galaxies = space_map.collect_galaxies();
        let pairs = SpaceMap::establish_pairs(galaxies);

        space_map.shortest_path_between_pairs =
            Some(pairs.iter().map(|pair| pair.0.distance_from(&pair.1)).sum());
        space_map
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn measures_distances() {
        assert_eq!(
            Coordinate { x: 1, y: 6 }.distance_from(&Coordinate { x: 5, y: 11 }),
            9
        );
    }

    #[test]
    fn part_one_integration() {
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

        assert_eq!(
            SpaceMap::calculate_galaxies(input, &2).shortest_path_between_pairs,
            Some(374)
        );
        assert_eq!(
            SpaceMap::calculate_galaxies(input, &10).shortest_path_between_pairs,
            Some(1030)
        );
        assert_eq!(
            SpaceMap::calculate_galaxies(input, &100).shortest_path_between_pairs,
            Some(8410)
        );
    }
}
