fn main() {
    println!("Hello, world!");
}

mod parsing;
use parsing::ParseGrid;

mod expand;
use expand::ExpandsSpace;

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

#[derive(Debug, PartialEq, Clone)]
struct Location {
    coordinate: Coordinate,
    observation: Observation,
    is_expanded: bool,
}

struct SpaceMap {
    grid: Vec<Vec<Location>>,
}

#[cfg(test)]
mod tests {
    use super::*;

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

        let mut space_map = SpaceMap::parse_grid(input);
        assert_eq!(space_map.grid.len(), 10);

        space_map.expand();
        assert_eq!(space_map.grid.len(), 12);
        assert_eq!(space_map.grid[0].len(), 13);

        todo!()
    }
}
