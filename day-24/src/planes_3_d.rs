use super::*;

mod nearest_points;

#[derive(Debug, PartialEq)]
pub struct Line3D {
    point: Vector,
    direction: Vector,
}

impl Line3D {
    pub fn contains(&self, point: &Vector) -> bool {
        let x = (point.x - self.point.x) / self.direction.x;
        let y = (point.y - self.point.y) / self.direction.y;
        let z = (point.z - self.point.z) / self.direction.z;

        x == y && y == z
    }

    pub fn from_vectors(point: &Vector, direction: &Vector) -> Line3D {
        Line3D {
            point: point.clone(),
            direction: direction.clone(),
        }
    }
}

impl PointIntersection {
    pub fn find_t0_origin(&self, other: &PointIntersection) -> Vector {
        let time_unit_vector = Vector {
            x: (other.point.x - self.point.x) / (other.t - self.t),
            y: (other.point.y - self.point.y) / (other.t - self.t),
            z: (other.point.z - self.point.z) / (other.t - self.t),
        };

        Vector {
            x: self.point.x - self.t * time_unit_vector.x,
            y: self.point.y - self.t * time_unit_vector.y,
            z: self.point.z - self.t * time_unit_vector.z,
        }
    }
}

impl Vector {
    fn dot(&self, other: &Vector) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

impl Hailstone {
    fn time_to_reach(&self, point: &Vector) -> f64 {
        (point.x - self.position.x) / self.velocity.x
    }
}

#[derive(Debug, PartialEq)]
pub struct Plane {
    normal: Vector,
    distance: f64,
}

impl Plane {
    pub fn intersect_with_hailstone(
        &self,
        hailstone: &Hailstone,
        time_factor: f64,
    ) -> Option<PointIntersection> {
        let scaled_velocity = Vector {
            x: hailstone.velocity.x * time_factor,
            y: hailstone.velocity.y * time_factor,
            z: hailstone.velocity.z * time_factor,
        };
        let denominator = self.normal.dot(&scaled_velocity);

        if denominator == 0.0 {
            // log line
            println!("Hailstone is parallel to the plane");
            return None;
        }

        let t = -(self.normal.x * hailstone.position.x
            + self.normal.y * hailstone.position.y
            + self.normal.z * hailstone.position.z
            + self.distance)
            / denominator;

        if t < 0.0 {
            return None;
        }

        Some(PointIntersection {
            point: Vector {
                x: hailstone.position.x + t * scaled_velocity.x,
                y: hailstone.position.y + t * scaled_velocity.y,
                z: hailstone.position.z + t * scaled_velocity.z,
            },
            t,
        })
    }

    pub fn from_three_points(a: &Vector, b: &Vector, c: &Vector) -> Plane {
        let ab = Vector {
            x: b.x - a.x,
            y: b.y - a.y,
            z: b.z - a.z,
        };
        let ac = Vector {
            x: c.x - a.x,
            y: c.y - a.y,
            z: c.z - a.z,
        };

        let normal = Vector {
            x: ab.y * ac.z - ab.z * ac.y,
            y: ab.z * ac.x - ab.x * ac.z,
            z: ab.x * ac.y - ab.y * ac.x,
        };

        let distance = -(normal.x * a.x + normal.y * a.y + normal.z * a.z);

        Plane { normal, distance }
    }

    fn from_hailstones_at_time(a: &Hailstone, b: &Hailstone, t: f64) -> Plane {
        Plane::from_three_points(
            &a.position,
            &Vector {
                x: a.position.x + a.velocity.x * 10000000.0,
                y: a.position.y + a.velocity.y * 10000000.0,
                z: a.position.z + a.velocity.z * 10000000.0,
            },
            &Vector {
                x: b.position.x + b.velocity.x * t,
                y: b.position.y + b.velocity.y * t,
                z: b.position.z + b.velocity.z * t,
            },
        )
    }

