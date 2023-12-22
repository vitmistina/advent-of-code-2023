use super::*;

impl Direction {
    fn parse(input: &str) -> Self {
        match input {
            "U" => Self::Up,
            "D" => Self::Down,
            "L" => Self::Left,
            "R" => Self::Right,
            _ => panic!("{input}"),
        }
    }
    fn parse_char(input: &char) -> Self {
        match input {
            '3' => Self::Up,
            '1' => Self::Down,
            '2' => Self::Left,
            '0' => Self::Right,
            _ => panic!("{input}"),
        }
    }
}

impl Command {
    pub fn parse(input: &str) -> Self {
        let mut parts = input.split_whitespace();
        Self {
            dir: Direction::parse(parts.next().unwrap()),
            len: parts.next().unwrap().parse().unwrap(),
        }
    }

    pub fn parse_hex(input: &str) -> Self {
        let mut parts = input.split_whitespace();
        let hex_str = parts.nth(2).unwrap();
        let hex_digit = &hex_str[2..7];
        let len = usize::from_str_radix(hex_digit, 16).unwrap();
        let ch = &hex_str.chars().nth(7).unwrap();
        let dir = Direction::parse_char(ch);
        Self { dir, len }
    }
}

#[test]
fn parses_command() {
    let input = "R 6 (#70c710)";
    assert_eq!(
        Command::parse(input),
        Command {
            dir: Direction::Right,
            len: 6
        }
    );
}

#[test]
fn parses_hex_command() {
    let input = "R 6 (#70c710)";
    assert_eq!(
        Command::parse_hex(input),
        Command {
            dir: Direction::Right,
            len: 461937
        }
    );
}

impl Grid {
    pub fn new() -> Self {
        let grid_size = 1000;
        let current_coord = Coordinate {
            x: grid_size / 2,
            y: grid_size / 2,
        };
        let data = (0..grid_size)
            .map(|y| {
                (0..grid_size)
                    .map(|x| Node {
                        is_dug: if y == current_coord.y && x == current_coord.x {
                            true
                        } else {
                            false
                        },
                    })
                    .collect()
            })
            .collect();
        Self {
            data,
            current_coord,
        }
    }
}
