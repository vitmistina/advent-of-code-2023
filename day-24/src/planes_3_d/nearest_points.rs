use std::io::Write;

use super::*;

impl Area {
    // find the average of min and max of area
    // compute lenght of vector from position of each vector and the average

    fn compute_centrality(&self) -> Vec<(f64, Hailstone)> {
        let average = (self.min as f64 + self.max as f64) / 2.0;
        self.hailstones
            .iter()
            .map(|hailstone| {
                let distance = hailstone.position.distance_to(&Vector {
                    x: average,
                    y: average,
                    z: average,
                });
                (distance, hailstone.clone())
            })
            .collect()
    }

    pub fn sort_by_centrality(&mut self) {
        let mut centrality = self.compute_centrality();
        centrality.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

        self.hailstones = centrality
            .into_iter()
            .map(|(_, hailstone)| hailstone)
            .collect();
    }

    fn find_average_minimum_distance(&self, line: &Hailstone) -> u64 {
        let mut total = 0;
        for hailstone in &self.hailstones {
            if let Some(distance) = hailstone.line_min_distance(&line) {
                total += distance as u64;
            };
        }
        total / self.hailstones.len() as u64
    }

    pub fn gradient_descent(&self) -> (u64, u64) {
        let mut iterations = 0;
        let mut previous_average = u64::MAX;
        let mut min_average = u64::MAX;

        let mut x = 10;
        let mut y = 10;

        // 2 to the power of 40
        let mut learning_rate = 1_073_741_824.0 * 8.0;
        // let mut learning_rate = 1.0;

        while iterations < 50_000 {
            let grad_x: f64 = self.find_average_minimum_distance(&self.gradient_function(x + 1, y))
                as f64
                - self.find_average_minimum_distance(&self.gradient_function(x, y)) as f64;
            let grad_y: f64 = self.find_average_minimum_distance(&self.gradient_function(x, y + 1))
                as f64
                - self.find_average_minimum_distance(&self.gradient_function(x, y)) as f64;

            x = (x as f64 - guarantee_step_size(learning_rate, grad_x)) as u64;

            y = (y as f64 - guarantee_step_size(learning_rate, grad_y)) as u64;

            let average = self.find_average_minimum_distance(&self.gradient_function(x, y));

            previous_average = average;

            if average == 0 {
                break;
            }

            if (average < min_average) {
                min_average = average;
            }

            if learning_rate > 0.01 && iterations % 100 == 0 {
                learning_rate /= 2.0;
            }

            iterations += 1;
        }

        return (x, y);
    }

    pub fn sample(&self) {
        let path = "output.csv";
        let mut t_0: u64 = 0;
        let mut t_1 = 0;

        let max_t_0 = 27;
        let max_t_1 = 27;

        let increment = 1;

        println!("Increment: {}", increment);

        fs::remove_file(path).unwrap_or_default();
        let mut file_buffer = fs::File::create(path).unwrap();

        while t_0 < max_t_0 {
            while t_1 < max_t_1 {
                let stone = self.gradient_function(t_0, t_1);

                let average = self.find_average_minimum_distance(&stone);

                // t_0, t_1, average
                file_buffer
                    .write_all(format!("{}, {}, {}\n", t_0, t_1, average).as_bytes())
                    .unwrap();

                t_1 += increment;
            }

            t_1 = 0;
            t_0 += increment;
        }
    }

    fn gradient_function(&self, x: u64, y: u64) -> Hailstone {
        let position_at_t_0 = self.hailstones[0]
            .position
            .add(&self.hailstones[0].velocity.mul(x as f64));

        let position_at_t_1 = self.hailstones[1]
            .position
            .add(&self.hailstones[1].velocity.mul(y as f64));

        let stone = Hailstone {
            position: position_at_t_0.clone(),
            velocity: position_at_t_1.sub(&position_at_t_0),
        };
        stone
    }

    pub fn find_t0_origin(&self, t_0: u64, t_1: u64) -> Vector {
        let (later_t, later_hailstone, earlier_t, earlier_hailstone) = if t_0 > t_1 {
            (t_0, &self.hailstones[0], t_1, &self.hailstones[1])
        } else {
            (t_1, &self.hailstones[1], t_0, &self.hailstones[0])
        };

        let later_pos = later_hailstone
            .position
            .add(&later_hailstone.velocity.mul(later_t as f64));

        let earlier_pos = earlier_hailstone
            .position
            .add(&earlier_hailstone.velocity.mul(earlier_t as f64));

        let vector = later_pos
            .sub(&earlier_pos)
            .mul(1.0 / (later_t as f64 - earlier_t as f64));

        later_pos.sub(&vector.mul(later_t as f64))
    }
}

