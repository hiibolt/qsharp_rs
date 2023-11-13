use crate::complex;

const ONE_OVER_SQRT_TWO: ComplexNumber = ComplexNumber { a: 0.707106781f32, b: 0f32 };
const I_OVER_SQRT_TWO: ComplexNumber = ComplexNumber { a: 0f32, b: 0.707106781f32 };
const NEG_ONE_OVER_SQRT_TWO: ComplexNumber = ComplexNumber { a: -0.707106781f32, b: 0f32 };
const NEG_I_OVER_SQRT_TWO: ComplexNumber = ComplexNumber { a: 0f32, b: -0.707106781f32 };

const SQRT_TWO_OVER_TWO: f32 = std::f32::consts::SQRT_2 / 2f32;


use complex::{
    ComplexNumber,
    //ComplexPolarNumber
};

use std::ops::{
    Add,
    Sub,
    Mul,
    Div,

    AddAssign,
    SubAssign,
    MulAssign,
    DivAssign,

    Index,
    IndexMut
};

#[derive(Clone, PartialEq)]
pub struct Matrix {
    pub value: Vec<Vec<ComplexNumber>>,
    pub rows: usize,
    pub cols: usize
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
impl Add<ComplexNumber> for Matrix {
    type Output = Self;

    fn add ( self, to_add: ComplexNumber ) -> Self {
        let mut ret = self.clone();

        for row in 0..ret.value.len() {
            for col in 0..ret.value[row].len() {
                ret.value[row][col] += to_add.clone();
            }
        }

        ret
    }
}
impl Add<Matrix> for Matrix {
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
impl AddAssign<ComplexNumber> for Matrix {
    fn add_assign ( &mut self, to_add: ComplexNumber ) {
        for row in 0..self.value.len() {
            for col in 0..self.value[row].len() {
                self.value[row][col] += to_add.clone();
            }
        }
    }
}
impl AddAssign<Matrix> for Matrix {
    fn add_assign ( &mut self, to_add: Self ) {
        if self.rows != to_add.rows || self.cols != to_add.cols {
            panic!("Matrix size {}x{} doesn't match the base size {}x{}", to_add.rows, to_add.cols, self.rows, self.cols);
        }

        for row in 0..self.value.len() {
            for col in 0..self.value[row].len() {
                self.value[row][col] += to_add.value[row][col].clone();
            }
        }
    }
}
impl Sub<ComplexNumber> for Matrix {
    type Output = Self;

    fn sub ( self, to_add: ComplexNumber ) -> Self {
        let mut ret = self.clone();

        for row in 0..ret.value.len() {
            for col in 0..ret.value[row].len() {
                ret.value[row][col] -= to_add.clone();
            }
        }

        ret
    }
}
impl Sub<Matrix> for Matrix {
    type Output = Self;

    fn sub ( self, to_add: Self ) -> Self {
        if self.rows != to_add.rows || self.cols != to_add.cols {
            panic!("Matrix size {}x{} doesn't match the base size {}x{}", to_add.rows, to_add.cols, self.rows, self.cols);
        }

        let mut ret = self.clone();
        for row in 0..ret.value.len() {
            for col in 0..ret.value[row].len() {
                ret.value[row][col] -= to_add.value[row][col].clone();
            }
        }

        ret
    }
}
impl SubAssign<ComplexNumber> for Matrix {
    fn sub_assign ( &mut self, to_add: ComplexNumber ) {
        for row in 0..self.value.len() {
            for col in 0..self.value[row].len() {
                self.value[row][col] -= to_add.clone();
            }
        }
    }
}
impl SubAssign<Matrix> for Matrix {
    fn sub_assign ( &mut self, to_add: Self ) {
        if self.rows != to_add.rows || self.cols != to_add.cols {
            panic!("Matrix size {}x{} doesn't match the base size {}x{}", to_add.rows, to_add.cols, self.rows, self.cols);
        }

        for row in 0..self.value.len() {
            for col in 0..self.value[row].len() {
                self.value[row][col] -= to_add.value[row][col].clone();
            }
        }
    }
}
impl Mul<ComplexNumber> for Matrix {
    type Output = Self;