    fn intersection(&self, other: &Plane) -> Option<Line3D> {
        let direction = Vector {
            x: self.normal.y * other.normal.z - self.normal.z * other.normal.y,
            y: self.normal.z * other.normal.x - self.normal.x * other.normal.z,
            z: self.normal.x * other.normal.y - self.normal.y * other.normal.x,
        };

        let denominator =
            direction.x * direction.x + direction.y * direction.y + direction.z * direction.z;
        if denominator == 0.0 {
            return None;
        }

        let point = Vector {
            x: (self.normal.y * other.distance - self.distance * other.normal.y) / denominator,
            y: (self.distance * other.normal.x - self.normal.x * other.distance) / denominator,
            z: (self.normal.x * other.normal.y - self.normal.y * other.normal.x) / denominator,
        };

        Some(Line3D { point, direction })
    }
}

#[derive(Debug, PartialEq)]
struct ConstrainedHailstone {
    hailstone: Hailstone,
    min_t: f64,
    max_t: f64,
}

impl ConstrainedHailstone {
    fn new(hailstone: &Hailstone) -> ConstrainedHailstone {
        ConstrainedHailstone {
            hailstone: hailstone.clone(),
            min_t: 0.0,
            max_t: f64::INFINITY,
        }
    }
}

impl Area {
    pub fn find_time_0_origin(
        &self,
        contrains: &ConstrainedHailstone,
        n: usize,
        m: usize,
    ) -> Vector {
        let hailstone_1 = self.hailstones.get(m).unwrap();
        let hailstone_2 = self.hailstones.get(n).unwrap();
        // remove nth and mth element from hailstones
        let subset_of_hailstones = self
            .hailstones
            .iter()
            .enumerate()
            .filter(|(i, _)| *i != n && *i != m)
            .map(|(_, hailstone)| hailstone)
            .collect::<Vec<&Hailstone>>();

        println!("min_t millions: {}", contrains.min_t / 1_000_000.0);
        println!("max_t millions: {}", contrains.max_t / 1_000_000.0);
        let diff = contrains.max_t - contrains.min_t;
        println!("diff millions: {}", diff / 1_000_000.0);
        let mut t = contrains.max_t.floor() + 1.0;
        loop {
            if (contrains.max_t.floor() - t) % 1_000.0 == 0.0 {
                println!("t: {}", t);
            }
            let hailstone_2_position_t = Vector {
                x: hailstone_2.position.x + t * hailstone_2.velocity.x,
                y: hailstone_2.position.y + t * hailstone_2.velocity.y,
                z: hailstone_2.position.z + t * hailstone_2.velocity.z,
            };

            let plane = Plane::from_three_points(
                &hailstone_1.position,
                &Vector {
                    x: hailstone_1.position.x + t * hailstone_1.velocity.x,
                    y: hailstone_1.position.y + t * hailstone_1.velocity.y,
                    z: hailstone_1.position.z + t * hailstone_1.velocity.z,
                },
                &hailstone_2_position_t,
            );
            let mut is_line_found = true;

            let mut line: Option<Line3D> = None;
            let mut found_intersection: Option<PointIntersection> = None;
            for hailstone in &subset_of_hailstones {
                if let Some(intersection) = plane.intersect_with_hailstone(hailstone, 1.0) {
                    if intersection.t % 1.0 != 0.0 {
                        is_line_found = false;
                        break;
                    }

                    println!("intersection: {:?}", intersection.t);
                    found_intersection = Some(intersection);
                }
            }
            if is_line_found {
                return found_intersection
                    .unwrap()
                    .find_t0_origin(&PointIntersection {
                        point: hailstone_2_position_t,
                        t,
                    });
            }
            t -= 1.0;
            assert!(t > 0.0);
        }
    }

    fn find_constrains_of_nth_hailstone_based_on_m(
        &self,
        n: usize,
        m: usize,
    ) -> ConstrainedHailstone {
        let mut constrained_hailstone_n =
            ConstrainedHailstone::new(self.hailstones.get(n).unwrap());
        let hailstone_m = self.hailstones.get(m).unwrap();

        // remove nth and mth element from hailstones
        let subset_of_hailstones = self
            .hailstones
            .iter()
            .enumerate()
            .filter(|(i, _)| *i != n && *i != m)
            .map(|(_, hailstone)| hailstone)
            .collect::<Vec<&Hailstone>>();

        for current_hailstone in subset_of_hailstones {
            let times = vec![0.0, self.max as f64]
                .iter()
                .map(|t| {
                    let plane = Plane::from_hailstones_at_time(hailstone_m, current_hailstone, *t);
                    if let Some(intersection) =
                        plane.intersect_with_hailstone(&constrained_hailstone_n.hailstone, 1.0)
                    {
                        // let t = constrained_hailstone_n
                        //     .hailstone
                        //     .time_to_reach(&intersection.point);
                        return Some(intersection.t);
                    } else {
                        return None;
                    }
                })
                .collect::<Vec<Option<f64>>>();

            if let Some(t_a) = times.get(0).unwrap() {
                if let Some(t_b) = times.get(1).unwrap() {
                    // if t_a > t_b
                    //     && t_a < &constrained_hailstone_n.max_t
                    //     && t_a > &constrained_hailstone_n.min_t
                    // {
                    //     assert!(t_a >= &constrained_hailstone_n.min_t);
                    //     constrained_hailstone_n.max_t = *t_a;
                    // } else if t_a < t_b
                    //     && t_a > &constrained_hailstone_n.min_t
                    //     && t_a < &constrained_hailstone_n.max_t
                    // {
                    //     assert!(t_a <= &constrained_hailstone_n.max_t);
                    //     constrained_hailstone_n.min_t = *t_a;
                    // } else if t_a == t_b {
                    //     constrained_hailstone_n.min_t = *t_a;
                    //     constrained_hailstone_n.max_t = *t_a;
                    // }
                    if t_a > t_b {
                        constrained_hailstone_n.max_t = *t_a;
                    }
                }
            }
        }

        constrained_hailstone_n
    }

