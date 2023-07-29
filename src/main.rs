mod complex;
mod matrix;

use complex::{
    ComplexNumber,
    ComplexPolarNumber
};
use matrix::Matrix;

fn main() {
    let mut matrix_1 = Matrix::new(vec![
        vec![ ComplexNumber { a: 3f32, b: 1f32}, ComplexNumber { a: 2f32, b: 1f32}],
        vec![ ComplexNumber { a: 4f32, b: 3f32}, ComplexNumber { a: 1f32, b: 2f32}]]);

    let matrix_2 = Matrix::new(vec![
        vec![ ComplexNumber { a: 8f32, b: 4f32}, ComplexNumber { a: 2f32, b: 1f32} ],
        vec![ ComplexNumber { a: 8f32, b: 4f32}, ComplexNumber { a: 2f32, b: 1f32} ]]);
    
    matrix_1 = matrix_1 * matrix_2;
    println!("{:?}", matrix_1);



    let mut matrix_3 = Matrix::new(vec![
        vec![ ComplexNumber { a: 1f32, b: 1f32}, ComplexNumber { a: 0f32, b: 1f32} ],
        vec![ ComplexNumber { a: 0f32, b: 1f32}, ComplexNumber { a: 1f32, b: 0f32} ]]);

    println!("{:?}", matrix_3.determinant());

    matrix_3.invert();
    println!("{:?}", matrix_3);



    let mut matrix_4 = Matrix::new(vec![
        vec![ ComplexNumber { a: 1f32, b: 1f32}, ComplexNumber { a: 0f32, b: 1f32} ],
        vec![ ComplexNumber { a: 1f32, b: 1f32}, ComplexNumber { a: 0f32, b: 1f32} ],
        vec![ ComplexNumber { a: 0f32, b: 1f32}, ComplexNumber { a: 1f32, b: 0f32} ]]);

    matrix_4.transpose();
    println!("{:?}", matrix_4);
}