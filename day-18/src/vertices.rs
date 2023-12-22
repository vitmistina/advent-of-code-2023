use super::*;

impl Direction {
    fn get_angle(&self) -> u16 {
        match self {
            Direction::Up => 0,
            Direction::Right => 90,
            Direction::Down => 180,
            Direction::Left => 270,
        }
    }

    fn get_opposite(&self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }

    fn is_horizontal(&self) -> bool {
        match self {
            Direction::Up => false,
            Direction::Down => false,
            Direction::Left => true,
            Direction::Right => true,
        }
    }
}

impl Vertices {
    pub fn execute(&mut self, current: &Command, next: &Command) {
        let current_coord = self.data.iter().last().unwrap();
        let (y_offset, x_offset) = get_offset(&current.dir);

        // add len in direction
        let mut new_coord = BigCoordinate {
            x: current_coord.x + x_offset * current.len as isize,
            y: current_coord.y + y_offset * current.len as isize,
        };

        // get new corner
        let corner = get_new_corner(&current.dir, &next.dir);
        match corner.is_horizontal() {
            true => {
                if self.horizontal != corner {
                    match corner {
                        Direction::Left => new_coord.x -= 1,
                        Direction::Right => new_coord.x += 1,
                        _ => panic!(),
                    };
                    self.horizontal = corner;
                };
            }
            false => {
                if self.vertical != corner {
                    match corner {
                        Direction::Up => new_coord.y -= 1,
                        Direction::Down => new_coord.y += 1,
                        _ => panic!(),
                    };
                };
                self.vertical = corner;
            }
        };

        self.data.push(new_coord);
    }

    pub fn new() -> Self {
        Self {
            data: vec![BigCoordinate { x: 0, y: 0 }],
            horizontal: Direction::Left,
            vertical: Direction::Up,
        }
    }
}

fn get_new_corner(current: &Direction, next: &Direction) -> Direction {
    if (current.get_angle() + 90) % 360 == next.get_angle() {
        current.clone()
    } else {
        current.get_opposite()
    }
}

fn get_offset(direction: &Direction) -> (isize, isize) {
    let (y_offset, x_offset) = match direction {
        Direction::Down => (1, 0),
        Direction::Up => (-1, 0),
        Direction::Right => (0, 1),
        Direction::Left => (0, -1),
        _ => panic!(),
    };
    (y_offset, x_offset)
}

#[test]
fn checks_clockwiseness() {
    assert_eq!(
        get_new_corner(&Direction::Up, &Direction::Left),
        Direction::Down
    );
    assert_eq!(
        get_new_corner(&Direction::Left, &Direction::Up),
        Direction::Left
    );
    assert_eq!(
        get_new_corner(&Direction::Left, &Direction::Down),
        Direction::Right
    );
}

#[test]
fn constructs_vertices() {
    let mut vert = Vertices::new();
    vert.execute(
        &Command {
            dir: Direction::Right,
            len: 4,
        },
        &Command {
            dir: Direction::Down,
            len: 1,
        },
    );
    vert.execute(
        &Command {
            dir: Direction::Down,
            len: 1,
        },
        &Command {
            dir: Direction::Left,
            len: 2,
        },
    );
    vert.execute(
        &Command {
            dir: Direction::Left,
            len: 2,
        },
        &Command {
            dir: Direction::Down,
            len: 3,
        },
    );
    vert.execute(
        &Command {
            dir: Direction::Down,
            len: 3,
        },
        &Command {
            dir: Direction::Right,
            len: 2,
        },
    );
    vert.execute(
        &Command {
            dir: Direction::Right,
            len: 2,
        },
        &Command {
            dir: Direction::Down,
            len: 1,
        },
    );
    vert.execute(
        &Command {
            dir: Direction::Down,
            len: 1,
        },
        &Command {
            dir: Direction::Left,
            len: 5,
        },
    );
    vert.execute(
        &Command {
            dir: Direction::Left,
            len: 4,
        },
        &Command {
            dir: Direction::Up,
            len: 6,
        },
    );

    assert_eq!(
        vert.data,
        vec![
            BigCoordinate { x: 0, y: 0 },
            BigCoordinate { x: 5, y: 0 },
            BigCoordinate { x: 5, y: 2 },
            BigCoordinate { x: 3, y: 2 },
            BigCoordinate { x: 3, y: 4 },
            BigCoordinate { x: 5, y: 4 },
            BigCoordinate { x: 5, y: 6 },
            BigCoordinate { x: 0, y: 6 },
        ]
    );
}
