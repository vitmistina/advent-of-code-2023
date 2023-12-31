use super::*;

mod ordinal;
mod quadrant;

pub fn integrate_with_simplification(input: &str, steps: u64) -> usize {
    let mut garden = Garden::parse(input);
    let mut stats = Stats {
        horizontals: Vec::new(),
        quadrants: Vec::new(),
    };

    let mut iteration = 0;
    let mut is_finished = false;
    while is_finished == false {
        garden.spread_infinitely(&iteration, &mut stats);
        iteration += 1;

        if iteration > 100 {
            println!("{iteration}");
            panic!("too long");
        }

        if let Some(ord) = stats.horizontals.get(0) {
            let has_required_snapshots = ord.snapshots.len() == (garden.x_size * 4 / 2) as usize;
            is_finished = stats.horizontals.len() == 4
                && stats.quadrants.len() == 4
                && has_required_snapshots;
        }
    }

    let quadrants: usize = stats
        .quadrants
        .iter()
        .map(|quad| garden.simulate_in_quadrant(quad, steps as usize))
        .sum();

    let ordinals: usize = stats
        .horizontals
        .iter()
        .map(|ord| garden.simulate_in_ordinal(ord, steps as usize))
        .sum();

    let rounds_to_simulate = (garden.x_size as usize * 4) + (steps as usize % 2);
    let sim_garden = garden.simulate_garden(
        &TileStatistics {
            position: Coordinate { x: 0, y: 0 },
            tile: Tile {
                starting: Coordinate { x: 0, y: 0 },
                iteration_started: 0,
            },
            repeats_every: 0,
            snapshots: HashMap::new(),
        },
        rounds_to_simulate,
    );
    let center = sim_garden.steps.len();

    quadrants + ordinals + center
}

fn compute_filled_triangle(
    iterations: usize,
    quadrant_start: usize,
    repeat: usize,
) -> (usize, usize) {
    let base = (iterations - quadrant_start) / repeat - 1;

    compute_even_odd_triangle(base)
}

fn compute_even_odd_triangle(base: usize) -> (usize, usize) {
    let n = base / 2; // For even numbers

    let even_sum = n * (n + 1);
    let odd_sum = if base % 2 == 0 {
        n * n // Last number is even, so there are equal numbers of odd and even
    } else {
        (n + 1) * (n + 1) // Last number is odd, so one more odd number
    };

    (even_sum, odd_sum)
}

fn compute_edge(iterations: usize, quadrant_start: usize, repeat: usize) -> usize {
    1 + (iterations - quadrant_start) / repeat
}

impl Garden {
    fn simulate_in_quadrant(&self, quadrant: &TileStatistics, iterations: usize) -> usize {
        let rounds_to_simulate =
            (iterations - quadrant.tile.iteration_started) % quadrant.repeats_every;
        let sim_garden = self.simulate_garden(&quadrant, rounds_to_simulate);
        let leading_edge = compute_edge(
            iterations,
            quadrant.tile.iteration_started,
            quadrant.repeats_every,
        );
        let leading_steps = sim_garden.steps.len() * leading_edge;

        let rounds_to_simulate = quadrant.repeats_every
            + (iterations - quadrant.tile.iteration_started) % quadrant.repeats_every;
        let sim_garden = self.simulate_garden(&quadrant, rounds_to_simulate);
        let trailing_edge = compute_edge(
            iterations - quadrant.repeats_every,
            quadrant.tile.iteration_started,
            quadrant.repeats_every,
        );
        let trailing_steps = sim_garden.steps.len() * trailing_edge;

        let (even, odd) = compute_filled_triangle(
            iterations,
            quadrant.tile.iteration_started,
            quadrant.repeats_every,
        );

        let rounds_to_simulate = (self.x_size as usize * 4) + (iterations % 2);
        let sim_garden = self.simulate_garden(&quadrant, rounds_to_simulate);
        let even_triangle_steps = sim_garden.steps.len() * even;

        let rounds_to_simulate = (self.x_size as usize * 4) + (iterations % 2) + 1;
        let sim_garden = self.simulate_garden(&quadrant, rounds_to_simulate);
        let odd_triangle_steps = sim_garden.steps.len() * odd;

        leading_steps + trailing_steps + even_triangle_steps + odd_triangle_steps
    }
}

