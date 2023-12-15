use crate::{Dish, Space};

impl Dish {
    pub fn calculate_score(&self) -> usize {
        let len = self.data.len();
        self.data
            .iter()
            .enumerate()
            .map(|(index, line)| {
                (len - index) * line.iter().filter(|space| **space == Space::Round).count()
            })
            .sum()
    }
}

#[test]
fn calculates_score() {
    let input = "OOOO.#.O..
OO..#....#
OO..O##..O
O..#.OO...
........#.
..#....#.#
..O..#.O.O
..O.......
#....###..
#....#....";
    let dish = Dish::parse(&input);
    assert_eq!(dish.calculate_score(), 136);
}