fn guarantee_step_size(learning_rate: f64, grad: f64) -> f64 {
    if grad < 0.0 {
        return (learning_rate * grad as f64).min(-1.0);
    } else {
        return (learning_rate * grad as f64).max(1.0);
    }
}

impl Hailstone {
    fn line_min_distance(&self, line: &Hailstone) -> Option<f64> {
        let cross = self.velocity.cross_product(&line.velocity);

        if cross.x == 0.0 && cross.y == 0.0 && cross.z == 0.0 {
            // parallel lines
            return None;
        };

        return Some(cross.dot(&self.position.sub(&line.position)).abs() / cross.magnitude());
    }
}

impl Vector {
    pub fn distance_to(&self, other: &Vector) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2) + (self.z - other.z).powi(2))
            .sqrt()
    }

    pub fn cross_product(&self, other: &Vector) -> Vector {
        Vector {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn sub(&self, other: &Vector) -> Vector {
        Vector {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }

    pub fn add(&self, other: &Vector) -> Vector {
        Vector {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }

    pub fn mul(&self, scalar: f64) -> Vector {
        Vector {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }

    pub fn magnitude(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    pub fn sum(&self) -> u64 {
        self.x as u64 + self.y as u64 + self.z as u64
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn computes_centrality() {
        let area =
            Area::from_str("19, 13, 30 @ -2,  1, -2\n18, 19, 22 @ -1, -1, -2", 7, 27).unwrap();
        let expected = vec![
            (
                13.74772708486752,
                Hailstone {
                    position: Vector {
                        x: 19.0,
                        y: 13.0,
                        z: 30.0,
                    },
                    velocity: Vector {
                        x: -2.0,
                        y: 1.0,
                        z: -2.0,
                    },
                },
            ),
            (
                5.477225575051661,
                Hailstone {
                    position: Vector {
                        x: 18.0,
                        y: 19.0,
                        z: 22.0,
                    },
                    velocity: Vector {
                        x: -1.0,
                        y: -1.0,
                        z: -2.0,
                    },
                },
            ),
        ];
        assert_eq!(area.compute_centrality(), expected);
    }

    #[test]
    fn finds_0_origin() {
        let area =
            Area::from_str("19, 13, 30 @ -2,  1, -2\n18, 19, 22 @ -1, -1, -2", 7, 27).unwrap();
        let expected = Vector {
            x: 24.0,
            y: 13.0,
            z: 10.0,
        };
        assert_eq!(area.find_t0_origin(5, 3), expected);
    }

    #[test]
    fn finds_0_origin_reverse_order() {
        let area =
            Area::from_str("18, 19, 22 @ -1, -1, -2\n19, 13, 30 @ -2,  1, -2", 7, 27).unwrap();
        let expected = Vector {
            x: 24.0,
            y: 13.0,
            z: 10.0,
        };
        assert_eq!(area.find_t0_origin(3, 5), expected);
    }

    #[test]
    fn finds_average_minimum_distance_0() {
        let area =
            Area::from_str("19, 13, 30 @ -2,  1, -2\n18, 19, 22 @ -1, -1, -2", 7, 27).unwrap();

        let line = Hailstone {
            position: Vector {
                x: 24.0,
                y: 13.0,
                z: 10.0,
            },
            velocity: Vector {
                x: -3.0,
                y: 1.0,
                z: 2.0,
            },
        };

        let expected = 0;

        assert_eq!(area.find_average_minimum_distance(&line), expected);
    }

    #[test]
    fn finds_average_minimum_distance_non_zero() {
        let area =
            Area::from_str("19, 13, 30 @ -2,  1, -2\n18, 19, 22 @ -1, -1, -2", 7, 27).unwrap();

        let line = Hailstone {
            position: Vector {
                x: 24.0,
                y: 13.0,
                z: 10.0,
            },
            velocity: Vector {
                x: -3.0,
                y: 1.0,
                z: 10.0,
            },
        };

        let expected = 3;

        assert_eq!(area.find_average_minimum_distance(&line), expected);
    }
}
