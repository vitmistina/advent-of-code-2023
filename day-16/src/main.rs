mod beaming;
mod parsing;

fn main() {
    println!("Hello, world!");
}

#[derive(Debug, PartialEq)]
struct Beam {
    start_x: usize,
    start_y: usize,
    angle: u16,
}

#[derive(Debug, PartialEq)]
struct Mirror {
    angle: u16,
}

#[derive(Debug, PartialEq)]
struct Location {
    mirror: Option<Mirror>,
    is_energized: bool,
}

#[derive(Debug, PartialEq)]
struct Grid {
    data: Vec<Vec<Location>>,
}
