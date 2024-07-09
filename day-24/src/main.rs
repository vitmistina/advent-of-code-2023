mod collisions_2d;
mod parsing;

use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");
    let area = Area::from_str(&input, 200000000000000, 400000000000000).unwrap();
    let count = area.count_2_d_intersections(&input);
    println!("Hello, world! {} intersections found.", count);
}

#[derive(Debug, PartialEq)]
struct Vector {
    x: f64,
    y: f64,
    z: f64,
}

#[derive(Debug, PartialEq)]
struct Area {
    min: u64,
    max: u64,
    hailstones: Vec<Hailstone>,
}

impl Area {
    fn new(min: u64, max: u64) -> Area {
        Area {
            min,
            max,
            hailstones: Vec::new(),
        }
    }

    fn count_2_d_intersections(&self, input: &str) -> u64 {
        self.hailstones
            .iter()
            .enumerate()
            .map(|(i, hailstone)| {
                self.hailstones
                    .iter()
                    .skip(i + 1)
                    .filter(|other| {
                        if let Some(collision) = hailstone.detect_2_d_collision(other) {
                            return self.contains_2_d(&collision);
                        } else {
                            return false;
                        }
                    })
                    .count() as u64
            })
            .sum()
    }
}

#[derive(Debug, PartialEq)]
struct Hailstone {
    position: Vector,
    velocity: Vector,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_path_intersections() {
        let input = "19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3
";
        let area = Area::from_str(input, 7, 27).unwrap();
        let expected = 2;

        assert_eq!(area.count_2_d_intersections(input), expected);
    }
}
