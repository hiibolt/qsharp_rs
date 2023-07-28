mod complex;
mod matrix;

use complex::{
    ComplexNumber,
    ComplexPolarNumber
};
use matrix::Matrice;

fn main() {
    let matrix_1 = Matrice::new(vec![
        vec![ ComplexNumber { a: 0f32, b: 1f32}, ComplexNumber { a: 0f32, b: 1f32} ],
        vec![ ComplexNumber { a: 0f32, b: 1f32}, ComplexNumber { a: 0f32, b: 1f32} ]]);

    let matrix_2 = Matrice::new(vec![
        vec![ ComplexNumber { a: 1f32, b: 1f32}, ComplexNumber { a: 1f32, b: 1f32} ],
        vec![ ComplexNumber { a: 1f32, b: 1f32}, ComplexNumber { a: 1f32, b: 1f32} ]]);
    
    println!("{:?}", matrix_1.matrix_mult(matrix_2));
}