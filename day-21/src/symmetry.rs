use super::*;

pub(super) fn get_blue_initial(blue_garden: &Garden) -> HashSet<Coordinate> {
    let blue_initial = HashSet::from([
        Coordinate { x: 0, y: 0 },
        Coordinate { x: 2, y: 0 },
        Coordinate { x: 0, y: 2 },
        Coordinate { x: 1, y: 1 },
        Coordinate {
            x: blue_garden.x_size - 2,
            y: 0,
        },
        Coordinate {
            x: blue_garden.x_size - 1,
            y: 1,
        },
        Coordinate {
            x: blue_garden.x_size - 1,
            y: blue_garden.y_size - 1,
        },
        Coordinate {
            x: blue_garden.x_size - 3,
            y: blue_garden.y_size - 1,
        },
        Coordinate {
            x: blue_garden.x_size - 1,
            y: blue_garden.y_size - 3,
        },
        Coordinate {
            x: blue_garden.x_size - 2,
            y: blue_garden.y_size - 2,
        },
        Coordinate {
            x: 1,
            y: blue_garden.y_size - 1,
        },
        Coordinate {
            x: 0,
            y: blue_garden.y_size - 2,
        },
    ]);
    blue_initial
}

pub(super) fn get_purple_initial(blue_garden: &Garden) -> HashSet<Coordinate> {
    let blue_initial = HashSet::from([
        Coordinate { x: 1, y: 0 },
        Coordinate { x: 0, y: 1 },
        Coordinate {
            x: blue_garden.x_size - 1,
            y: 0,
        },
        Coordinate {
            x: blue_garden.x_size - 2,
            y: 1,
        },
        Coordinate {
            x: blue_garden.x_size - 3,
            y: 0,
        },
        Coordinate {
            x: blue_garden.x_size - 1,
            y: 2,
        },
        Coordinate {
            x: blue_garden.x_size - 2,
            y: blue_garden.y_size - 1,
        },
        Coordinate {
            x: blue_garden.x_size - 1,
            y: blue_garden.y_size - 2,
        },
        Coordinate {
            x: 0,
            y: blue_garden.y_size - 1,
        },
        Coordinate {
            x: 0,
            y: blue_garden.y_size - 3,
        },
        Coordinate {
            x: 2,
            y: blue_garden.y_size - 1,
        },
        Coordinate {
            x: 1,
            y: blue_garden.y_size - 2,
        },
    ]);
    blue_initial
}
