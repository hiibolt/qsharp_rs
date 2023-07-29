use crate::complex;

use complex::{
    ComplexNumber,
    ComplexPolarNumber
};

use std::ops::{
    Add,
    Mul
};

#[derive(Clone)]
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
        
        self.value[0][0] = d;
        self.value[0][1] = b * -1f32;
        self.value[1][0] = c * -1f32;
        self.value[1][1] = a;

        self
    }
}