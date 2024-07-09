use super::*;

impl Vector {
    fn from_str(input: &str) -> Result<Self, &str> {
        let parts: Vec<&str> = input.split(',').collect();
        if parts.len() != 3 {
            return Err("Invalid input");
        }

        let x = parts[0].trim().parse::<f64>().unwrap();
        let y = parts[1].trim().parse::<f64>().unwrap();
        let z = parts[2].trim().parse::<f64>().unwrap();

        Ok(Vector { x, y, z })
    }
}

impl Hailstone {
    fn from_str(input: &str) -> Result<Self, &str> {
        let parts: Vec<&str> = input.split('@').collect();
        if parts.len() != 2 {
            return Err("Invalid input");
        }

        let position = Vector::from_str(parts[0])?;
        let velocity = Vector::from_str(parts[1])?;

        Ok(Hailstone { position, velocity })
    }
}

impl Area {
    pub fn from_str(input: &str, min: u64, max: u64) -> Result<Self, &str> {
        let mut hailstones = Vec::new();
        for line in input.lines() {
            hailstones.push(Hailstone::from_str(line).unwrap());
        }

        Ok(Area {
            min,
            max,
            hailstones,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_string_to_hailstone() {
        let input = "19, 13, 30 @ -2,  1, -2";
        let expected = Hailstone {
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

        assert_eq!(Hailstone::from_str(input), Ok(expected));
    }

    #[test]
    fn parses_lines_to_area() {
        let input = "19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
";
        let expected = Area {
            min: 7,
            max: 27,
            hailstones: vec![
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
            ],
        };

        assert_eq!(Area::from_str(input, 7, 27), Ok(expected));
    }
}
