mod qubit;
mod matrix;
mod complex;
mod system;

use crate::qubit::*;
use crate::system::*;
// delete later, debug only
use crate::complex::*;
use crate::matrix::*;

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


    /* QUANTUM KATAS */
    /* Task 1.1. State flip: |0> to |1> and vice versa */
    fn task1_1 () {
        let mut system = System::new();

        let q = system.allocate();

        q.X();

        system.dump();
    }
    /* Task 1.2. Basis change: |0> to |+> and |1> to |-> (and vice versa) */
    fn task1_2 () {
        let mut system = System::new();

        let q = system.allocate();

        q.H();

        system.dump();
    }
    /* Task 1.3. Sign flip: |+> to |-> and vice versa. */
    fn task1_3 () {
        let mut system = System::new();

        let q = system.allocate_ket("PLUS");

        q.Z();
        
        system.dump();
    }
    /* Task 1.4. Amplitude change: |0> to |1>. */
    fn task1_4 () {
        let mut system = System::new();

        let q = system.allocate();
        let theta = std::f32::consts::FRAC_PI_2;

        q.R_y(theta * 2f32);

        system.dump();
    }
    /* Task 1.5. Phase flip */
    fn task1_5 () {
        let mut system = System::new();

        let q = system.allocate_ket("ONE");

        q.S();

        // Alternatively, using the R1 gate suffices:
        // q.R_1(std::consts::f32::FRAC_PI_2)

        system.dump();
    }
    /* Task 1.6. Phase Change */
    fn task1_6 () {
        let mut system = System::new();

        let q = system.allocate();
        let theta = std::f32::consts::FRAC_PI_2;

        q.R_1( theta );

        system.dump();
    }
    /* Task 1.7. Global phase change */
    fn task1_7 () {
        let mut system = System::new();

        let q = system.allocate_ket("PLUS");

        q.Z();
        q.X();
        q.Z();
        q.X();

        system.dump();
    }

    /* BASIC GATES EXERCISES */
    /* Task 1.1. State flip: |0> to |1> and vice versa */
    fn bg_task1_1 () {
        let mut system = System::new();

        let q = system.allocate();

        q.X();

        system.dump();
    }

    /* Task 1.2. Basis change: |0> to |+> and |1> to |-> (and vice versa) */
    fn bg_task1_2 () {
        let mut system = System::new();

        let q = system.allocate();

        q.H();

        system.dump();
    }

    /* Task 1.3. Basis change: |0> to |+> and |1> to |-> (and vice versa) */
    fn bg_task1_3 () {
        let mut system = System::new();

        let q = system.allocate_ket("PLUS");

        q.Z();

        system.dump();
    }

    /* Task 1.4. Amplitude change: |0> to cos(alpha)|0> + sin(alpha)|1>. */
    fn bg_task1_4 () {
        let mut system = System::new();

        let q = system.allocate();
        let alpha = 32.334f32;

        q.R_y(alpha * 2f32);

        system.dump();
    }

    /* Task 1.5. Phase Flip */
    fn bg_task1_5 () {
        let mut system = System::new();

        let q = system.allocate_ket("ONE");

        q.S();

        // or, 
        // q.T();

        system.dump();
    }

    /* Task 1.6. Phase change */
    fn bg_task1_6 () {
        let mut system = System::new();

        let q = system.allocate_ket("ONE");
        let alpha = std::f32::consts::FRAC_PI_2;

        q.R_1( alpha );

        system.dump();
    }

    /* Task 1.7. Global phase change */
    fn bg_task1_7 () {
        let mut system = System::new();

        let q = system.allocate_ket("PLUS");

        q.Z();
        q.X();
        q.Z();
        q.X();

        system.dump();
    }

    /* Task 1.8. Bell state change -1 */
    fn bg_task1_8 () {
        todo!();
    }

    /* MULTI QUBIT SYSTEMS EXERCISES */
    /* Exercise 1: Show that the state is separable */
    // Yes, (1/sqrt(2))[ 1, -i ] * (1/sqrt(2))[1, i]

    /* Exercise 2: Is this separable? */
    // No, the system of equations has no solution

    /* Exercise 3: Prepare a basis state */
    fn mqs_exercise_3 () {
        let mut system = System::new();

        system.allocate_ket("ONE");
        system.allocate();

        system[0].unwrap_qubit().X();
        system[1].unwrap_qubit().X();

        system.dump();
    }

    /* Exercise 4: Prepare a superposition of two basis states */
    fn mqs_exercise_4 () {
        let mut system = System::new();

        system.allocate();
        system.allocate();

        system[1].unwrap_qubit().X();
        system[1].unwrap_qubit().H();

        system.dump();
    }

    /* Exercise 5: Prepare a superposition with real amplitudes */
    fn mqs_exercise_5 () {
        let mut system = System::new();

        system.allocate();
        system.allocate();

        system[0].unwrap_qubit().H();

        system[1].unwrap_qubit().X();
        system[1].unwrap_qubit().H();

        system.dump();
    }

    /* Exercise 6: Prepare a superposition with complex amplitudes */
    fn mqs_exercise_6 () {
        let mut system = System::new();

        system.allocate();
        system.allocate();

        system[0].unwrap_qubit().H();
        system[0].unwrap_qubit().R_1(std::f32::consts::FRAC_PI_2);

        system[1].unwrap_qubit().H();
        system[1].unwrap_qubit().R_1(std::f32::consts::FRAC_PI_4);

        system.dump();
    }


    /* MULTI QUBIT GATES EXERCISES */
    /* Exercise 1: Compound Gate */
    fn mqg_exercise_1 () {
        let mut system = System::new();

        system.allocate();
        system.allocate();
        system.allocate();

        system[0].unwrap_qubit().S();
        system[1].unwrap_qubit().I();
        system[2].unwrap_qubit().Y();

        system.dump();
    }
    /* Exercise 2: Preparing a Bell state */
    fn mqg_exercise_2 () {
        let mut system = System::new();

        system.allocate();
        system.allocate();

        system[0].unwrap_qubit().H();
        system.CNOT(0, 1);

        system.dump();
    }
    /* Exercise 3: Swapping two qubits */
    fn mqg_exercise_3 () {
        let mut system = System::new();

        system.allocate();
        system.allocate();


        system[0].unwrap_qubit().X();

        system.dump_register(0);
        system.dump_register(1);

        system.SWAP(0, 1);

        system.dump_register(0);
        system.dump_register(1);

        system.dump();
    }
    /* Exercise 4: Controlled Rotation */
    fn mqg_exercise_4 () {
        let mut system = System::new();

        system.allocate();
        system.allocate();
        system.allocate();

        system[0].unwrap_qubit().X();
        system[1].unwrap_qubit().X();

        //system.CONTROLLED_X( 0, vec![1, 2] );

        println!("Built Gate: {:?}", system.build_gate( vec![(1, Matrix::new(vec![
            vec![ComplexNumber{ a: 1f32, b: 0f32 }, ComplexNumber{ a: 0f32, b: 0f32}],
            vec![ComplexNumber{ a: 0f32, b: 0f32 }, ComplexNumber{ a: 1f32, b: 0f32}]
        ]))] ));
        
        system.dump();
    }

    /* Other */
    fn inverse_tensor_product_test () {
        let a = Matrix::new(vec![
            vec![ComplexNumber{ a: 0f32, b: 0f32 }],
            vec![ComplexNumber{ a: 0f32, b: 1f32 }],
            vec![ComplexNumber{ a: 0f32, b: 0f32 }]
        ]);
        let b = Matrix::new(vec![
            vec![ComplexNumber{ a: 1f32, b: 0f32 }],
            vec![ComplexNumber{ a: 0f32, b: 0f32 }]
        ]);
        let product = a.tensor_product( &b );

        let assumed_a = product.inverse_tensor_product( a.clone() );
        println!("Assumed A: {:?}", assumed_a);
    }
    fn linear_combination_test () {
        let mut system = System::new();

        system.allocate();
        system[0].unwrap_qubit().X();
        system[0].unwrap_qubit().T();
        system[0].unwrap_qubit().Z();
        system.allocate();

        println!("Before dump:");
        system.dump_register(0);
        system.dump_register(1);

        system.CONTROLLED_X( 0, 1 );
        println!("After dump:");
        system.dump_register(0);
        system.dump_register(1);

        system.dump();
    }
    /* Exercise 5: */ 
    println!("QUANTUM KATA EXERCISES (INCOMPLETE!)");
    println!("Task 1.1:\n--------------------------");
    task1_1();

    println!("Task 1.2:\n--------------------------");
    task1_2();

    println!("Task 1.3:\n--------------------------");
    task1_3();

    println!("Task 1.4:\n--------------------------");
    task1_4();

    println!("Task 1.5:\n--------------------------");
    task1_5();

    println!("Task 1.6:\n--------------------------");
    task1_6();

    println!("Task 1.7:\n--------------------------");
    task1_7();


    println!("BASIC GATES EXERCISES (INCOMPLETE)");
    println!("BG Task 1.1:\n--------------------------");
    bg_task1_1();

    println!("BG Task 1.2:\n--------------------------");
    bg_task1_2();

    println!("BG Task 1.3:\n--------------------------");
    bg_task1_3();

    println!("BG Task 1.4:\n--------------------------");
    bg_task1_4();

    println!("BG Task 1.5:\n--------------------------");
    bg_task1_5();

    println!("BG Task 1.6:\n--------------------------");
    bg_task1_6();

    println!("BG Task 1.7:\n--------------------------");
    bg_task1_7();

    println!("MULTI QUBIT SYSTEM EXERCISES (DONE)");
    println!("MQS Exercise 3:\n--------------------------");
    mqs_exercise_3();

    println!("MQS Exercise 4:\n--------------------------");
    mqs_exercise_4();

    println!("MQS Exercise 5:\n--------------------------");
    mqs_exercise_5();

    println!("MQS Exercise 6:\n--------------------------");
    mqs_exercise_6();

    println!("MULTI QUBIT GATE EXERCISES (INCOMPLETE)");
    println!("MQG Exercise 1:\n--------------------------");
    mqg_exercise_1();

    println!("MQG Exercise 2:\n--------------------------");
    mqg_exercise_2();

    println!("MQG Exercise 3:\n--------------------------");
    mqg_exercise_3();

    println!("MQG Exercise 4:\n--------------------------");
    mqg_exercise_4();


    /* OTHER */
    println!("Demonstration of the inverse tensor product");
    inverse_tensor_product_test();

    println!("Demonstration of the controlled NOT gate via linear combination");
    linear_combination_test();
}
