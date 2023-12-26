use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn main() {
    let data = fs::read_to_string("data.txt").unwrap();
    let mut gearbox = Gearbox {
        numbers: vec![],
        parts: HashMap::new(),
    };

    gearbox.parse(&data);
    gearbox.evaluate_adjacency();

    let sum_of_adjacent_parts = gearbox.sum_adjacent();

    println!("Sum of adjacent: {sum_of_adjacent_parts}");

    let sum_of_gears = gearbox.sum_gears();

    println!("Sum of gears: {sum_of_gears}");
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
struct Coordinate {
    x: i16,
    y: i16,
}

#[derive(Debug, PartialEq, Clone)]
struct Number {
    coordinates: Vec<Coordinate>,
    value: u16,
    is_adjacent: Option<bool>,
}

#[derive(PartialEq, Debug)]
struct Part {
    symbol: String,
    adjacent: Vec<u32>,
}

impl Number {
    fn evaluate_adjacency(&mut self, parts: &mut HashMap<Coordinate, Part>) {
        for coordinate in &self.coordinates {
            for x in -1..2 {
                for y in -1..2 {
                    if x == 0 && y == 0 {
                        continue;
                    }
                    let lookup_coord = Coordinate {
                        x: coordinate.x + x,
                        y: coordinate.y + y,
                    };

                    match parts.get_mut(&lookup_coord) {
                        Some(part) => {
                            self.is_adjacent = Some(true);
                            part.adjacent.push(self.value as u32);
                        }
                        None => (),
                    };
                }
            }
        }

        if self.is_adjacent == None {
            self.is_adjacent = Some(false);
        }
    }

    fn new() -> Self {
        Number {
            coordinates: vec![],
            value: 0,
            is_adjacent: None,
        }
    }
}

struct Gearbox {
    numbers: Vec<Number>,
    parts: HashMap<Coordinate, Part>,
}

impl Gearbox {
    fn parse(&mut self, data: &str) {
        for (y, line) in data.lines().enumerate() {
            let mut buffer = String::new();
            let mut current_number = Number::new();
            for (x, ch) in line.chars().enumerate() {
                if ch.is_digit(10) {
                    buffer.push(ch);
                    current_number.coordinates.push(Coordinate {
                        x: x as i16,
                        y: y as i16,
                    });

                    if x == line.len() - 1 {
                        self.process_buffer(&mut buffer, &mut current_number);
                    }
                    continue;
                }

                self.process_buffer(&mut buffer, &mut current_number);

                if ch == '.' {
                    continue;
                }

                self.parts.insert(
                    Coordinate {
                        x: x as i16,
                        y: y as i16,
                    },
                    Part {
                        symbol: ch.to_string(),
                        adjacent: Vec::new(),
                    },
                );
            }
        }
    }

    fn process_buffer(&mut self, buffer: &mut String, current_number: &mut Number) {
        if buffer.len() > 0 {
            current_number.value = buffer.parse::<u16>().unwrap();
            buffer.clear();
            self.numbers.push(current_number.clone());
            *current_number = Number::new();
        }
    }

    fn evaluate_adjacency(&mut self) {
        for number in &mut self.numbers {
            number.evaluate_adjacency(&mut self.parts);
        }
    }

    fn sum_adjacent(&self) -> u32 {
        self.numbers.iter().fold(0u32, |acc, number| {
            if number.is_adjacent.unwrap() {
                acc + number.value as u32
            } else {
                acc
            }
        })
    }

    fn sum_gears(&self) -> u32 {
        self.parts
            .values()
            .map(|part| {
                let set: HashSet<u32> = HashSet::from_iter(part.adjacent.iter().cloned());
                if part.symbol == "*" && set.len() == 2 {
                    set.iter().product::<u32>() as u32
                } else {
                    0
                }
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn integration_test_part_numbers() {
        let data = "
..224.....487...................718.....................378............................................284........310......313..........311.
....*..............................*744....486*485......*......741......@...359.#666...439................*925....*......$..+........@515
.235................758..440...........................251....*......262.....*..........*......................752......774..............
.........705%..@746........+..942*591.347.470...#..257.........637...........793.......299..../.....813....509......464......&.........688..
.....82................................*.../..901.....*..................836.....&............814...*........*..............80...17*....*...";
        let mut gearbox = Gearbox {
            numbers: vec![],
            parts: HashMap::new(),
        };

        gearbox.parse(&data);
        gearbox.evaluate_adjacency();

        let sum_of_adjacent_parts = gearbox.sum_adjacent();

        assert_eq!(sum_of_adjacent_parts, 19910);
    }

    #[test]
    fn integration_gears() {
        let data = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        let mut gearbox = Gearbox {
            numbers: vec![],
            parts: HashMap::new(),
        };

        gearbox.parse(&data);
        gearbox.evaluate_adjacency();

        let sum_gears = gearbox.sum_gears();

        assert_eq!(sum_gears, 467835);
    }

    #[test]
    fn parses_correctly() {
        let data = "467*114...";

        let mut gearbox = Gearbox {
            numbers: vec![],
            parts: HashMap::new(),
        };

        gearbox.parse(data);

        assert_eq!(gearbox.numbers.len(), 2);
        assert_eq!(
            *gearbox.numbers.get(0).unwrap(),
            Number {
                coordinates: vec![
                    Coordinate { x: 0, y: 0 },
                    Coordinate { x: 1, y: 0 },
                    Coordinate { x: 2, y: 0 },
                ],
                value: 467,
                is_adjacent: None,
            }
        );
        assert_eq!(
            *gearbox.numbers.get(1).unwrap(),
            Number {
                coordinates: vec![
                    Coordinate { x: 4, y: 0 },
                    Coordinate { x: 5, y: 0 },
                    Coordinate { x: 6, y: 0 },
                ],
                value: 114,
                is_adjacent: None,
            }
        );

        assert_eq!(
            gearbox.parts,
            HashMap::from([(
                Coordinate { x: 3, y: 0 },
                Part {
                    symbol: "*".to_string(),
                    adjacent: Vec::new()
                }
            )])
        )
    }

    #[test]
    fn evaluates_adjacency() {
        let mut adjacent_number = Number {
            coordinates: vec![
                Coordinate { x: 0, y: 0 },
                Coordinate { x: 1, y: 0 },
                Coordinate { x: 2, y: 0 },
            ],
            value: 467,
            is_adjacent: None,
        };

        let mut non_adjacent_number = Number {
            coordinates: vec![
                Coordinate { x: 5, y: 0 },
                Coordinate { x: 6, y: 0 },
                Coordinate { x: 7, y: 0 },
            ],
            value: 114,
            is_adjacent: None,
        };

        let mut parts = HashMap::from([(
            Coordinate { x: 3, y: 1 },
            Part {
                symbol: "*".to_string(),
                adjacent: Vec::new(),
            },
        )]);

        adjacent_number.evaluate_adjacency(&mut parts);
        non_adjacent_number.evaluate_adjacency(&mut parts);

        assert_eq!(adjacent_number.is_adjacent.unwrap(), true);
        assert_eq!(non_adjacent_number.is_adjacent.unwrap(), false);
    }
}
