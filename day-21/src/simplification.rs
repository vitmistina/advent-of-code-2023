use super::*;

mod horizontal;
mod quadrant;

pub fn integrate_with_simplification(input: &str, steps: u64) -> u64 {
    todo!()
}

struct Stats {
    horizontals: Vec<(TileStatistics, HashSet<Coordinate>)>,
    quadrants: Vec<TileStatistics>,
}

#[derive(Debug, PartialEq)]
struct TileStatistics {
    position: Coordinate,
    tile: Tile,
    repeats_every: usize,
}

fn compute_filled_triangle(iterations: usize, quadrant_start: usize, repeat: usize) -> usize {
    let base = (iterations - quadrant_start) / repeat - 1;

    base * (base + 1) / 2
}

fn compute_edge(iterations: usize, quadrant_start: usize, repeat: usize) -> usize {
    1 + (iterations - quadrant_start) / repeat
}

impl Garden {
    fn simulate_in_quadrant(&self, quadrant: TileStatistics, iterations: usize) -> usize {
        let rounds_to_simulate =
            (iterations - quadrant.tile.iteration_started) % quadrant.repeats_every;
        let sim_garden = self.simulate_garden(&quadrant, rounds_to_simulate);
        let leading_steps = sim_garden.steps.len()
            * compute_edge(
                iterations,
                quadrant.tile.iteration_started,
                quadrant.repeats_every,
            );

        let rounds_to_simulate = quadrant.repeats_every
            + (iterations - quadrant.tile.iteration_started) % quadrant.repeats_every;
        let sim_garden = self.simulate_garden(&quadrant, rounds_to_simulate);
        let trailing_steps = sim_garden.steps.len()
            * compute_edge(
                iterations - quadrant.repeats_every,
                quadrant.tile.iteration_started,
                quadrant.repeats_every,
            );

        let rounds_to_simulate = (self.x_size as usize * 4) + (iterations % 2);
        let sim_garden = self.simulate_garden(&quadrant, rounds_to_simulate);
        let triangle_steps = sim_garden.steps.len()
            * compute_filled_triangle(
                iterations,
                quadrant.tile.iteration_started,
                quadrant.repeats_every,
            );

        leading_steps + trailing_steps + triangle_steps
    }
}

impl Garden {
    fn simulate_in_horizontal(&self, ordinal: TileStatistics, iterations: usize) -> usize {
        let rounds_to_simulate =
            (iterations - ordinal.tile.iteration_started) % ordinal.repeats_every;

        let sim_garden = self.simulate_garden(&ordinal, rounds_to_simulate);
        let edge = sim_garden.steps.len();

        sim_garden.print();

        edge
    }
}

impl Garden {
    fn simulate_garden(&self, quadrant: &TileStatistics, rounds_to_simulate: usize) -> Garden {
        let mut sim_garden = Garden {
            rocks: self.rocks.clone(),
            steps: HashSet::from([quadrant.tile.starting.clone()]),
            y_size: self.y_size,
            x_size: self.x_size,
            tiles: HashMap::new(),
        };

        for _ in 1..rounds_to_simulate {
            sim_garden.spread()
        }
        sim_garden
    }
}

#[cfg(test)]
mod t {
    use super::*;

    #[test]
    fn computes_triangle_area() {
        let iterations = 50;
        let quadrant_start = 15;
        let repeat = 11;
        let result: usize = compute_filled_triangle(iterations, quadrant_start, repeat);
        assert_eq!(result, 3);
    }

    #[test]
    fn computes_triangle_edge() {
        let iterations = 50;
        let quadrant_start = 15;
        let repeat = 11;
        let leading_edge: usize = compute_edge(iterations, quadrant_start, repeat);
        assert_eq!(leading_edge, 4);
        let trailing_edge: usize = compute_edge(iterations - repeat, quadrant_start, repeat);
        assert_eq!(trailing_edge, 3);
    }

    #[test]
    fn simulates_for_quadrant() {
        let input = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";
        let garden: Garden = Garden::parse(input);
        let quadrant = TileStatistics {
            position: Coordinate { x: 1, y: 2 },
            tile: Tile {
                starting: Coordinate { x: 0, y: 0 },
                iteration_started: 15,
            },
            repeats_every: 11,
        };
        let iterations = 50;

        let result: usize = garden.simulate_in_quadrant(quadrant, iterations);
        assert_eq!(result, 212);
    }

    #[test]
    fn simulates_horizontal() {
        let input = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";
        let garden: Garden = Garden::parse(input);
        let horizontal = TileStatistics {
            position: Coordinate { x: 2, y: 0 },
            tile: Tile {
                starting: Coordinate { x: 0, y: 0 },
                iteration_started: 21,
            },
            repeats_every: 11,
        };
        let iterations = 50;

        let result: usize = garden.simulate_in_horizontal(horizontal, iterations);
        assert_eq!(result, 212);
    }

    #[ignore = "not yet implemented"]
    #[test]
    fn integrates_5000_steps() {
        let input = "...........
    .....###.#.
    .###.##..#.
    ..#.#...#..
    ....#.#....
    .##..S####.
    .##..#...#.
    .......##..
    .##.#.####.
    .##..##.##.
    ...........";
        let result: u64 = integrate_with_simplification(input, 5000);
        assert_eq!(result, 16733044);
    }
}
