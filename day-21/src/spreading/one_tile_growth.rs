use super::*;

impl Coordinate {
    fn get_neighbors(&self, y_size: isize, x_size: isize) -> Vec<Coordinate> {
        let offsets = [(-1, 0), (1, 0), (0, -1), (0, 1)];

        offsets
            .iter()
            .filter_map(|off: &(i32, i32)| {
                let (y, x) = {
                    (
                        get_coords(self.y as usize, off.0, y_size as usize),
                        get_coords(self.x as usize, off.1, x_size as usize),
                    )
                };
                match (y, x) {
                    (Some(y), Some(x)) => Some(Coordinate {
                        y: y as isize,
                        x: x as isize,
                    }),
                    (_, _) => None,
                }
            })
            .collect()
    }
}

fn get_coords(index: usize, offset: i32, len: usize) -> Option<usize> {
    let adjusted_index = if offset.is_negative() {
        index.checked_sub(offset.wrapping_abs() as usize)
    } else {
        index.checked_add(offset as usize)
    };
    match adjusted_index {
        Some(new_index) if new_index < len => Some(new_index),
        _ => None,
    }
}

impl Garden {
    pub fn spread(&mut self) {
        let mut new_steps = HashSet::new();
        for step in &self.steps {
            let neighbors = step.get_neighbors(self.y_size, self.x_size);
            for n in neighbors {
                if self.rocks.contains(&n) == false {
                    new_steps.insert(n);
                }
            }
        }

        self.steps = new_steps;
    }
}
#[cfg(test)]
mod t {
    use super::*;

    #[test]
    fn get_coords() {
        let input = Coordinate { x: 1, y: 0 };

        let result: Vec<Coordinate> = input.get_neighbors(3, 3);

        assert_eq!(result.len(), 3);
        assert_eq!(result[0], Coordinate { x: 1, y: 1 });
        assert_eq!(result[1], Coordinate { x: 0, y: 0 });
        assert_eq!(result[2], Coordinate { x: 2, y: 0 });
    }

    #[test]
    fn spreads() {
        let mut garden = Garden {
            rocks: HashSet::from([Coordinate { x: 1, y: 0 }]),
            steps: HashSet::from([Coordinate { x: 1, y: 1 }]),
            y_size: 3,
            x_size: 3,
            tiles: HashMap::new(),
        };
        garden.spread();

        assert_eq!(
            garden.steps,
            HashSet::from([
                Coordinate { x: 0, y: 1 },
                Coordinate { x: 2, y: 1 },
                Coordinate { x: 1, y: 2 }
            ])
        );
    }
}
