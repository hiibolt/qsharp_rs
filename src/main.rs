mod qubit;
mod matrix;
mod complex;

use crate::qubit::*;
use crate::matrix::*;
use crate::complex::*;

fn main() {
    let mut q = Qubit::ket("NEG_I");

    println!("{:?}", q);
}