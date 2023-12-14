use crate::{GroundType, Pattern, Slice};

pub trait CreatesSubslice {
    fn create_subslice(&self, cut_position: &usize) -> Slice;
}

impl CreatesSubslice for Pattern {
    fn create_subslice(&self, cut_position: &usize) -> Slice {
        let nearest_edge = self.find_nearest_edge(cut_position);

        let subslice = self.data[cut_position - nearest_edge..cut_position + nearest_edge].to_vec();
        Slice { data: subslice }
    }
}

impl Pattern {
    fn find_nearest_edge(&self, cut_position: &usize) -> usize {
        let top_distance = cut_position;
        let bottom_distance = self.data.len() - cut_position;
        *top_distance.min(&bottom_distance)
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
        Slice {
            data: vec![vec![GroundType::Rocks], vec![GroundType::Ash],],
        }
    );

    assert_eq!(
        input.create_subslice(&2),
        Slice {
            data: vec![vec![GroundType::Ash], vec![GroundType::Ash],],
        }
    );
}
