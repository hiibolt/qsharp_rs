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
    /* The Y gate */
    fn exercise_1 () {
        let mut system = System::new();

        let q = system.allocate();

        q.Y();
        
        system.dump();
    }
    /* Applying a global phase i */
    fn exercise_2 () {
        let mut system = System::new();

        let q = system.allocate_ket("I");

        q.Z();
        q.Y();
        q.X();

        system.dump();
    }
    /* Applying a -1 phase to |0>state */
    fn exercise_3 () {
        let mut system = System::new();

        let q = system.allocate_ket("I");

        q.X();
        q.Z();
        q.X();

        system.dump();
    }
    /* Preparing a |-> state */
    fn exercise_4 () {
        let mut system = System::new();

        let q = system.allocate();

        q.X();
        q.H();

        system.dump();
    }
    /* Three-fourths phase */
    fn exercise_5 () {
        let mut system = System::new();

        let q = system.allocate_ket("ONE");

        q.S();
        q.T();

        system.dump();
    }
    /* Preparing a rotated state */
    fn exercise_6 () {
        let mut system = System::new();

        let q = system.allocate();

        let alpha = 0.5f32;
        let beta = 0.86602540378443864676372317075294f32; // (Square root of 0.75)

        // It doesn't matter which is Y or X as
        // long as the correct trig function is used
        let parameter_theta = beta.atan2(alpha) * 2f32;

        q.R_y(parameter_theta);

        system.dump();
    }
    /* Preparing an arbitrary state */
    fn exercise_7 () {
        let mut system = System::new();

        let q = system.allocate();

        let alpha = 0.5f32;
        let beta = 0.86602540378443864676372317075294f32; // (Square root of 0.75)
        let theta = std::f32::consts::FRAC_PI_2;

        // It doesn't matter which is Y or X as
        // long as the correct trig function is used
        let parameter_theta = beta.atan2(alpha) * 2f32;

        q.R_y(parameter_theta);
        q.measure();

        q.R_1(theta);

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

    println!("Exercise 4:\n--------------------------");
    exercise_4();

    println!("Exercise 5:\n--------------------------");
    exercise_5();

    println!("Exercise 6:\n--------------------------");
    exercise_6();

    println!("Exercise 7:\n--------------------------");
    exercise_7();
}