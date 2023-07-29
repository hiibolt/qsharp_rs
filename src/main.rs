mod complex;
mod matrix;

use complex::{
    ComplexNumber,
    ComplexPolarNumber
};
use matrix::Matrix;

fn main() {
    let mut matrix_1 = Matrix::new(vec![
        vec![ ComplexNumber { a: 1f32, b: 1f32}, ComplexNumber { a: 2f32, b: 1f32} ]]);

    let matrix_2 = Matrix::new(vec![
        vec![ ComplexNumber { a: 1f32, b: 1f32}],
        vec![ ComplexNumber { a: 1f32, b: 1f32}]]);
    
    matrix_1 = matrix_1 * matrix_2;
    println!("{:?}", matrix_1);
}