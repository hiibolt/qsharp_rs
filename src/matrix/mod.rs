use crate::complex;

use complex::{
    ComplexNumber,
    //ComplexPolarNumber
};

use std::ops::{
    Add,
    Mul
};

#[derive(Clone, PartialEq)]
pub struct Matrix {
    value: Vec<Vec<ComplexNumber>>,
    rows: usize,
    cols: usize
}
impl std::fmt::Debug for Matrix {
    fn fmt ( &self, f: &mut std::fmt::Formatter<'_> ) -> std::fmt::Result {
        write!(f, "Complex Matrix: {}x{}\n", self.rows, self.cols )?;
        let mut row_divider: String = String::new();
        for row in &self.value {
            let stringified_row = row
                .into_iter()
                .map(|i| format!("{:?}",i))
                .collect::<Vec<String>>()
                .join(" | ");
            row_divider = vec!["-"; stringified_row.chars().count() + 4 ].join("");
            write!(f, "{}\n| {} |\n", row_divider, stringified_row )?;
        }
        write!(f, "{}", row_divider)?;
        Ok(())
    }
}
impl Add for Matrix {
    type Output = Self;

    fn add ( self, to_add: Self ) -> Self {
        if self.rows != to_add.rows || self.cols != to_add.cols {
            panic!("Matrix size {}x{} doesn't match the base size {}x{}", to_add.rows, to_add.cols, self.rows, self.cols);
        }

        let mut ret = self.clone();
        for row in 0..ret.value.len() {
            for col in 0..ret.value[row].len() {
                ret.value[row][col] += to_add.value[row][col].clone();
            }
        }

        ret
    }
}
impl Mul<f32> for Matrix {
    type Output = Self;

    fn mul ( self, to_mul: f32 ) -> Self {
        let mut ret = self.clone();

        for row in 0..ret.value.len() {
            for col in 0..ret.value[row].len() {
                ret.value[row][col] *= to_mul;
            }
        }

        ret
    }
}
impl Mul<Matrix> for Matrix {
    type Output = Self;

    fn mul ( self, to_mul: Self ) -> Self {
        if self.cols != to_mul.rows {
            panic!("Number of columns in the base matrix must match the number of columns in the second row!");
        }

        let mut end_result = Matrix::from_dimensions( self.rows, to_mul.cols );

        for r in 0..end_result.value.len() {
            for c in 0..(end_result.value[r].len()) {
                let row: Vec<ComplexNumber> = self.value[r].clone();
                let col: Vec<ComplexNumber> = to_mul
                    .value
                    .clone()
                    .into_iter()
                    .map(|i| i[c].clone())
                    .collect();
                
                let dot_product: ComplexNumber = row
                    .clone()
                    .into_iter()
                    .enumerate()
                    .map(|(ind, i)| { // Multiply each row by element of equivalent index in each intersecting column
                        i.clone() * col[ind].clone()
                    })
                    .reduce(|total: ComplexNumber, i: ComplexNumber| { // Add each element of the now row to create the final dot product
                        total + i
                    })
                    .expect("Should never happen given the matrices make it past first check")
                    .clone();

                end_result.value[r][c] = dot_product;
            }
        }

        end_result
    }
}
impl Matrix {
    pub fn IDENTITY () -> Self {
        Matrix::new(vec![
            vec![ ComplexNumber { a: 1f32, b: 0f32}, ComplexNumber { a: 0f32, b: 0f32} ],
            vec![ ComplexNumber { a: 0f32, b: 0f32}, ComplexNumber { a: 1f32, b: 0f32} ]])
    }

    // Basic Functions
    pub fn new (value: Vec<Vec<ComplexNumber>>) -> Self {
        if !value.iter().all(|i| i.len() == value[0].len()) {
            panic!("Matrix is not two-dimensional! Ensure all rows are equal in length.");
        }
        let rows = value.len();
        let cols = value[0].len();
        Self {
            value,
            rows,
            cols
        }
    }
    pub fn from_dimensions ( rows: usize, cols: usize ) -> Self {
        let row: Vec<ComplexNumber> = vec![ComplexNumber { a: 0f32, b: 0f32 }; cols];
        let mut ret: Vec<Vec<ComplexNumber>> = Vec::new();
        for _ in 0..rows {
            ret.push( row.clone() );
        }
        Matrix::new(ret)
    }
    pub fn determinant ( &self ) -> ComplexNumber {
        if self.cols != 2 || self.rows != 2 {
            todo!();
        }
        let a = self.value[0][0].clone();
        let b = self.value[0][1].clone();
        let c = self.value[1][0].clone();
        let d = self.value[1][1].clone();

        (a * d) - (b * c)
    }
    pub fn invert ( &mut self ) -> &Self {
        let determinant = self.determinant();

        let a = self.value[0][0].clone();
        let b = self.value[0][1].clone();
        let c = self.value[1][0].clone();
        let d = self.value[1][1].clone();
        
        self.value[0][0] = d / determinant.clone();
        self.value[0][1] = b * -1f32 / determinant.clone();
        self.value[1][0] = c * -1f32 / determinant.clone();
        self.value[1][1] = a / determinant;

        self
    }
    pub fn transpose ( &mut self ) -> &Self {
        let mut ret = Matrix::from_dimensions( self.cols, self.rows );

        for r in 0..self.rows {
            for c in 0..self.cols {
                ret.value[c][r] = self.value[r][c].clone();
            }
        }

        *self = ret;
        self
    }
    pub fn conjugate ( &mut self ) -> &Self {
        for r in 0..self.rows {
            for c in 0..self.cols {
                self.value[r][c].conjugate();
            }
        }
        self
    }
    pub fn adjunct ( &mut self ) -> &Self {
        self.transpose();
        self.conjugate()
    }
    pub fn unitary ( &self ) -> bool {
        (self.clone() * self.clone().adjunct().clone()) == Matrix::IDENTITY()
    }

