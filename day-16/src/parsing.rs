use crate::{Grid, Location, Mirror};

impl Grid {
    pub fn from(input: &str) -> Self {
        let data = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|char| match char {
                        '.' => Location {
                            mirror: None,
                            is_energized: false,
                        },
                        '|' => Location {
                            mirror: Some(Mirror { angle: 0 }),
                            is_energized: false,
                        },
                        '-' => Location {
                            mirror: Some(Mirror { angle: 90 }),
                            is_energized: false,
                        },
                        '/' => Location {
                            mirror: Some(Mirror { angle: 45 }),
                            is_energized: false,
                        },
                        '\\' => Location {
                            mirror: Some(Mirror { angle: 135 }),
                            is_energized: false,
                        },
                        _ => panic!("{char}"),
                    })
                    .collect()
            })
            .collect();
        Self { data }
    }
}

#[test]
fn parses_grid() {
    let input = ".|./.\\
|.-.\\.";

    assert_eq!(
        Grid::from(input),
        Grid {
            data: vec![
                vec![
                    Location {
                        mirror: None,
                        is_energized: false
                    },
                    Location {
                        mirror: Some(crate::Mirror { angle: 0 }),
                        is_energized: false
                    },
                    Location {
                        mirror: None,
                        is_energized: false
                    },
                    Location {
                        mirror: Some(crate::Mirror { angle: 45 }),
                        is_energized: false
                    },
                    Location {
                        mirror: None,
                        is_energized: false
                    },
                    Location {
                        mirror: Some(crate::Mirror { angle: 135 }),
                        is_energized: false
                    }
                ],
                vec![
                    Location {
                        mirror: Some(crate::Mirror { angle: 0 }),
                        is_energized: false
                    },
                    Location {
                        mirror: None,
                        is_energized: false
                    },
                    Location {
                        mirror: Some(crate::Mirror { angle: 90 }),
                        is_energized: false
                    },
                    Location {
                        mirror: None,
                        is_energized: false
                    },
                    Location {
                        mirror: Some(crate::Mirror { angle: 135 }),
                        is_energized: false
                    },
                    Location {
                        mirror: None,
                        is_energized: false
                    }
                ]
            ]
        }
    )
}
