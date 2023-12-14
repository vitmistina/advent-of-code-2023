use crate::{GroundType, Pattern};

pub trait Parse {
    fn parse_sections(input: &str) -> Vec<Pattern>;
}

impl Parse for Pattern {
    fn parse_sections(input: &str) -> Vec<Pattern> {
        input
            .split("\n\n")
            .map(|section| Pattern::parse_pattern(section))
            .collect()
    }
}

impl Pattern {
    fn parse_pattern(input: &str) -> Self {
        let data = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|char| match char {
                        '.' => GroundType::Ash,
                        '#' => GroundType::Rocks,
                        _ => panic!(),
                    })
                    .collect()
            })
            .collect();
        Self { data }
    }
}

#[test]
fn parse_pattern() {
    let input = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.";

    let pattern = Pattern::parse_pattern(input);
    assert_eq!(pattern.data.len(), 7);
    assert_eq!(pattern.data[0].len(), 9);
}

#[test]
fn parse_sections() {
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
    let valley = Pattern::parse_sections(input);

    assert_eq!(valley[0].data.len(), 7);
    assert_eq!(valley[0].data[0].len(), 9);
    assert_eq!(valley[1].data.len(), 7);
    assert_eq!(valley[1].data[0].len(), 9);
}
