use crate::{GroundType, Slice};

pub trait Mirrors {
    fn check_mirrorness(&self) -> bool;
}

impl Mirrors for Slice {
    fn check_mirrorness(&self) -> bool {
        let cut_index = self.data.len() / 2;
        (0..cut_index).all(|index| self.eval_counterparts_match(&index))
    }
}

impl Slice {
    fn find_counter_parts(&self, index: &usize) -> (Vec<GroundType>, Vec<GroundType>) {
        assert!(index < &(self.data.len() / 2));
        let counter_index = self.data.len() - index - 1;
        (self.data[*index].clone(), self.data[counter_index].clone())
    }

    fn eval_counterparts_match(&self, index: &usize) -> bool {
        let tuple = self.find_counter_parts(&index);
        tuple.0 == tuple.1
    }
}

#[test]
fn finds_counterpart() {
    let input = Slice {
        data: vec![vec![GroundType::Rocks], vec![GroundType::Ash]],
    };
    assert_eq!(
        input.find_counter_parts(&0),
        (vec![GroundType::Rocks], vec![GroundType::Ash])
    );

    let input = Slice {
        data: vec![
            vec![GroundType::Rocks, GroundType::Ash],
            vec![GroundType::Rocks, GroundType::Ash],
            vec![GroundType::Rocks, GroundType::Ash],
            vec![GroundType::Ash, GroundType::Rocks],
        ],
    };
    assert_eq!(
        input.find_counter_parts(&0),
        (
            vec![GroundType::Rocks, GroundType::Ash],
            vec![GroundType::Ash, GroundType::Rocks]
        )
    );
    assert_eq!(
        input.find_counter_parts(&1),
        (
            vec![GroundType::Rocks, GroundType::Ash],
            vec![GroundType::Rocks, GroundType::Ash],
        )
    );
}

#[test]
#[should_panic]
fn panics_for_invalid_counterpart() {
    let input = Slice {
        data: vec![vec![GroundType::Rocks], vec![GroundType::Ash]],
    };
    input.find_counter_parts(&1);
}

#[test]
fn evaluates_subslice_mirrorness() {
    let input = Slice {
        data: vec![vec![GroundType::Rocks], vec![GroundType::Ash]],
    };
    assert_eq!(input.check_mirrorness(), false);

    let input = Slice {
        data: vec![vec![GroundType::Rocks], vec![GroundType::Rocks]],
    };
    assert_eq!(input.check_mirrorness(), true);

    let input = Slice {
        data: vec![
            vec![GroundType::Rocks, GroundType::Ash],
            vec![GroundType::Ash, GroundType::Rocks],
        ],
    };
    assert_eq!(input.check_mirrorness(), false);

    let input = Slice {
        data: vec![
            vec![GroundType::Rocks, GroundType::Ash],
            vec![GroundType::Rocks, GroundType::Ash],
        ],
    };
    assert_eq!(input.check_mirrorness(), true);
}
