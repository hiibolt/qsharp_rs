mod qubit;
mod matrix;
mod complex;
mod system;

use crate::qubit::*;
use crate::system::*;

fn main() {
    fn basic_example () {
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
    fn exercise_1 () {
        let mut system = System::new();

        let q = system.allocate();

        q.Y();
        
        system.dump();
    }
    fn exercise_2 () {
        let mut system = System::new();

        let q = system.allocate_ket("I");

        q.Z();
        q.Y();
        q.X();

        system.dump();
    }
    fn exercise_3 () {
        let mut system = System::new();

        let q = system.allocate_ket("I");

        q.X();
        q.Z();
        q.X();

        system.dump();
    }

    println!("Basic Example\n--------------------------");
    basic_example();

    println!("Exercise 1:\n--------------------------");
    exercise_1();

    println!("Exercise 2:\n--------------------------");
    exercise_2();

    println!("Exercise 3:\n--------------------------");
    exercise_3();
}