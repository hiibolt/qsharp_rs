mod complex;
mod matrix;

use complex::{
    ComplexNumber,
    ComplexPolarNumber
};
use matrix::Matrice;

fn main() {
    println!("Hello, world!");

    let mut matrix = Matrice::new(vec![
        vec![ ComplexNumber { a: 0f32, b: 1f32}, ComplexNumber { a: 0f32, b: 1f32} ],
        vec![ ComplexNumber { a: 0f32, b: 1f32}, ComplexNumber { a: 0f32, b: 1f32} ]]);

    matrix.add(Matrice::new(vec![
        vec![ ComplexNumber { a: 1f32, b: 1f32}, ComplexNumber { a: 1f32, b: 1f32} ],
        vec![ ComplexNumber { a: 1f32, b: 1f32}, ComplexNumber { a: 1f32, b: 1f32} ]]));
    
    println!("{:?}", matrix);
}