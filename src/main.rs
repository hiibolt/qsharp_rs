mod qubit;
mod matrix;
mod complex;
mod system;

use crate::qubit::*;
use crate::system::*;
// delete later, debug only
use crate::complex::*;
use crate::matrix::*;

fn main() {
    fn inverse_tensor_product_test () {
        let a = Matrix::new(vec![
            vec![ComplexNumber{ a: 0f32, b: 0f32 }],
            vec![ComplexNumber{ a: 0f32, b: 1f32 }],
            vec![ComplexNumber{ a: 0f32, b: 0f32 }]
        ]);
        let b = Matrix::new(vec![
            vec![ComplexNumber{ a: 1f32, b: 0f32 }],
            vec![ComplexNumber{ a: 0f32, b: 0f32 }]
        ]);
        let product = a.tensor_product( &b );

        let assumed_a = product.inverse_tensor_product( a.clone() );
        println!("Assumed A: {:?}", assumed_a);
    }
    fn linear_combination_test () {
        let mut system = System::new();

        system.allocate();
        system.allocate();
        system.allocate();

        system.apply(vec![
            (0, Gate::Standard(Gates::X())),
            (0, Gate::Standard(Gates::T())),
            (0, Gate::Standard(Gates::Z())),
            (1, Gate::Standard(Gates::H())),
            (0, Gate::Control),
            (1, Gate::Control),
            (2, Gate::Standard(Gates::X())),
        ]);

        println!(":");
        system.dump_register(0);
        system.dump_register(1);
        system.dump_register(2);
        println!(":");
        
        system.dump();
    }
    
    println!("Demonstration of the inverse tensor product");
    inverse_tensor_product_test();

    println!("Demonstration of the controlled NOT gate via linear combination");
    linear_combination_test();
}
