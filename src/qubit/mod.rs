use crate::{
    matrix,
    complex
};

use matrix::Matrix;
use complex::ComplexNumber;

const ONE_OVER_SQRT_2: ComplexNumber = ComplexNumber { a: 0.707106781f32, b: 0f32 };
const I_OVER_SQRT_2: ComplexNumber = ComplexNumber { a: 0f32, b: 0.707106781f32 };
const NEG_ONE_OVER_SQRT_2: ComplexNumber = ComplexNumber { a: -0.707106781f32, b: 0f32 };
const NEG_I_OVER_SQRT_2: ComplexNumber = ComplexNumber { a: 0f32, b: -0.707106781f32 };

pub struct Qubit {
    state: Matrix
}
impl std::fmt::Debug for Qubit {
    fn fmt ( &self, f: &mut std::fmt::Formatter<'_> ) -> std::fmt::Result {
        write!(f, "Machine Dump:\n|0> {:?}\n|1> {:?}\n", self.state[0][0], self.state[1][0] )?;
        Ok(())
    }
}
impl Qubit {
    pub fn ket ( id: &str ) -> Self {
        return match id {
            "ZERO" => Qubit::new(Matrix::new(vec![
                vec![ComplexNumber { a: 1f32, b: 0f32 }],
                vec![ComplexNumber { a: 0f32, b: 0f32 }]
            ])),
            "ONE" => Qubit::new(Matrix::new(vec![
                vec![ComplexNumber { a: 0f32, b: 0f32 }],
                vec![ComplexNumber { a: 1f32, b: 0f32 }]
            ])),
            "PLUS" => Qubit::new(Matrix::new(vec![
                vec![ONE_OVER_SQRT_2],
                vec![ONE_OVER_SQRT_2]
            ])),
            _ => panic!("Not a valid or recognized ket state!")
        }
    }

    pub fn new ( input_state: Matrix ) -> Self {
        let mut state = input_state.clone();
        state.normalize();
        Self {
            state
        }
    }
}