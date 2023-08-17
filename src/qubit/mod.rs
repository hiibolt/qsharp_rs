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
        write!(f, "Should only be used for debugging! Use dump() or measure()!\n")?;
        self.measure();
        Ok(())
    }
}
impl Qubit {
    pub fn ket ( id: &str ) -> Self {
        return match id {
            "ZERO" => Self{
                state: Matrix::new(vec![
                    vec![ComplexNumber { a: 1f32, b: 0f32 }],
                    vec![ComplexNumber { a: 0f32, b: 0f32 }]
                ])
            },
            "ONE" => Self{
                state: Matrix::new(vec![
                    vec![ComplexNumber { a: 0f32, b: 0f32 }],
                    vec![ComplexNumber { a: 1f32, b: 0f32 }]
                ])
            },
            "PLUS" => Self{
                state: Matrix::new(vec![
                    vec![ONE_OVER_SQRT_2],
                    vec![ONE_OVER_SQRT_2]
                ])
            },
            "NEG" => Self{
                state: Matrix::new(vec![
                    vec![ONE_OVER_SQRT_2],
                    vec![NEG_ONE_OVER_SQRT_2]
                ])
            },
            "I" => Self{
                state: Matrix::new(vec![
                    vec![ONE_OVER_SQRT_2],
                    vec![I_OVER_SQRT_2]
                ])
            },
            "NEG_I" => Self{
                state: Matrix::new(vec![
                    vec![ONE_OVER_SQRT_2],
                    vec![NEG_I_OVER_SQRT_2]
                ])
            },
            _ => panic!("Not a valid or recognized ket state!")
        }
    }

    pub fn measure ( &self ) {
        let alpha_combined = self.state[0][0].a.clone().powi(2) + self.state[0][0].b.clone().powi(2);
        let beta_combined = self.state[1][0].a.clone().powi(2) + self.state[1][0].b.clone().powi(2);

        let alpha_bar = String::from("#".repeat((alpha_combined * 20.).floor() as usize)) + &"_".repeat((20. - (alpha_combined * 20.).floor()) as usize);
        let beta_bar = String::from("#".repeat((beta_combined * 20.).floor() as usize)) + &"_".repeat((20. - (beta_combined * 20.).floor()) as usize);

        let alpha_phase = self.state[0][0]
            .clone().polar()
            .theta
            .to_degrees();
        let beta_phase = self.state[1][0]
            .clone().polar()
            .theta
            .to_degrees();
        
        println!("Chance of being measured as on: {}%", (self.state[1][0].clone() * self.state[1][0].clone()).modulus()  * 100f32);
        println!("|0> {:?} | {} | {}°\n|1> {:?} | {} | {}°\n", self.state[0][0], alpha_bar, alpha_phase, self.state[1][0], beta_bar, beta_phase );
    }
    pub fn new () -> Self {
        Self {
            state: Matrix::new(vec![
                vec![ComplexNumber { a: 1f32, b: 0f32 }],
                vec![ComplexNumber { a: 0f32, b: 0f32 }]
            ])
        }
    }


    /* Gate Operations */
    // X Gate - 'Not' Gate
    #[allow(non_snake_case)]
    pub fn X ( &mut self ) -> &Self {
        self.state = Matrix::new(vec![
            vec![ComplexNumber { a: 0f32, b: 0f32 }, ComplexNumber { a: 1f32, b: 0f32 }],
            vec![ComplexNumber { a: 1f32, b: 0f32 }, ComplexNumber { a: 0f32, b: 0f32 }],
        ]) * self.state.clone();
        self
    }

    // Y Gate - 'TODO'
    #[allow(non_snake_case)]
    pub fn Y ( &mut self ) -> &Self {
        self.state = Matrix::new(vec![
            vec![ComplexNumber { a: 0f32, b: 0f32 }, ComplexNumber { a: 0f32, b: -1f32 }],
            vec![ComplexNumber { a: 0f32, b: 1f32 }, ComplexNumber { a: 0f32, b: 0f32 }],
        ]) * self.state.clone();
        self
    }

    // Z Gate - 'TODO'
    #[allow(non_snake_case)]
    pub fn Z ( &mut self ) -> &Self {
        self.state = Matrix::new(vec![
            vec![ComplexNumber { a: 1f32, b: 0f32 }, ComplexNumber { a: 0f32, b: 0f32 }],
            vec![ComplexNumber { a: 0f32, b: 0f32 }, ComplexNumber { a: -1f32, b: 0f32 }],
        ]) * self.state.clone();
        self
    }
}