use super::*;

impl Garden {
    pub fn parse(input: &str) -> Self {
        let mut rocks = HashSet::new();
        let mut steps = HashSet::new();

        for (y, line) in input.lines().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                match ch {
                    '#' => rocks.insert(Coordinate {
                        x: x as isize,
                        y: y as isize,
                    }),
                    'S' => steps.insert(Coordinate {
                        x: x as isize,
                        y: y as isize,
                    }),
                    _ => false,
                };
            }
        }

        let y_size = input.lines().count() as isize;
        let x_size = input.lines().nth(0).unwrap().chars().count() as isize;

        Self {
            rocks,
            steps,
            y_size,
            x_size,
            tiles: HashMap::new(),
        }
    }
}

#[cfg(test)]
mod t {
    use super::*;

    #[test]
    fn parses() {
        let input = ".#
.S";
        let garden = Garden::parse(input);

        assert_eq!(
            garden,
            Garden {
                rocks: HashSet::from([Coordinate { x: 1, y: 0 }]),
                steps: HashSet::from([Coordinate { x: 1, y: 1 }]),
                y_size: 2,
                x_size: 2,
                tiles: HashMap::new()
            }
        );
    }
}
