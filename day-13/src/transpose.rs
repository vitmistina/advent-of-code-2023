use crate::{GroundType, Pattern};

pub trait Transposes {
    fn transpose(&self) -> Self;
}

impl Transposes for Pattern {
    fn transpose(&self) -> Self {
        let len = self.data[0].len();

        let data = (0..len)
            .map(|i| self.data.iter().map(|row| row[i].clone()).collect())
            .collect();

        Self { data }
    }
}

#[test]
fn transposes() {
    let input = Pattern {
        data: vec![vec![GroundType::Rocks], vec![GroundType::Ash]],
    };

    assert_eq!(
        input.transpose().data,
        vec![vec![GroundType::Rocks, GroundType::Ash]]
    );

    let input = Pattern {
        data: vec![vec![GroundType::Rocks, GroundType::Ash]],
    };

    assert_eq!(
        input.transpose().data,
        vec![vec![GroundType::Rocks], vec![GroundType::Ash]]
    );
}
