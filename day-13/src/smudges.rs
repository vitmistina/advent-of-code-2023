use crate::{
    mirrors::Mirrors, parsing::Parse, subslice::CreatesSubslice, transpose::Transposes, Pattern,
};

pub trait FixesSmudges {
    fn fix_smudge(&self) -> (usize, usize);
}

impl FixesSmudges for Pattern {
    fn fix_smudge(&self) -> (usize, usize) {
        let initial_lines = self.find_both_lines((0, 0));

        for (row_index, row) in self.data.iter().enumerate() {
            for (col_index, _) in row.iter().enumerate() {
                let mut copy = self.clone();
                {
                    let ground = copy
                        .data
                        .get_mut(row_index)
                        .unwrap()
                        .get_mut(col_index)
                        .unwrap();
                    *ground = ground.switch();
                }

                let new_lines = copy.find_both_lines(initial_lines);
                if new_lines != (0, 0) && new_lines != initial_lines {
                    return new_lines;
                }
            }
        }
        panic!()
    }
}

impl Pattern {
    fn find_both_lines(&self, initial: (usize, usize)) -> (usize, usize) {
        let row = self.find_mirror_line(initial.0);
        let transposed = self.transpose();
        let col = transposed.find_mirror_line(initial.1);
        (row, col)
    }

    fn find_mirror_line(&self, initial: usize) -> usize {
        for index in 1..self.data.len() {
            let slice = self.create_subslice(&index);
            if slice.check_mirrorness() == true && initial != index {
                return index;
            }
        }
        0
    }
}

#[test]
fn real_example_2() {
    let input = ".##......
###.####.
##.##...#
..###..##
...##..##
#..#.##.#
..#......
.##..##..
.##..##..";

    let mut sections = Pattern::parse_sections(input);

    assert_eq!(sections[0].fix_smudge(), (0, 6));
}

#[test]
fn real_example() {
    let input = ".......####..##..
.####.##.#.#....#
.####...#.#.####.
######.##.#######
........#..#....#
.####....########
......###.###..##
#....##.###......
.####...#.###..##
#....####.##....#
........##..#####
##..##.####.####.
######..#....##..
#....#..####....#
.####.##.#.######";

    let mut sections = Pattern::parse_sections(input);

    assert_eq!(sections[0].fix_smudge(), (0, 14));

    let input = "##.####.######.##
.#.#..#.#....#.#.
...#..#...##...#.
###....########..
#..#..#..####..#.
.#.#..#.#.##.#.#.
...####...##...##
..##..##......##.
##.#..#.######.#.
..#....#....#.#..
..#....#......#..
#...##...####...#
#.######.####.###
..##..##......##.
#........####....";

    let mut sections = Pattern::parse_sections(input);

    assert_eq!(sections[0].fix_smudge(), (0, 11));
}

#[test]
fn finds_smudge() {
    let input = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
    let mut sections = Pattern::parse_sections(input);

    assert_eq!(sections[0].fix_smudge(), (3, 0));
    assert_eq!(sections[1].fix_smudge(), (1, 0));
}

#[test]
fn finds_both_lines() {
    let input = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.";
    let sections = Pattern::parse_sections(input);

    assert_eq!(sections[0].find_both_lines((0, 0)), (0, 5));
    let input = ".......####..##..
.####.##.#.#....#
.####...#.#.####.
######.##.#######
........#..#....#
.####....########
......###.###..##
#....##.###......
.####...#.###..##
#....####.##....#
........##..#####
##..##.####.####.
######..#....##..
#....#..####....#
.####.##.#.######";
    let sections = Pattern::parse_sections(input);

    assert_eq!(sections[0].find_both_lines((0, 0)), (0, 3));
}
