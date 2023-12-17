use super::*;

impl Grid {
    pub fn parse(input: &str) -> Self {
        let mut data: Vec<Vec<Node>> = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|char| {
                        let heat_loss = char.to_digit(10).unwrap() as u8;
                        Node {
                            current_score: None,
                            is_visited: false,
                            heat_loss,
                            is_target: false,
                            prev_directions: Vec::new(),
                        }
                    })
                    .collect()
            })
            .collect();

        let (x_len, y_len) = { (data[0].len(), data.len()) };

        //start
        data[0][0].current_score = Some(0);

        //target
        data[y_len - 1][x_len - 1].is_target = true;

        Self { data }
    }
}

#[test]
fn parses_grid() {
    let input = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";
    let grid = Grid::parse(input);
    assert_eq!(grid.data.len(), 13);
    assert_eq!(grid.data[0].len(), 13);
    assert_eq!(grid.data[12][12].heat_loss, 3);
    assert_eq!(grid.data[12][12].is_target, true);
}
