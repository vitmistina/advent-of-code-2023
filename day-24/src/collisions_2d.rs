use super::*;

impl Area {
    pub fn contains_2_d(&self, vector: &Vector) -> bool {
        vector.x >= self.min as f64
            && vector.x <= self.max as f64
            && vector.y >= self.min as f64
            && vector.y <= self.max as f64
    }
}

impl Hailstone {
    pub fn detect_2_d_collision(&self, other: &Hailstone) -> Option<Vector> {
        if self.velocity.x == other.velocity.x && self.velocity.y == other.velocity.y {
            return None;
        }
        let slope_self = self.velocity.y / self.velocity.x;
        let y_intercept_self = self.position.y - slope_self * self.position.x;
        let slope_other = other.velocity.y / other.velocity.x;
        let y_intercept_other = other.position.y - slope_other * other.position.x;

        let x = (y_intercept_other - y_intercept_self) / (slope_self - slope_other);
        let y = slope_self * x + y_intercept_self;

        let t_self = (x - self.position.x) / self.velocity.x;
        let t_other = (x - other.position.x) / other.velocity.x;

        if t_self < 0.0 || t_other < 0.0 {
            return None;
        }

        Some(Vector { x, y, z: 0.0 })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_collisions_within_area() {
        let a = Hailstone {
            position: Vector {
                x: 19.0,
                y: 13.0,
                z: 30.0,
            },
            velocity: Vector {
                x: -2.0,
                y: 1.0,
                z: 2.0,
            },
        };
        let b = Hailstone {
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
        };

        let area = Area::new(7, 27);
        let intercept = a.detect_2_d_collision(&b);
        assert_eq!(
            intercept,
            Some(Vector {
                x: 14.333333333333334,
                y: 15.333333333333332,
                z: 0.0
            })
        );
        assert_eq!(area.contains_2_d(&intercept.unwrap()), true);
    }

    #[test]
    fn detects_collisions_outside_area() {
        let a = Hailstone {
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
        };
        let b = Hailstone {
            position: Vector {
                x: 12.0,
                y: 31.0,
                z: 28.0,
            },
            velocity: Vector {
                x: -1.0,
                y: -2.0,
                z: -1.0,
            },
        };

        let area = Area::new(7, 27);
        let intercept = a.detect_2_d_collision(&b);
        assert_eq!(
            intercept,
            Some(Vector {
                x: 6.2,
                y: 19.4,
                z: 0.0
            })
        );
        assert_eq!(area.contains_2_d(&intercept.unwrap()), false);
    }

    #[test]
    fn detects_parallel_paths() {
        let a = Hailstone {
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
        };
        let b = Hailstone {
            position: Vector {
                x: 20.0,
                y: 25.0,
                z: 34.0,
            },
            velocity: Vector {
                x: -2.0,
                y: -2.0,
                z: -4.0,
            },
        };

        let area = Area::new(7, 27);
        let intercept = a.detect_2_d_collision(&b);
        assert_eq!(intercept, None);
    }

    #[test]
    fn detects_past_collisions() {
        let a = Hailstone {
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
        };
        let b = Hailstone {
            position: Vector {
                x: 20.0,
                y: 19.0,
                z: 15.0,
            },
            velocity: Vector {
                x: 1.0,
                y: -5.0,
                z: -3.0,
            },
        };

        let intercept = a.detect_2_d_collision(&b);
        assert_eq!(intercept, None);
    }

    #[test]
    fn detects_past_collisions_for_both() {
        let a = Hailstone {
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
        };
        let b = Hailstone {
            position: Vector {
                x: 20.0,
                y: 19.0,
                z: 15.0,
            },
            velocity: Vector {
                x: 1.0,
                y: -5.0,
                z: -3.0,
            },
        };

        let intercept = a.detect_2_d_collision(&b);
        assert_eq!(intercept, None);
    }
}
