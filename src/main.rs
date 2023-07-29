mod complex;
mod matrix;

use complex::{
    ComplexNumber,
    //ComplexPolarNumber
};
use matrix::Matrix;

fn main() {
    // Matrice Multiplication
    let mut matrix_1 = Matrix::new(vec![
        vec![ ComplexNumber { a: 3f32, b: 1f32}, ComplexNumber { a: 2f32, b: 1f32}],
        vec![ ComplexNumber { a: 4f32, b: 3f32}, ComplexNumber { a: 1f32, b: 2f32}]]);

    let matrix_2 = Matrix::new(vec![
        vec![ ComplexNumber { a: 8f32, b: 4f32}, ComplexNumber { a: 2f32, b: 1f32} ],
        vec![ ComplexNumber { a: 8f32, b: 4f32}, ComplexNumber { a: 2f32, b: 1f32} ]]);
    
    matrix_1 = matrix_1 * matrix_2;
    println!("Multiplied: {:?}\n", matrix_1);


    // Matrice Inversion
    let mut matrix_3 = Matrix::new(vec![
        vec![ ComplexNumber { a: 1f32, b: 1f32}, ComplexNumber { a: 0f32, b: 1f32} ],
        vec![ ComplexNumber { a: 0f32, b: 1f32}, ComplexNumber { a: 1f32, b: 0f32} ]]);

    println!("Determinant: {:?}", matrix_3.determinant());

    matrix_3.invert();
    println!("Inversion: {:?}\n", matrix_3);


    // Matrice Transposition
    let mut matrix_4 = Matrix::new(vec![
        vec![ ComplexNumber { a: 1f32, b: 1f32}, ComplexNumber { a: 0f32, b: 1f32} ],
        vec![ ComplexNumber { a: 1f32, b: 1f32}, ComplexNumber { a: 0f32, b: 1f32} ],
        vec![ ComplexNumber { a: 0f32, b: 1f32}, ComplexNumber { a: 1f32, b: 0f32} ]]);

    matrix_4.transpose();
    println!("Transposed: {:?}\n", matrix_4);

    // Matrice Conjugation
    let mut matrix_5 = Matrix::new(vec![
        vec![ ComplexNumber { a: 1f32, b: 1f32}, ComplexNumber { a: 0f32, b: 1f32} ],
        vec![ ComplexNumber { a: 0f32, b: -1f32}, ComplexNumber { a: 1f32, b: 0f32} ]]);
    
    matrix_5.conjugate();
    println!("Conjugated: {:?}\n", matrix_5);
}