    pub fn find_time_0_origin_bisections(&self) -> Vector {
        // create a map of combinations of constrained hailstones based on n and m provided by iterating over lenght of hailstone list
        let mut min_diff = f64::INFINITY;
        let mut result: Vec<(ConstrainedHailstone, usize, usize)> = Vec::new();

        'outer: for n in 0..self.hailstones.len() {
            for m in 0..self.hailstones.len() {
                if n == m {
                    continue;
                }

                let constrained_hailstone = self.find_constrains_of_nth_hailstone_based_on_m(n, m);

                if constrained_hailstone.min_t == constrained_hailstone.max_t {
                    min_diff = 0.0;
                    result.push((constrained_hailstone, n, m));
                    break 'outer;
                }

                let diff = (constrained_hailstone.max_t - constrained_hailstone.min_t).abs();
                if diff <= min_diff {
                    min_diff = diff;
                    result.push((constrained_hailstone, n, m));
                }
            }
            println!("n: {}", n);
        }

        let (contrains, n, m) = result.last().unwrap();

        self.find_time_0_origin(contrains, *n, *m)
    }
}

#[cfg(test)]
mod tests {
    use std::f64::EPSILON;

    use super::*;

    #[test]
    fn finds_future_intersect_of_hailstone_with_plane() {
        let plane = Plane {
            normal: Vector {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
            distance: 0.0,
        };

        let hailstone_with_future_intersection = Hailstone {
            position: Vector {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
            velocity: Vector {
                x: 0.0,
                y: 0.0,
                z: -1.0,
            },
        };

        let hailstone_with_past_intersection = Hailstone {
            position: Vector {
                x: 0.0,
                y: 0.0,
                z: -1.0,
            },
            velocity: Vector {
                x: 0.0,
                y: 0.0,
                z: -1.0,
            },
        };

        let expected_intersect = PointIntersection {
            point: Vector {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            t: 1.0,
        };

        assert_eq!(
            plane.intersect_with_hailstone(&hailstone_with_future_intersection, 1.0),
            Some(expected_intersect)
        );

        assert_eq!(
            plane.intersect_with_hailstone(&hailstone_with_past_intersection, 1.0),
            None
        );
    }

    #[test]
    fn validates_point_on_a_line() {
        let line = Line3D {
            point: Vector {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            direction: Vector {
                x: 1.0,
                y: 2.0,
                z: 3.0,
            },
        };

        let point_on_line = Vector {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };

        let point_on_line_past = Vector {
            x: -1.0,
            y: -2.0,
            z: -3.0,
        };

        let point_off_line = Vector {
            x: 2.0,
            y: 4.0,
            z: EPSILON,
        };

        assert_eq!(line.contains(&point_on_line), true);
        assert_eq!(line.contains(&point_on_line_past), true);
        assert_eq!(line.contains(&point_off_line), false);
    }

    #[test]
    fn computes_time_at_which_hailstone_reaches_a_point() {
        let hailstone = Hailstone {
            position: Vector {
                x: -1.0,
                y: 0.0,
                z: 0.0,
            },
            velocity: Vector {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            },
        };

        let point = Vector {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        };

        let expected_time = 2.0;

        assert_eq!(hailstone.time_to_reach(&point), expected_time);
    }
}