    fn mul ( self, to_mul: ComplexNumber ) -> Self {
        let mut ret = self.clone();

        for row in 0..ret.value.len() {
            for col in 0..ret.value[row].len() {
                ret.value[row][col] *= to_mul.clone();
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
impl MulAssign<ComplexNumber> for Matrix {
    fn mul_assign ( &mut self, to_mul: ComplexNumber ) {
        for row in 0..self.value.len() {
            for col in 0..self.value[row].len() {
                self.value[row][col] *= to_mul.clone();
            }
        }
    }
}
impl MulAssign<Matrix> for Matrix {
    fn mul_assign ( &mut self, to_mul: Self ) {
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

        *self = end_result;
    }
}
impl Div<ComplexNumber> for Matrix {
    type Output = Self;

    fn div ( self, to_mul: ComplexNumber ) -> Self {
        let mut ret = self.clone();

        for row in 0..ret.value.len() {
            for col in 0..ret.value[row].len() {
                ret.value[row][col] /= to_mul.clone();
            }
        }

        ret
    }
}
impl DivAssign<ComplexNumber> for Matrix {
    fn div_assign ( &mut self, to_mul: ComplexNumber ) {
        for row in 0..self.value.len() {
            for col in 0..self.value[row].len() {
                self.value[row][col] /= to_mul.clone();
            }
        }
    }
}
impl Index<usize> for Matrix {
    type Output = Vec<ComplexNumber>;

    fn index ( &self, index: usize ) -> &Self::Output {
        &(self.value[index])
    }
}
impl IndexMut<usize> for Matrix {
    fn index_mut ( &mut self, index: usize ) -> &mut Self::Output {
        &mut (self.value[index])
    }
}

impl Matrix {
    #[allow(non_snake_case)]
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
    pub fn inverse_tensor_product ( &self, old_base: Self ) -> Self {
        let mult_rows = self.rows / old_base.rows;
        let mult_cols = self.cols / old_base.cols;
        let mut multiplicand = Self::from_dimensions( mult_rows, mult_cols );
        
        if old_base.value.iter().fold( ComplexNumber{ a: 0f32, b: 0f32}, |acc, x| {
            acc + x.iter().fold( ComplexNumber{ a: 0f32, b: 0f32 }, |acc, x| acc + x.clone() )
        })  == (ComplexNumber{ a: 0f32, b: 0f32}) {
            panic!("Base MUST have at least one non-zero value to produce a multiplicative array!");
        }
        for row_idx in 0..self.value.len() {
            for col_idx in 0..self[row_idx].len() {
                let result_row_idx = if row_idx > mult_rows - 1 { (row_idx) % mult_rows } else { row_idx };
                let result_col_idx = if col_idx > mult_cols - 1 { (col_idx) % mult_cols } else { col_idx };
                
                let base_row_idx = row_idx / mult_rows;
                let base_col_idx = col_idx / mult_cols;
 
                if old_base[ base_row_idx][ base_col_idx ] == (ComplexNumber { a: 0f32, b: 0f32 }) {
                    if old_base[ base_row_idx ][ base_col_idx ] != (ComplexNumber { a: 0f32, b: 0f32 }) {
                        multiplicand[ result_row_idx ][ result_col_idx ] = ComplexNumber { a: 0f32, b: 0f32 };
                        continue;
                    }
                    continue;
                }
                
                multiplicand[ result_row_idx ][ result_col_idx ] = self[row_idx][col_idx].clone() / old_base[ base_row_idx ][ base_col_idx ].clone();
            }
        }
        
        multiplicand
    }
}


pub enum Gate {
    Control,
    AntiControl,
    Swap(usize),
    Standard(Matrix)
}
pub struct Gates {}
impl Gates {
    /* Gate Constants */
    pub fn I () -> Matrix {
        Matrix::new(vec![
            vec![ComplexNumber { a: 1f32, b: 0f32 }, ComplexNumber { a: 0f32, b: 0f32 }],
            vec![ComplexNumber { a: 0f32, b: 0f32 }, ComplexNumber { a: 1f32, b: 0f32 }],
        ])
    }

    pub fn X () -> Matrix { 
        Matrix::new(vec![
            vec![ComplexNumber { a: 0f32, b: 0f32 }, ComplexNumber { a: 1f32, b: 0f32 }],
            vec![ComplexNumber { a: 1f32, b: 0f32 }, ComplexNumber { a: 0f32, b: 0f32 }],
        ])
    }
    pub fn Y () -> Matrix {
        Matrix::new(vec![
            vec![ComplexNumber { a: 0f32, b: 0f32 }, ComplexNumber { a: 0f32, b: -1f32 }],
            vec![ComplexNumber { a: 0f32, b: 1f32 }, ComplexNumber { a: 0f32, b: 0f32 }],
        ])
    }
    pub fn Z () -> Matrix {
        Matrix::new(vec![
            vec![ComplexNumber { a: 1f32, b: 0f32 }, ComplexNumber { a: 0f32, b: 0f32 }],
            vec![ComplexNumber { a: 0f32, b: 0f32 }, ComplexNumber { a: -1f32, b: 0f32 }],
        ])
    }

    pub fn H () -> Matrix {
        Matrix::new(vec![
            vec![ONE_OVER_SQRT_TWO, ONE_OVER_SQRT_TWO],
            vec![ONE_OVER_SQRT_TWO, NEG_ONE_OVER_SQRT_TWO]
        ])
    }
    pub fn S () -> Matrix {
        Matrix::new(vec![
            vec![ComplexNumber { a: 1f32, b: 0f32 }, ComplexNumber { a: 0f32, b: 0f32 }],
            vec![ComplexNumber { a: 0f32, b: 0f32 }, ComplexNumber { a: 0f32, b: 1f32 }]
        ])
    }
    pub fn T () -> Matrix {
        Matrix::new(vec![
            vec![ComplexNumber { a: 1f32, b: 0f32 }, ComplexNumber { a: 0f32, b: 0f32 }],
            vec![ComplexNumber { a: 0f32, b: 0f32 }, ComplexNumber { a: SQRT_TWO_OVER_TWO, b: SQRT_TWO_OVER_TWO }]
        ])
    }

    pub fn R_x ( theta: f32 ) -> Matrix {
        Matrix::new(vec![
            vec![ComplexNumber { a: (theta / 2f32).cos(), b: 0f32 }, ComplexNumber { a: 0f32, b: -(theta / 2f32).sin() }],
            vec![ComplexNumber { a: 0f32, b: -(theta / 2f32).sin() }, ComplexNumber { a: (theta / 2f32).cos(), b: 0f32 }]
        ])
    }
    pub fn R_y ( theta: f32 ) -> Matrix {
        Matrix::new(vec![
            vec![ComplexNumber { a: (theta / 2f32).cos(), b: 0f32 }, ComplexNumber { a: -(theta / 2f32).sin(), b: 0f32 }],
            vec![ComplexNumber { a: (theta / 2f32).sin(), b: 0f32 }, ComplexNumber { a: (theta / 2f32).cos(), b: 0f32 }]
        ])
    }
    pub fn R_z ( theta: f32 ) -> Matrix {
        Matrix::new(vec![
            vec![ComplexNumber { a: (-theta / 2f32).cos(), b: (-theta / 2f32).sin() }, ComplexNumber { a: 0f32, b: 0f32 }],
            vec![ComplexNumber { a: 0f32, b: 0f32 }, ComplexNumber { a: (theta / 2f32).cos(), b: (theta / 2f32).sin() }]
        ])
    }
    pub fn R_1 ( theta: f32 ) -> Matrix {
        Matrix::new(vec![
            vec![ComplexNumber { a: 1f32, b: 0f32 }, ComplexNumber { a: 0f32, b: 0f32 }],
            vec![ComplexNumber { a: 0f32, b: 0f32 }, ComplexNumber { a: theta.cos(), b: theta.sin() }]
        ])
    }

}