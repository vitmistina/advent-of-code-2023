use super::*;

mod one_tile_growth;
mod unlimited_tiles;

pub(crate) fn shifted_rock(i: isize, size: isize) -> isize {
    (i % size + size) % size
}

#[test]
fn shifts_correctly() {
    assert_eq!(shifted_rock(0, 3), 0);
    assert_eq!(shifted_rock(3, 3), 0);
    assert_eq!(shifted_rock(-3, 3), 0);

    assert_eq!(shifted_rock(1, 3), 1);
    assert_eq!(shifted_rock(4, 3), 1);
    assert_eq!(shifted_rock(-2, 3), 1);
    assert_eq!(shifted_rock(-5, 3), 1);
}
