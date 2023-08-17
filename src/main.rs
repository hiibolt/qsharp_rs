mod qubit;
mod matrix;
mod complex;
mod system;

use crate::qubit::*;
use crate::system::*;

fn main() {
    let mut system = System::new();

    let q = system.allocate();
    q.measure();

    // X Gate
    q.X();
    q.measure();

    // Y Gate
    q.Y();
    q.measure();

    // Z Gate
    q.Z();

    system.dump();
}