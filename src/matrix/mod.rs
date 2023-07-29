use crate::complex;

use complex::{
    ComplexNumber,
    ComplexPolarNumber
};

#[derive(Clone)]
pub struct Matrice {
    value: Vec<Vec<ComplexNumber>>,
    rows: usize,
    cols: usize
}
impl std::fmt::Debug for Matrice {
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
impl Matrice {
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
        Matrice::new(ret)
    }
    pub fn add ( &mut self, to_add: Matrice ) -> &Self {
        if self.rows != to_add.rows || self.cols != to_add.cols {
            panic!("Matrix size {}x{} doesn't match the base size {}x{}", to_add.rows, to_add.cols, self.rows, self.cols);
        }
        for row in 0..self.value.len() {
            for col in 0..self.value[row].len() {
                self.value[row][col].add(to_add.value[row][col].clone());
            }
        }

        self
    }
    pub fn scalar_mult ( &mut self, to_mult: f32 ) -> &Self {
        for row in 0..self.value.len() {
            for col in 0..self.value[row].len() {
                self.value[row][col].scalar_mult(to_mult);
            }
        }

        self
    }
    pub fn matrix_mult ( self, to_mult: Matrice ) -> Self {
        if self.cols != to_mult.rows {
            panic!("Number of columns in the base matrix must match the number of columns in the second row!");
        }

        let mut end_result = Matrice::from_dimensions( self.rows, to_mult.cols );

        for r in 0..end_result.value.len() {
            for c in 0..(end_result.value[r].len()) {
                let row: Vec<ComplexNumber> = self.value[r].clone();
                let col: Vec<ComplexNumber> = to_mult
                    .value
                    .clone()
                    .into_iter()
                    .map(|i| i[c].clone())
                    .collect();
                
                println!("{:?}, {:?}", row, col);

                let dot_product: ComplexNumber = row
                    .clone()
                    .into_iter()
                    .enumerate()
                    .map(|(ind, mut i)| { // Multiply each row by element of equivalent index in each intersecting column
                        i.mult(col[ind].clone());
                        i
                    })
                    .reduce(|total: ComplexNumber, i: ComplexNumber| { // Add each element of the now row to create the final dot product
                        total.clone().add(i.clone()).clone()
                    })
                    .expect("Should never happen given the matrices make it past first check")
                    .clone();


                end_result.value[r][c] = dot_product;
            }
        }

        end_result
    }
}