impl Garden {
    fn simulate_in_ordinal(&self, ordinal: &TileStatistics, iterations: usize) -> usize {
        let rounds_to_simulate =
            (iterations - ordinal.tile.iteration_started) % ordinal.repeats_every;
        let leading_edge = *ordinal.snapshots.get(&rounds_to_simulate).unwrap();

        let rounds_to_simulate = ((iterations - ordinal.tile.iteration_started)
            % ordinal.repeats_every)
            + ordinal.repeats_every;
        // let sim_garden = self.simulate_garden(&ordinal, rounds_to_simulate);
        // let trailing_edge = sim_garden.steps.len();
        let trailing_edge = *ordinal.snapshots.get(&rounds_to_simulate).unwrap();

        let (even, odd) = {
            let steady_ordinal_squares =
                (iterations - ordinal.tile.iteration_started) / ordinal.repeats_every - 2
                    + ordinal.position.x.abs() as usize
                    + ordinal.position.y.abs() as usize;
            get_even_odd(steady_ordinal_squares)
        };

        let rounds_to_simulate = (self.x_size as usize * 4) + (iterations % 2) + 1;
        let sim_garden = self.simulate_garden(&ordinal, rounds_to_simulate);
        let steady_state_even = sim_garden.steps.len() * even;

        let rounds_to_simulate = (self.x_size as usize * 4) + (iterations % 2);
        let sim_garden = self.simulate_garden(&ordinal, rounds_to_simulate);
        let steady_state_odd = sim_garden.steps.len() * odd;

        leading_edge + trailing_edge + steady_state_even + steady_state_odd
    }
}

fn get_even_odd(base: usize) -> (usize, usize) {
    let n = base / 2; // For even numbers

    let odd = if base % 2 == 0 {
        n // Last number is even, so there are equal numbers of odd and even
    } else {
        n + 1 // Last number is odd, so one more odd number
    };

    (n, odd)
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
    fn computes_even_and_odd() {
        assert_eq!(compute_even_odd_triangle(1), (0, 1));
        assert_eq!(compute_even_odd_triangle(2), (2, 1));
        assert_eq!(compute_even_odd_triangle(3), (2, 4));
        assert_eq!(compute_even_odd_triangle(4), (6, 4));
        assert_eq!(compute_even_odd_triangle(7), (12, 16));
    }

    #[test]
    fn computes_triangle_area() {
        let iterations = 50;
        let quadrant_start = 15;
        let repeat = 11;
        let result = compute_filled_triangle(iterations, quadrant_start, repeat);
        assert_eq!(result, (2, 1));
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
            position: Coordinate { x: -1, y: -2 },
            tile: Tile {
                starting: Coordinate { x: 10, y: 10 },
                iteration_started: 11,
            },
            repeats_every: 11,
            snapshots: HashMap::new(),
        };
        let iterations = 100;

        let result: usize = garden.simulate_in_quadrant(&quadrant, iterations);
        assert_eq!(result, 1317);
    }

    #[test]
    fn simulates_ordinal() {
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
            snapshots: HashMap::from([(2, 4), (13, 39)]),
        };
        let iterations = 100;

        let result: usize = garden.simulate_in_ordinal(&horizontal, iterations);
        assert_eq!(result, 325);
    }

    #[ignore = "I found a much easier way to make this work"]
    #[test]
    fn integrates_100_steps() {
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
        let result = integrate_with_simplification(input, 100);
        todo!();
        assert_eq!(result, 6536);
    }

    #[ignore = "I found a much easier way to make this work"]
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
        let result = integrate_with_simplification(input, 5000);
        todo!();
        assert_eq!(result, 16733044);
    }
}
