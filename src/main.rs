mod complex;
mod matrix;

use complex::{
    ComplexNumber,
    //ComplexPolarNumber
};
use matrix::Matrix;

fn main() {
    /* Basic Functions */
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


    // Matrice Adjunction
    let mut matrix_6 = Matrix::new(vec![
        vec![ ComplexNumber { a: 1f32, b: 1f32}, ComplexNumber { a: 0f32, b: 1f32} ],
        vec![ ComplexNumber { a: 0f32, b: -1f32}, ComplexNumber { a: 1f32, b: 0f32} ]]);
    
    matrix_6.adjunct();
    println!("Adjuncted: {:?}\n", matrix_6);


    // Unitary Verification
    let mut matrix_7 = Matrix::new(vec![
        vec![ ComplexNumber { a: 1f32, b: 0f32}, ComplexNumber { a: 0f32, b: 0f32} ],
        vec![ ComplexNumber { a: 0f32, b: 0f32}, ComplexNumber { a: 1f32, b: 0f32} ]]);
    
    println!("Is unitary: {}\n", matrix_7.unitary());


    /* Advanced Functions */
    // Inner Product
    let mut matrix_8 = Matrix::new(vec![
        vec![ ComplexNumber { a: 3f32, b: 1f32} ],
        vec![ ComplexNumber { a: 4f32, b: 3f32} ]]);

    let matrix_9 = Matrix::new(vec![
        vec![ ComplexNumber { a: 8f32, b: 4f32} ],
        vec![ ComplexNumber { a: 8f32, b: 4f32} ]]);
    
    println!("Inner product: {:?}\n", matrix_8.inner_product(&matrix_9));


    // Normalize Matrix
    let mut matrix_10 = Matrix::new(vec![
        vec![ ComplexNumber { a: 1f32, b: 0f32}],
        vec![ ComplexNumber { a: 1f32, b: 0f32}],
        vec![ ComplexNumber { a: 1f32, b: 0f32}]]);
    
    println!("Normalized: {:?}\n", matrix_10.normalize());

    
    // Outer Product
    let matrix_11 = Matrix::new(vec![
        vec![ ComplexNumber { a: 3f32, b: 1f32} ],
        vec![ ComplexNumber { a: 4f32, b: 3f32} ]]);

    let matrix_12 = Matrix::new(vec![
        vec![ ComplexNumber { a: 8f32, b: 4f32} ],
        vec![ ComplexNumber { a: 8f32, b: 4f32} ],
        vec![ ComplexNumber { a: 8f32, b: 4f32} ]]);
    
    println!("Outer product: {:?}\n", matrix_11.outer_product(&matrix_12));



    /* Further Advanced Functions */
    // Tensor Product
    let matrix_13 = Matrix::new(vec![
        vec![ ComplexNumber { a: 3f32, b: 1f32}, ComplexNumber { a: 2f32, b: 1f32}],
        vec![ ComplexNumber { a: 4f32, b: 3f32}, ComplexNumber { a: 1f32, b: 2f32}]]);

    let matrix_14 = Matrix::new(vec![
        vec![ ComplexNumber { a: 8f32, b: 4f32}, ComplexNumber { a: 2f32, b: 1f32} ],
        vec![ ComplexNumber { a: 8f32, b: 4f32}, ComplexNumber { a: 2f32, b: 1f32} ]]);
    
    println!("Tensor Product: {:?}\n", matrix_13.tensor_product(&matrix_14));


    // Procuring an eigenvalue from a base vector and an eigenvector
    let matrix_14 = Matrix::new(vec![
        vec![ ComplexNumber { a: 1f32, b: 0f32}, ComplexNumber { a: 0f32, b: 0f32}],
        vec![ ComplexNumber { a: 0f32, b: 0f32}, ComplexNumber { a: 1f32, b: 0f32}]]);

    let eigenvector_1 = Matrix::new(vec![
        vec![ ComplexNumber { a: 1f32, b: 0f32} ],
        vec![ ComplexNumber { a: 0f32, b: 0f32} ]]);
    
    println!("Eigenvalue: {:?}\n", matrix_14.eigenvalue_from_eigenvector( &eigenvector_1));


    // Procuring an eigenvector from a base vector and an eigenvalue
    let matrix_15 = Matrix::new(vec![
        vec![ ComplexNumber { a: 1f32, b: 86f32}, ComplexNumber { a: 0f32, b: 28f32}],
        vec![ ComplexNumber { a: 12f32, b: 23f32}, ComplexNumber { a: 1f32, b: 23f32}]]);

    let eigenvalue_1 = ComplexNumber { a: 1f32, b: 18f32};
    
    println!("Eigenvalue: {:?}\n", matrix_15.eigenvector_from_eigenvalue( eigenvalue_1 ));
}