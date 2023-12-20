use super::*;

impl Grid {
    pub fn parse(input: &str, min_repeat: &u8, max_repeat: &u8) -> Self {
        let layer: Vec<Vec<Node>> = input
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, char)| {
                        let heat_loss = char.to_digit(10).unwrap() as u8;
                        Node {
                            current_score: None,
                            heuristic_current_score: None,
                            heat_loss,
                            is_target: false,
                            prev_directions: Vec::new(),
                            coord: Coordinate { x, y },
                            is_visited: false,
                        }
                    })
                    .collect()
            })
            .collect();

        let (x_len, y_len) = { (layer[0].len(), layer.len()) };

        let mut data = HashMap::new();

        for direction in Direction::full_set() {
            for rep in 1..(max_repeat + 1) {
                data.insert((rep.into(), direction), layer.clone());
            }
        }

        //start
        data.get_mut(&(1, Direction::Right)).unwrap()[0][0].current_score = Some(0);

        //target in each layer
        for layer in data.values_mut() {
            layer[y_len - 1][x_len - 1].is_target = true;
        }

        Self {
            data,
            min_repeat: *min_repeat,
            max_repeat: *max_repeat,
        }
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
    let grid = Grid::parse(input, &1, &3);
    assert_eq!(grid.data.len(), 12);
    assert_eq!(grid.data[&(1, Direction::Left)].len(), 13);
    assert_eq!(grid.data[&(1, Direction::Left)][0].len(), 13);
    assert_eq!(grid.data[&(1, Direction::Left)][12][12].heat_loss, 3);
    assert_eq!(grid.data[&(1, Direction::Left)][12][12].is_target, true);
}
