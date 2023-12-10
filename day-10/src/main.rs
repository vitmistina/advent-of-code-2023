use std::{collections::HashMap, fs, vec};

fn main() {
    let input = fs::read_to_string("data.txt").unwrap();
    let mut grid = Grid::parse_grid(&input);
    grid.calculate_loop();
    println!("Hello, world! {}", grid.farthes_distance.unwrap());
    grid.find_contained();
    println!("Nest, world boundaries! {}", grid.contained.unwrap());

    let mut grid = Grid::parse_grid(&input);
    grid.calculate_loop();
    grid.floodfill();
    println!("Nest, world floodfill! {}", grid.contained.unwrap());
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Coordinate {
    x: usize,
    y: usize,
}

#[derive(Eq, PartialEq, Hash)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

#[derive(Debug, PartialEq)]
struct Location {
    is_pipe: bool,
    coord: Coordinate,
    shape: char,
    is_main_loop: Option<bool>,
    distance_from_start: Option<u16>,
    is_inside: Option<bool>,
}

impl Location {
    fn get_matching_pipes() -> HashMap<Direction, Vec<char>> {
        HashMap::from([
            (Direction::Up, vec!['|', 'F', '7']),
            (Direction::Down, vec!['|', 'L', 'J']),
            (Direction::Right, vec!['-', 'J', '7']),
            (Direction::Left, vec!['-', 'F', 'L']),
        ])
    }

    fn get_pipe_directions() -> HashMap<char, Vec<Direction>> {
        HashMap::from([
            ('|', vec![Direction::Down, Direction::Up]),
            ('-', vec![Direction::Right, Direction::Left]),
            ('L', vec![Direction::Up, Direction::Right]),
            ('J', vec![Direction::Up, Direction::Left]),
            ('7', vec![Direction::Down, Direction::Left]),
            ('F', vec![Direction::Down, Direction::Right]),
            (
                'S',
                vec![
                    Direction::Up,
                    Direction::Down,
                    Direction::Right,
                    Direction::Left,
                ],
            ),
        ])
    }
}

struct Grid {
    locations: Vec<Vec<Location>>,
    start: Coordinate,
    matching_pipes: HashMap<Direction, Vec<char>>,
    pipe_directions: HashMap<char, Vec<Direction>>,
    farthes_distance: Option<u16>,
    size: (usize, usize),
    contained: Option<usize>,
}

impl Grid {
    fn parse_grid(input: &str) -> Self {
        let mut locations = Vec::new();
        let mut start = None;

        for (row_index, line) in input.lines().enumerate() {
            let row = line
                .chars()
                .enumerate()
                .map(|(col_index, shape)| {
                    let location = Coordinate {
                        x: col_index,
                        y: row_index,
                    };
                    match shape {
                        '.' => Location {
                            is_pipe: false,
                            coord: location,
                            shape,
                            is_main_loop: Some(false),
                            distance_from_start: None,
                            is_inside: None,
                        },
                        '|' => Location {
                            is_pipe: true,
                            coord: location,
                            shape,
                            is_main_loop: None,
                            distance_from_start: None,
                            is_inside: None,
                        },
                        '-' => Location {
                            is_pipe: true,
                            coord: location,
                            shape,
                            is_main_loop: None,
                            distance_from_start: None,
                            is_inside: None,
                        },
                        'L' => Location {
                            is_pipe: true,
                            coord: location,
                            shape,
                            is_main_loop: None,
                            distance_from_start: None,
                            is_inside: None,
                        },
                        'J' => Location {
                            is_pipe: true,
                            coord: location,
                            shape,
                            is_main_loop: None,
                            distance_from_start: None,
                            is_inside: None,
                        },
                        '7' => Location {
                            is_pipe: true,
                            coord: location,
                            shape,
                            is_main_loop: None,
                            distance_from_start: None,
                            is_inside: None,
                        },
                        'F' => Location {
                            is_pipe: true,
                            coord: location,
                            shape,
                            is_main_loop: None,
                            distance_from_start: None,
                            is_inside: None,
                        },
                        'S' => {
                            start = Some(location.clone());
                            Location {
                                is_pipe: true,
                                coord: location,
                                shape,
                                is_main_loop: Some(true),
                                distance_from_start: Some(0),
                                is_inside: None,
                            }
                        }
                        _ => panic!("unexpected character!"),
                    }
                })
                .collect::<Vec<_>>();
            locations.push(row);
        }

        let size = (locations.len(), locations.get(0).unwrap().len());

        Self {
            locations,
            start: start.unwrap(),
            matching_pipes: Location::get_matching_pipes(),
            pipe_directions: Location::get_pipe_directions(),
            farthes_distance: None,
            size,
            contained: None,
        }
    }

    fn mark_neighbors(
        &mut self,
        current: &Coordinate,
        previous: Option<&Coordinate>,
    ) -> Vec<Coordinate> {
        let mut neighbors = Vec::new();
        let mut neighboring_directions = Vec::new();

        let mut previous_distance = 0;

        {
            let current_loc = &self.locations[current.y][current.x];
            previous_distance = current_loc.distance_from_start.unwrap();

            for direction in self.pipe_directions.get(&current_loc.shape).unwrap() {
                if let Some(potential_coord) = calculate_offset(&direction, current) {
                    let is_different_from_previous = match previous {
                        Some(previous_value) => *previous_value != potential_coord,
                        None => true,
                    };

                    if potential_coord.y < self.locations.len()
                        && potential_coord.x < self.locations[0].len()
                        && is_different_from_previous
                    {
                        let potential = self
                            .locations
                            .get(potential_coord.y)
                            .unwrap()
                            .get(potential_coord.x)
                            .unwrap();
                        let matching_pipes = &self.matching_pipes[&direction];
                        if matching_pipes.contains(&potential.shape) {
                            neighbors.push(potential_coord.clone());
                            neighboring_directions.push(direction);
                        }
                    }
                }
            }
        }

        for neighbor in &neighbors {
            let potential = self
                .locations
                .get_mut(neighbor.y)
                .unwrap()
                .get_mut(neighbor.x)
                .unwrap();

            potential.is_main_loop = Some(true);
            match potential.distance_from_start {
                Some(_) => (),
                None => potential.distance_from_start = Some(previous_distance + 1),
            };
        }

        if previous.is_none() {
            for (key, dirs) in &self.pipe_directions {
                if dirs.contains(neighboring_directions[0])
                    && dirs.contains(neighboring_directions[1])
                {
                    let starting_loc = self
                        .locations
                        .get_mut(current.y)
                        .unwrap()
                        .get_mut(current.x)
                        .unwrap();
                    starting_loc.shape = *key;
                }
            }
        }

        neighbors
    }

    fn calculate_loop(&mut self) {
        let start = &self.start.clone();
        let mut neighbors = self.mark_neighbors(start, None);
        let mut previous_a_neighbor = *start;
        let mut previous_b_neighbor = *start;
        let mut is_end_found = false;

        while is_end_found == false {
            if let Some(first_neighbor) = neighbors.get_mut(0) {
                let next_a_neighbor = self
                    .mark_neighbors(first_neighbor, Some(&previous_a_neighbor))
                    .get(0)
                    .unwrap()
                    .clone();

                previous_a_neighbor = first_neighbor.clone();
                *first_neighbor = next_a_neighbor;
            }

            if let Some(second_neighbor) = neighbors.get_mut(1) {
                let next_b_neighbor = self
                    .mark_neighbors(second_neighbor, Some(&previous_b_neighbor))
                    .get(0)
                    .unwrap()
                    .clone();

                previous_b_neighbor = second_neighbor.clone();
                *second_neighbor = next_b_neighbor;
            }

            let current_first = neighbors.get(0).unwrap();
            let current_second = neighbors.get(1).unwrap();
            let current_path_length = self.locations[current_first.y][neighbors.get(0).unwrap().x]
                .distance_from_start
                .unwrap();

            if *current_first == *current_second {
                self.farthes_distance = Some(current_path_length);
                is_end_found = true;
            }
        }
    }

    fn find_contained(&mut self) {
        let mut boundary_info = Vec::new();

        for row in &self.locations {
            for location in row {
                let is_inside: bool = if location.is_main_loop.is_some_and(|x| x == true) {
                    false
                } else {
                    self.calculate_boundary_crossings(&location)
                };
                boundary_info.push((location.coord, is_inside))
            }
        }

        for (coord, is_inside) in boundary_info {
            let target_location = self
                .locations
                .get_mut(coord.y)
                .unwrap()
                .get_mut(coord.x)
                .unwrap();
            target_location.is_inside = Some(is_inside);
        }

        for row in &self.locations {
            let row_string = row
                .iter()
                .map(|loc| {
                    if loc.is_inside.unwrap() {
                        'I'
                    } else if loc.is_main_loop.is_some_and(|x| x == true) {
                        loc.shape
                    } else {
                        'O'
                    }
                })
                .collect::<String>();
            println!("{row_string}");
        }

        self.contained = Some(
            self.locations
                .iter()
                .map(|row| {
                    row.iter()
                        .filter(|location| location.is_inside.is_some_and(|x| x == true))
                        .count()
                })
                .sum::<usize>(),
        );
    }

    fn str_to_boundary_crossings(input: HashMap<Direction, &str>) -> bool {
        let boundary_crossings = input
            .iter()
            .map(|(direction, pipes)| {
                let mut left_up_crossings = 0;
                let mut bottom_right_crossings = 0;

                for evaluated_loc in pipes.chars() {
                    if *direction == Direction::Up || *direction == Direction::Down {
                        match evaluated_loc {
                            '-' => {
                                left_up_crossings += 1;
                                bottom_right_crossings += 1;
                            }
                            'F' => bottom_right_crossings += 1,
                            'L' => bottom_right_crossings += 1,
                            '7' => left_up_crossings += 1,
                            'J' => left_up_crossings += 1,
                            _ => (),
                        }
                    } else if *direction == Direction::Left || *direction == Direction::Right {
                        match evaluated_loc {
                            '|' => {
                                left_up_crossings += 1;
                                bottom_right_crossings += 1;
                            }
                            'L' => left_up_crossings += 1,
                            'J' => left_up_crossings += 1,
                            '7' => bottom_right_crossings += 1,
                            'F' => bottom_right_crossings += 1,
                            _ => (),
                        }
                    }
                }
                let result = (left_up_crossings, bottom_right_crossings, direction);
                result
            })
            .collect::<Vec<_>>();

        if boundary_crossings
            .iter()
            .any(|crossings| crossings.0 == 0 || crossings.1 == 0)
        {
            false
        } else if boundary_crossings
            .iter()
            .all(|crossing| crossing.0 % 2 == 1 && crossing.1 % 2 == 1)
        {
            true
        } else {
            false
        }
    }

    fn calculate_boundary_crossings(&self, location: &Location) -> bool {
        let boundary_crossings = [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ]
        .map(|direction| {
            let mut current_coord = location.coord.clone();
            let mut boundary_crossings = 0;
            let mut is_left_up_open = true;
            let mut is_bottom_right_open = true;
            let mut left_up_crossings = 0;
            let mut bottom_right_crossings = 0;
            while let Some(coord) = calculate_offset(&direction, &current_coord) {
                if coord.y == self.size.0 || coord.x == self.size.1 {
                    break;
                }
                current_coord = coord;

                let evaluated_loc = &self.locations[coord.y][coord.x];
                if evaluated_loc.is_main_loop.is_none()
                    || evaluated_loc.is_main_loop.is_some_and(|x| x == false)
                {
                    continue;
                }

                if direction == Direction::Up || direction == Direction::Down {
                    match evaluated_loc.shape {
                        '-' => {
                            is_left_up_open = false;
                            is_bottom_right_open = false;
                        }
                        'F' => is_bottom_right_open = false,
                        'L' => is_bottom_right_open = false,
                        '7' => is_left_up_open = false,
                        'J' => is_left_up_open = false,
                        _ => (),
                    }
                } else if direction == Direction::Left || direction == Direction::Right {
                    match evaluated_loc.shape {
                        '|' => {
                            is_left_up_open = false;
                            is_bottom_right_open = false;
                        }
                        'L' => is_left_up_open = false,
                        'J' => is_left_up_open = false,
                        '7' => is_bottom_right_open = false,
                        'F' => is_bottom_right_open = false,
                        _ => (),
                    }
                }

                if direction == Direction::Up || direction == Direction::Down {
                    match evaluated_loc.shape {
                        '-' => {
                            left_up_crossings += 1;
                            bottom_right_crossings += 1;
                        }
                        'F' => bottom_right_crossings += 1,
                        'L' => bottom_right_crossings += 1,
                        '7' => left_up_crossings += 1,
                        'J' => left_up_crossings += 1,
                        _ => (),
                    }
                } else if direction == Direction::Left || direction == Direction::Right {
                    match evaluated_loc.shape {
                        '|' => {
                            left_up_crossings += 1;
                            bottom_right_crossings += 1;
                        }
                        'L' => left_up_crossings += 1,
                        'J' => left_up_crossings += 1,
                        '7' => bottom_right_crossings += 1,
                        'F' => bottom_right_crossings += 1,
                        _ => (),
                    }
                }
                if is_left_up_open == false && is_bottom_right_open == false {
                    boundary_crossings += 1;
                    is_left_up_open = true;
                    is_bottom_right_open = true;
                }
            }
            // boundary_crossings

            (left_up_crossings, bottom_right_crossings)
        });

        if boundary_crossings
            .iter()
            .any(|crossings| crossings.0 == 0 || crossings.1 == 0)
        {
            false
        } else if boundary_crossings
            .iter()
            .all(|crossing| crossing.0 % 2 == 1 && crossing.1 % 2 == 1)
        {
            true
        } else {
            false
        }
    }

    fn floodfill(&mut self) {
        let mut max_grid = self.expand_grid();

        fill(&mut max_grid);

        let shrinked = shrink_grid(&max_grid);

        self.contained = Some(
            shrinked
                .iter()
                .map(|row| row.iter().filter(|char| *char == &'.').count())
                .sum(),
        );

        for row in shrinked {
            let row_string = row.into_iter().collect::<String>();
            println!("{row_string}");
        }
    }

    fn expand_grid(&self) -> Vec<Vec<char>> {
        let mut max_grid = Vec::new();
        let first_row = (0..self.locations[0].len() * 2 + 1)
            .into_iter()
            .map(|_| '.')
            .collect::<Vec<_>>();
        max_grid.push(first_row);

        for row in &self.locations {
            let mut max_row = Vec::new();
            let mut second_max_row = Vec::new();

            max_row.push('.');
            second_max_row.push('.');

            for loc in row {
                if loc.is_main_loop.is_some_and(|x| x == true) {
                    max_row.push('#');
                    let matching_directions = self.pipe_directions.get(&loc.shape).unwrap();

                    if matching_directions.contains(&Direction::Right) {
                        max_row.push('#');
                    } else {
                        max_row.push('.');
                    }
                    if matching_directions.contains(&Direction::Down) {
                        second_max_row.push('#');
                    } else {
                        second_max_row.push('.');
                    }
                    second_max_row.push('.');
                } else {
                    max_row.push('.');
                    max_row.push('.');
                    second_max_row.push('.');
                    second_max_row.push('.');
                }
            }
            max_grid.push(max_row);
            max_grid.push(second_max_row);
        }
        max_grid
    }

    fn calculate_expanded_boundaries(&self, max_grid: &Vec<Vec<char>>, y: usize, x: usize) -> bool {
        let boundary_crossings = [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ]
        .map(|direction| {
            let mut boundary_crossings = 0;
            let mut current_coord = Coordinate { x, y };
            let mut previous_char = &max_grid[current_coord.y][current_coord.x];
            while let Some(coord) = calculate_offset(&direction, &current_coord) {
                if coord.y == max_grid.len() || coord.x == max_grid[0].len() {
                    break;
                }
                current_coord = coord;

                let evaluated_loc = &max_grid[coord.y][coord.x];

                if evaluated_loc == &'#' && previous_char != &'#' {
                    boundary_crossings += 1;
                };
                previous_char = evaluated_loc;
            }
            boundary_crossings
        });

        if boundary_crossings.contains(&0) {
            false
        } else if boundary_crossings.iter().all(|crossing| crossing % 2 == 1) {
            true
        } else {
            false
        }
    }
}

fn shrink_grid(max_grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    max_grid
        .iter()
        .skip(1)
        .collect::<Vec<_>>()
        .chunks(2)
        .map(|chunk| {
            chunk[0]
                .iter()
                .skip(1)
                .collect::<Vec<_>>()
                .chunks(2)
                .map(|char_chunk| *char_chunk[0])
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn fill(grid: &mut Vec<Vec<char>>) {
    let mut queue = Vec::new();
    queue.push(Coordinate { x: 0, y: 0 });

    while queue.len() > 0 {
        if let Some(current_coord) = queue.pop() {
            for direction in [
                Direction::Up,
                Direction::Down,
                Direction::Left,
                Direction::Right,
            ] {
                if let Some(new_coord) = calculate_offset(&direction, &current_coord) {
                    if new_coord.y == grid.len() || new_coord.x == grid[0].len() {
                        continue;
                    }
                    let field = grid
                        .get_mut(new_coord.y)
                        .unwrap()
                        .get_mut(new_coord.x)
                        .unwrap();
                    if field == &'.' {
                        *field = 'O';
                        queue.push(new_coord);
                    }
                };
            }
        };
    }
}

fn calculate_offset(direction: &Direction, coord: &Coordinate) -> Option<Coordinate> {
    match direction {
        Direction::Up => coord.y.checked_sub(1).map(|y| Coordinate { x: coord.x, y }),
        Direction::Down => coord.y.checked_add(1).map(|y| Coordinate { x: coord.x, y }),
        Direction::Right => coord.x.checked_add(1).map(|x| Coordinate { x, y: coord.y }),
        Direction::Left => coord.x.checked_sub(1).map(|x| Coordinate { x, y: coord.y }),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parses_grid() {
        let input = ".....
.S-7.";
        let expected_grid: Vec<Vec<Location>> = vec![
            vec![
                Location {
                    is_pipe: false,
                    coord: Coordinate { x: 0, y: 0 },
                    shape: '.',
                    is_main_loop: Some(false),
                    distance_from_start: None,
                    is_inside: None,
                },
                Location {
                    is_pipe: false,
                    coord: Coordinate { x: 1, y: 0 },
                    shape: '.',
                    is_main_loop: Some(false),
                    distance_from_start: None,
                    is_inside: None,
                },
                Location {
                    is_pipe: false,
                    coord: Coordinate { x: 2, y: 0 },
                    shape: '.',
                    is_main_loop: Some(false),
                    distance_from_start: None,
                    is_inside: None,
                },
                Location {
                    is_pipe: false,
                    coord: Coordinate { x: 3, y: 0 },
                    shape: '.',
                    is_main_loop: Some(false),
                    distance_from_start: None,
                    is_inside: None,
                },
                Location {
                    is_pipe: false,
                    coord: Coordinate { x: 4, y: 0 },
                    shape: '.',
                    is_main_loop: Some(false),
                    distance_from_start: None,
                    is_inside: None,
                },
            ],
            vec![
                Location {
                    is_pipe: false,
                    coord: Coordinate { x: 0, y: 1 },
                    shape: '.',
                    is_main_loop: Some(false),
                    distance_from_start: None,
                    is_inside: None,
                },
                Location {
                    is_pipe: true,
                    coord: Coordinate { x: 1, y: 1 },
                    shape: 'S',
                    is_main_loop: Some(true),
                    distance_from_start: Some(0),
                    is_inside: None,
                },
                Location {
                    is_pipe: true,
                    coord: Coordinate { x: 2, y: 1 },
                    shape: '-',
                    is_main_loop: None,
                    distance_from_start: None,
                    is_inside: None,
                },
                Location {
                    is_pipe: true,
                    coord: Coordinate { x: 3, y: 1 },
                    shape: '7',
                    is_main_loop: None,
                    distance_from_start: None,
                    is_inside: None,
                },
                Location {
                    is_pipe: false,
                    coord: Coordinate { x: 4, y: 1 },
                    shape: '.',
                    is_main_loop: Some(false),
                    distance_from_start: None,
                    is_inside: None,
                },
            ],
        ];

        let grid = Grid::parse_grid(input);

        assert_eq!(grid.locations, expected_grid);
        assert_eq!(grid.start, Coordinate { x: 1, y: 1 });
    }

    #[test]
    fn finds_connected_neighbor() {
        let input = "-L|F7
7S-7|
L|7||
-L-J|
L|-JF";
        let mut grid = Grid::parse_grid(input);

        let start = &grid.start.clone();
        let neighbors = grid.mark_neighbors(start, None);
        assert_eq!(neighbors.len(), 2);

        if let Some(first_neighbor) = neighbors.get(0) {
            assert_eq!(
                grid.locations[first_neighbor.y][first_neighbor.x]
                    .is_main_loop
                    .unwrap(),
                true
            );
            assert_eq!(
                grid.locations[first_neighbor.y][first_neighbor.x]
                    .distance_from_start
                    .unwrap(),
                1
            );
            let a_neighbor = grid.mark_neighbors(first_neighbor, Some(&start));
            assert_eq!(a_neighbor.len(), 1);
        }

        if let Some(second_neighbor) = neighbors.get(1) {
            assert_eq!(
                grid.locations[second_neighbor.y][second_neighbor.x]
                    .is_main_loop
                    .unwrap(),
                true
            );
            assert_eq!(
                grid.locations[second_neighbor.y][second_neighbor.x]
                    .distance_from_start
                    .unwrap(),
                1
            );
            let b_neighbor = grid.mark_neighbors(second_neighbor, Some(&start));
            assert_eq!(b_neighbor.len(), 1);
        }
    }

    #[test]
    fn finds_maximum_steps() {
        let input = "-L|F7
7S-7|
L|7||
-L-J|
L|-JF";
        let mut grid = Grid::parse_grid(input);

        grid.calculate_loop();

        let pipe_count = grid
            .locations
            .iter()
            .map(|row| {
                row.iter()
                    .filter(|location| location.is_main_loop.is_some_and(|x| x == true))
                    .count()
            })
            .sum::<usize>();

        assert_eq!(grid.farthes_distance, Some(4));
        assert_eq!(pipe_count, 8);
    }

    #[test]
    fn finds_nests() {
        let input = "..........
.S------7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
..........";

        let mut grid = Grid::parse_grid(input);

        grid.calculate_loop();

        grid.floodfill();

        assert_eq!(grid.contained, Some(4));
    }

    #[test]
    fn finds_nests_bigger() {
        let input = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";

        let mut grid = Grid::parse_grid(input);

        grid.calculate_loop();

        grid.floodfill();

        assert_eq!(grid.contained, Some(8));
    }

    #[test]
    fn finds_different_nest() {
        let input = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";

        let mut grid = Grid::parse_grid(input);

        grid.calculate_loop();

        grid.floodfill();

        assert_eq!(grid.contained, Some(10));
    }

    #[test]
    fn flood_fills() {
        let mut grid = vec![vec!['.', '.'], vec!['.', '.']];

        fill(&mut grid);

        assert_eq!(grid, vec![vec!['O', 'O'], vec!['O', 'O']]);
    }

    #[test]
    fn boundary_crosses_count() {
        let input: HashMap<Direction, &str> = HashMap::from([
            (
                Direction::Right,
                "||||FJLJLJLJLJL-JF7F7|LJL-JF----J|L-7LJLJF7F7FJL7F7F7",
            ),
            (
                Direction::Left,
                "F7L----7F7F7||FJL----JL-7||LJ||LJ|F7F--7F--JF7L7L-7L7F7F7FJ||",
            ),
            (
                Direction::Up,
                "FL7||||||L7J-7JFLFL--7|J7JF|||JIF||L-7J7J7J--FJ--FJ7L",
            ),
            (
                Direction::Down,
                "7|||J-7J7J7L7|L7J7|JFJFLFL-FJ7||||J7J7LFJ7J7JFLF||||L7|J7J7|||L",
            ),
        ]);
        assert_eq!(Grid::str_to_boundary_crossings(input), true);
        let input: HashMap<Direction, &str> = HashMap::from([
            (Direction::Right, "L-7"),
            (Direction::Left, "FJL7L7LJLJ||LJ"),
            (Direction::Up, "-FL"),
            (Direction::Down, "FLF|J"),
        ]);
        assert_eq!(Grid::str_to_boundary_crossings(input), true);
        let input: HashMap<Direction, &str> = HashMap::from([
            (Direction::Right, "FJ||||||||L7"),
            (Direction::Left, "||"),
            (Direction::Up, "--"),
            (Direction::Down, "7J"),
        ]);
        assert_eq!(Grid::str_to_boundary_crossings(input), false);
    }
}
