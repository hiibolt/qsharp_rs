use crate::{
    matrix,
    complex
};

use matrix::Matrix;
use matrix::Gate;
use complex::ComplexNumber;

const ONE_OVER_SQRT_TWO: ComplexNumber = ComplexNumber { a: 0.707106781f32, b: 0f32 };
const I_OVER_SQRT_TWO: ComplexNumber = ComplexNumber { a: 0f32, b: 0.707106781f32 };
const NEG_ONE_OVER_SQRT_TWO: ComplexNumber = ComplexNumber { a: -0.707106781f32, b: 0f32 };
const NEG_I_OVER_SQRT_TWO: ComplexNumber = ComplexNumber { a: 0f32, b: -0.707106781f32 };

const SQRT_TWO_OVER_TWO: f32 = std::f32::consts::SQRT_2 / 2f32;

pub struct Qubit {
    pub state: Matrix
}
impl std::ops::Mul<Qubit> for Qubit {
    type Output = Matrix;

    fn mul ( self, to_mul: Qubit ) -> Matrix {
        let mut state = self.state.clone();

        state * to_mul.state
    }
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
                    vec![ONE_OVER_SQRT_TWO],
                    vec![ONE_OVER_SQRT_TWO]
                ])
            },
            "NEG" => Self{
                state: Matrix::new(vec![
                    vec![ONE_OVER_SQRT_TWO],
                    vec![NEG_ONE_OVER_SQRT_TWO]
                ])
            },
            "I" => Self{
                state: Matrix::new(vec![
                    vec![ONE_OVER_SQRT_TWO],
                    vec![I_OVER_SQRT_TWO]
                ])
            },
            "NEG_I" => Self{
                state: Matrix::new(vec![
                    vec![ONE_OVER_SQRT_TWO],
                    vec![NEG_I_OVER_SQRT_TWO]
                ])
            },
            _ => panic!("Not a valid or recognized ket state!")
        }
    }

    pub fn measure ( &self ) {
        let alpha_combined = self.state[0][0].a.clone().powi(2) + self.state[0][0].b.clone().powi(2);
        let beta_combined = self.state[1][0].a.clone().powi(2) + self.state[1][0].b.clone().powi(2);

        let alpha_bar = String::from("#".repeat((alpha_combined * 20.).round() as usize)) + &"_".repeat((20. - (alpha_combined * 20.).round()) as usize);
        let beta_bar = String::from("#".repeat((beta_combined * 20.).round() as usize)) + &"_".repeat((20. - (beta_combined * 20.).round()) as usize);

        let alpha_phase = self.state[0][0]
            .clone().polar()
            .theta
            .to_degrees();
        let beta_phase = self.state[1][0]
            .clone().polar()
            .theta
            .to_degrees();
        
        println!("Chance of being measured as on: {}%", ((self.state[1][0].clone() * self.state[1][0].clone()).modulus()  * 1000000f32).round() / 10000f32);
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
    /* - IDENTITY GATE - */
    // Identity Gate - 'No effect' Gate
    #[allow(non_snake_case)]
    pub fn I ( &mut self ) -> &Self {
        self
    }

    /* - PAULI GATES - */
    // X Gate - 'Not' Gate
    #[allow(non_snake_case)]
    pub fn X ( &mut self ) -> &Self {
        self.state = Gate::X() * self.state.clone();
        self
    }
    // Y Gate - 'TODO'
    #[allow(non_snake_case)]
    pub fn Y ( &mut self ) -> &Self {
        self.state = Gate::Y() * self.state.clone();
        self
    }
    // Z Gate - 'TODO'
    #[allow(non_snake_case)]
    pub fn Z ( &mut self ) -> &Self {
        self.state = Gate::Z() * self.state.clone();
        self
    }

    /* - Hadamard Gates - */
    // H Gate - 'Superposition' gate
    #[allow(non_snake_case)]
    pub fn H ( &mut self ) -> &Self {
        self.state = Gate::H() * self.state.clone();
        self
    }

    /* - Phase Shift Gates - */
    // S Gate - 'i phase flip'
    #[allow(non_snake_case)]
    pub fn S ( &mut self ) -> &Self {
        self.state = Gate::S() * self.state.clone();
        self
    }
    // T Gate - '45 deg' (might be wrong)
    #[allow(non_snake_case)]
    pub fn T ( &mut self ) -> &Self {
        self.state = Gate::T() * self.state.clone();
        self
    }

    /* - Rotation Gates - */
    // R sub x gate - 'X rotation'
    #[allow(non_snake_case)]
    pub fn R_x( &mut self, theta: f32 ) -> &Self {
        self.state = Gate::R_x(theta) * self.state.clone();
        self
    }
    // R sub y gate - 'Y rotation'
    #[allow(non_snake_case)]
    pub fn R_y( &mut self, theta: f32 ) -> &Self {
        self.state = Gate::R_y( theta ) * self.state.clone();
        self
    }
    // R sub z gate - 'Z rotation'
    #[allow(non_snake_case)]
    pub fn R_z( &mut self, theta: f32 ) -> &Self {
        self.state = Gate::R_z( theta ) * self.state.clone();
        self
    }
    // R sub 1 gate - 'Arbitrary phase gate'
    #[allow(non_snake_case)]
    pub fn R_1( &mut self, theta: f32 ) -> &Self {
        self.state = Gate::R_1( theta ) * self.state.clone();
        self
    }
}