    // Advanced Functions
    pub fn inner_product ( &self, to_mul: &Self ) -> ComplexNumber {
        if self.cols != 1 || to_mul.cols != 1 {
            panic!("Both matrices must have only one column!");
        }

        (self.clone().adjunct().clone() * to_mul.clone()).value[0][0].clone()
    }
    pub fn outer_product ( &self, to_mul: &Self ) -> Self {
        if self.cols != 1 || to_mul.cols != 1 {
            panic!("Both matrices must have only one column!");
        }

        self.clone() * to_mul.clone().adjunct().clone()
    }
    pub fn normalize ( &mut self ) -> &Self {
        if self.cols != 1 {
            todo!(); // impl according to https://mathforums.com/t/how-do-i-normalize-a-matrix.18218/
        }
        let norm = self.inner_product( &self ).a.sqrt();

        for r in 0..self.value.len() {
            self.value[r][0] /= norm;
        }

        self
    }

    // Further Advanced Functions
    pub fn tensor_product ( &self, to_mul: &Self ) -> Self {
        let mut ret = Matrix::from_dimensions( self.rows * to_mul.rows, self.cols * to_mul.cols );

        for r in 0..self.value.len() {
            for c in 0..self.value[r].len() {
                for r_n in 0..to_mul.value.len() {
                    for c_n in 0..to_mul.value[r_n].len() {
                        ret.value[r * to_mul.value.len() + r_n][c * to_mul.value[c].len() + c_n] = self.value[r][c].clone() * to_mul.value[r_n][c_n].clone();
                    }
                }
            }
        }

        ret
    }
    pub fn eigenvalue_from_eigenvector ( &self, eigenvector: &Self ) -> Option<ComplexNumber> {
        if self.rows != self.cols || self.cols != eigenvector.rows {
            panic!("Arguments must be of size n x n and n x 1!");
        }
        let result = self.clone() * eigenvector.clone();

        let mut ret: Option<ComplexNumber> = None;
        for r in 0..eigenvector.value.len() {
            if eigenvector.value[r][0] != (ComplexNumber { a: 0f32, b: 0f32 }) {
                let eigenvalue = result.value[0][0].clone() / eigenvector.value[0][0].clone();
                match ret {
                    Some(_) => return None,
                    None => ret = Some(eigenvalue)
                }
            }
        }
        ret
    }
    pub fn eigenvector_from_eigenvalue ( &self, eigenvalue: ComplexNumber ) -> Self {
        let mut ret = Matrix::from_dimensions( 2, 1 );

        let a = self.value[0][0].clone();
        let b = self.value[0][1].clone();
        let c = self.value[1][0].clone();
        let d = self.value[1][1].clone();

        // The many, many edge cases
        if c == (ComplexNumber { a: 0f32, b: 0f32 }) {
            if a - eigenvalue.clone() != (ComplexNumber { a: 0f32, b: 0f32 }) {
                ret.value[0][0] = ComplexNumber { a: 0f32, b: 0f32 };
                ret.value[1][0] = ComplexNumber { a: 1f32, b: 0f32 };

                return ret;
            } else {
                if c == (ComplexNumber { a: 0f32, b: 0f32 }) {
                    ret.value[0][0] = ComplexNumber { a: 1f32, b: 0f32 };
                    ret.value[1][0] = ComplexNumber { a: 0f32, b: 0f32 };

                    return ret;
                } else {
                    ret.value[0][0] = (d - eigenvalue) / (c * -1f32 );
                    ret.value[1][0] = ComplexNumber { a: 1f32, b: 0f32 };

                    return ret;
                }
            }
        }

        ret.value[0][0] = ComplexNumber { a: 1f32, b: 0f32 };
        ret.value[1][0] = (eigenvalue - a) / b;

        ret
    }
}