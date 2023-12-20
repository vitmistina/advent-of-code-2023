use super::*;

impl Direction {
    pub fn opposite(&self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }
    pub fn full_set() -> HashSet<Self> {
        HashSet::from([
            Direction::Right,
            Direction::Down,
            Direction::Left,
            Direction::Up,
        ])
    }

    pub fn n_full_sets(steps: &u8) -> HashMap<u8, HashSet<Self>> {
        let mut map = HashMap::new();
        for i in 1..*steps + 1 {
            map.insert(
                i,
                HashSet::from([
                    Direction::Right,
                    Direction::Down,
                    Direction::Left,
                    Direction::Up,
                ]),
            );
        }
        map
    }
}

impl Node {
    pub fn sort_by_score(nodes: &mut Vec<Node>) {
        nodes.sort_by(|b, a| a.heuristic_current_score.cmp(&b.heuristic_current_score));
    }

    pub fn find_directions(&self, max_in_direction: &u8, min_in_direction: &u8) -> Vec<Direction> {
        let mut direction = vec![
            Direction::Right,
            Direction::Down,
            Direction::Left,
            Direction::Up,
        ];

        if let Some(last_dir) = self.prev_directions.iter().last() {
            let get_dir_count = get_dir_count(
                &self.prev_directions,
                last_dir,
                &(*max_in_direction as usize),
            );

            // cant turn 180 degrees
            if let Some(index) = direction.iter().position(|dir| dir == &last_dir.opposite()) {
                direction.remove(index);
            };

            // if self.prev_directions.len() >= *max_in_direction {
            //     let mut slice = self.prev_directions.iter().rev().take(*max_in_direction);
            //     if slice.all(|dir| dir == last_dir) {
            //         if let Some(index) = direction.iter().position(|dir| dir == last_dir) {
            //             direction.remove(index);
            //         };
            //     }
            // }

            // if at max, need to remove current direction
            if get_dir_count >= *max_in_direction {
                if let Some(index) = direction.iter().position(|dir| dir == last_dir) {
                    direction.remove(index);
                };
            }

            // if less than min, need to remove current all but current direction
            if get_dir_count < *min_in_direction {
                direction = vec![*last_dir];
            }
        };

        direction
    }
}

#[test]
fn finds_valid_directions() {
    let node = Node {
        current_score: Some(0),
        heuristic_current_score: Some(0),
        heat_loss: 1,
        is_target: false,
        prev_directions: vec![],
        coord: Coordinate { x: 0, y: 0 },
        is_visited: false,
    };
    assert_eq!(
        node.find_directions(&3, &1),
        Vec::from([
            Direction::Right,
            Direction::Down,
            Direction::Left,
            Direction::Up,
        ])
    );

    let node = Node {
        current_score: Some(0),
        heuristic_current_score: Some(0),
        heat_loss: 1,
        is_target: false,
        prev_directions: vec![Direction::Right],
        coord: Coordinate { x: 0, y: 0 },
        is_visited: false,
    };
    assert_eq!(
        node.find_directions(&3, &1),
        Vec::from([Direction::Right, Direction::Down, Direction::Up,])
    );

    let node = Node {
        current_score: Some(0),
        heuristic_current_score: Some(0),
        heat_loss: 1,
        is_target: false,
        prev_directions: vec![Direction::Right, Direction::Right, Direction::Right],
        coord: Coordinate { x: 0, y: 0 },
        is_visited: false,
    };
    assert_eq!(
        node.find_directions(&3, &1),
        Vec::from([Direction::Down, Direction::Up,])
    );
}

pub fn get_dir_count(
    prev_directions: &Vec<Direction>,
    direction: &Direction,
    max_repeat: &usize,
) -> u8 {
    let mut iter = prev_directions
        .iter()
        .rev()
        .take_while(|dir| *dir == direction);
    let mut current = 0;
    for i in 1..*max_repeat + 1 {
        if let Some(dir) = iter.next() {
            current = i as u8;
        }
    }
    current
}

#[test]
fn get_right_dir_counts() {
    assert_eq!(get_dir_count(&vec![Direction::Up], &Direction::Up, &3), 1);
    assert_eq!(
        get_dir_count(
            &vec![Direction::Up, Direction::Up, Direction::Up, Direction::Up],
            &Direction::Up,
            &3
        ),
        3
    );
}
