fn main() {
    println!("Hello, world!");
}

mod parsing;
use parsing::ParseGrid;

#[derive(Debug, PartialEq)]
enum Observation {
    Space,
    Galaxy,
}

#[derive(Debug, PartialEq)]
struct Coordinate {
    x: usize,
    y: usize,
}

#[derive(Debug, PartialEq)]
struct Location {
    coordinate: Coordinate,
    observation: Observation,
    is_expanded: bool,
}

struct SpaceMap {
    grid: Vec<Vec<Location>>,
}

trait ExpandsSpace {
    fn expand(&mut self);
}

impl ExpandsSpace for SpaceMap {
    fn expand(&mut self) {}
}

#[cfg(test)]
mod tests {
    use super::*;

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
    }

    #[test]
    fn collects_galaxies() {
        todo!()
    }

    #[test]
    fn establishes_pairs() {
        todo!()
    }

    #[test]
    fn measures_distances() {
        todo!()
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

        let space_map = SpaceMap::parse_grid(input);
        assert_eq!(space_map.grid.len(), 10);
        todo!()
    }
}
