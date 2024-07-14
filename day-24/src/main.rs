mod collisions_2_d;
mod parsing;
mod planes_3_d;

use planes_3_d::{Line3D, Plane};
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");
    let area = Area::from_str(&input, 200000000000000, 400000000000000).unwrap();
    let count = area.count_2_d_intersections(&input);
    println!("Hello, world! {} intersections found.", count);

    let mut area = Area::from_str(&input, 200000000000000, 400000000000000).unwrap();

    area.sort_by_centrality();

    let (t_0, t_1) = area.gradient_descent();

    let origin = area.find_t0_origin(t_0, t_1);

    // 903982621110895 too high
    println!("Time 0 origin: {:?}", origin.sum());

    // let origin = area.find_time_0_origin_bisections();
    // println!("Time 0 origin: {:?}", origin);
}

#[derive(Debug, PartialEq, Clone)]
struct Vector {
    x: f64,
    y: f64,
    z: f64,
}

#[derive(Debug, PartialEq)]
struct PointIntersection {
    point: Vector,
    t: f64,
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

#[derive(Debug, PartialEq, Clone)]
struct Hailstone {
    position: Vector,
    velocity: Vector,
}

fn largest_common_divisor(vectors: &[Vector]) -> f64 {
    let appended_xyz = vectors
        .iter()
        .map(|vector| vec![vector.x, vector.y, vector.z])
        .flatten()
        .collect::<Vec<f64>>();
    let mut gcd = appended_xyz[0];
    for vector in appended_xyz.iter() {
        gcd = gcd_of_two_numbers(gcd, *vector);
    }
    gcd
}

fn gcd_of_two_numbers(a: f64, b: f64) -> f64 {
    if a == 0.0 {
        return b;
    }
    gcd_of_two_numbers(b % a, a)
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

    //     #[test]
    //     fn finds_time_0_origin() {
    //         let input = "19, 13, 30 @ -2,  1, -2
    // 18, 19, 22 @ -1, -1, -2
    // 20, 25, 34 @ -2, -2, -4
    // 12, 31, 28 @ -1, -2, -1
    // 20, 19, 15 @  1, -5, -3
    // ";
    //         let area = Area::from_str(input, 7, 27).unwrap();
    //         let expected = Vector {
    //             x: 24.0,
    //             y: 13.0,
    //             z: 10.0,
    //         };

    //         assert_eq!(area.find_time_0_origin(), expected);
    //     }

    #[test]
    fn finds_time_0_origin_bisections() {
        let input = "19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3
";
        let area = Area::from_str(input, 7, 27).unwrap();
        let expected = Vector {
            x: 24.0,
            y: 13.0,
            z: 10.0,
        };

        assert_eq!(area.find_time_0_origin_bisections(), expected);
    }

    #[test]
    fn finds_largest_common_divisor() {
        let input = vec![
            Vector {
                x: 333.0,
                y: 6.0,
                z: 9.0,
            },
            Vector {
                x: 12.0,
                y: 15.0,
                z: 18.0,
            },
        ];

        let expected = 3.0;

        assert_eq!(largest_common_divisor(&input), expected);
    }
}
