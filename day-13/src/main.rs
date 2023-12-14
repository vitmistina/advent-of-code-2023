mod parsing;

fn main() {
    println!("Hello, world!");
}

#[derive(Debug, PartialEq, Clone)]
enum GroundType {
    Ash,
    Rocks,
}

#[derive(Debug, PartialEq)]
struct Pattern {
    data: Vec<Vec<GroundType>>,
}

impl Pattern {
    fn find_nearest_edge(&self, cut_position: &usize) -> usize {
        let top_distance = cut_position;
        let bottom_distance = self.data.len() - cut_position;
        *top_distance.min(&bottom_distance)
    }

    fn create_subslice(&self, cut_position: &usize) -> Self {
        let nearest_edge = self.find_nearest_edge(cut_position);

        let subslice = self.data[cut_position - nearest_edge..cut_position + nearest_edge].to_vec();
        Self { data: subslice }
    }

    fn find_counter_parts(
        &self,
        cut_position: &usize,
        index: &usize,
    ) -> (Vec<GroundType>, Vec<GroundType>) {
        let counter_index = 2 * cut_position - index - 1;
        (self.data[*index].clone(), self.data[counter_index].clone())
    }

    fn check_mirrorness(&self, cut_position: &usize) -> bool {
        false
    }
}

#[test]
fn finds_nearest_edges() {
    let input = Pattern {
        data: vec![
            vec![GroundType::Rocks],
            vec![GroundType::Ash],
            vec![GroundType::Ash],
        ],
    };
    assert_eq!(input.find_nearest_edge(&0), 0);
    assert_eq!(input.find_nearest_edge(&1), 1);
    assert_eq!(input.find_nearest_edge(&2), 1);
    assert_eq!(input.find_nearest_edge(&3), 0)
}

#[test]
fn finds_subslice() {
    let input = Pattern {
        data: vec![
            vec![GroundType::Rocks],
            vec![GroundType::Ash],
            vec![GroundType::Ash],
        ],
    };

    assert_eq!(
        input.create_subslice(&1),
        Pattern {
            data: vec![vec![GroundType::Rocks], vec![GroundType::Ash],],
        }
    );

    assert_eq!(
        input.create_subslice(&2),
        Pattern {
            data: vec![vec![GroundType::Ash], vec![GroundType::Ash],],
        }
    );
}

#[test]
fn finds_counterpart() {
    let input = Pattern {
        data: vec![vec![GroundType::Rocks], vec![GroundType::Ash]],
    };
    assert_eq!(
        input.find_counter_parts(&1, &0),
        (vec![GroundType::Rocks], vec![GroundType::Ash])
    );

    let input = Pattern {
        data: vec![
            vec![GroundType::Rocks, GroundType::Ash],
            vec![GroundType::Rocks, GroundType::Ash],
            vec![GroundType::Rocks, GroundType::Ash],
            vec![GroundType::Ash, GroundType::Rocks],
        ],
    };
    assert_eq!(
        input.find_counter_parts(&2, &0),
        (
            vec![GroundType::Rocks, GroundType::Ash],
            vec![GroundType::Ash, GroundType::Rocks]
        )
    )
}

#[test]
fn evaluates_subslice_mirrorness() {
    let input = Pattern {
        data: vec![vec![GroundType::Rocks], vec![GroundType::Ash]],
    };
    assert_eq!(input.check_mirrorness(&1), false);

    let input = Pattern {
        data: vec![vec![GroundType::Rocks], vec![GroundType::Rocks]],
    };
    assert_eq!(input.check_mirrorness(&1), true);

    let input = Pattern {
        data: vec![
            vec![GroundType::Rocks, GroundType::Ash],
            vec![GroundType::Ash, GroundType::Rocks],
        ],
    };
    assert_eq!(input.check_mirrorness(&1), true);
}
