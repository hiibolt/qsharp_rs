use crate::{
    matrix,
    complex,
    qubit
};
use qubit::Qubit;
use matrix::Matrix;
use complex::ComplexNumber;

pub struct System {
    state: Vec<StateEntry>,
}
pub enum StateEntry {
    StandardQubit(Qubit),
    EntangledState(Matrix),
    EntangledStatePtr(usize)
}
impl StateEntry {
    pub fn unwrap_qubit ( &mut self ) -> &mut Qubit {
        match self {
            StateEntry::StandardQubit(ref mut q) => { return q; },
            StateEntry::EntangledState(_) | StateEntry::EntangledStatePtr(_) => { panic!("Is entangled! Not a lone qubit."); },
        }
    }
}

impl std::ops::Index<usize> for System {
    type Output = StateEntry;

    fn index ( &self, index: usize ) -> &Self::Output {
        &self.state[index]
    }
}
impl std::ops::IndexMut<usize> for System {
    fn index_mut ( &mut self, index: usize ) -> &mut Self::Output {
        &mut self.state[index]
    }
}
impl System {
    pub fn dump ( self ) {
        println!("Machine Dump:");
        let mut base;
        match &self.state[0] {
            StateEntry::StandardQubit(q) => {
                base = q.state.clone();
            },
            StateEntry::EntangledState(s) => {
                base = s.clone();
            }
            StateEntry::EntangledStatePtr(_) => panic!("Impossible")
        }
        for i in 1..self.state.len() {
            match &self.state[i] {
                StateEntry::StandardQubit(q) => {
                    base = base.tensor_product(&q.state);
                },
                StateEntry::EntangledState(s) => {
                    base = base.tensor_product(&s);
                },
                StateEntry::EntangledStatePtr(_) => {}
            }
        }
        println!("{:?}", base);
    }
    pub fn dump_register ( &self, index: usize ) {
        println!("Register Dump:");

        if index > self.state.len() {
            panic!("Register {} does not exist!", index);
        }

        match &self.state[index] {
            StateEntry::StandardQubit(q) => {
                q.measure();
            },
            StateEntry::EntangledState(_) | StateEntry::EntangledStatePtr(_) => {
                println!("Register is currently entangled!");
            }
        }
    }
    pub fn new () -> Self {
        return System {
            state: Vec::new()
        }
    }

    pub fn allocate ( &mut self ) -> &mut Qubit {
        self.state.push(StateEntry::StandardQubit(Qubit::new()));
        match self.state
            .last_mut()
            .expect("Since we just allocated an element, it should be impossible to have no element")
        {
            StateEntry::StandardQubit(ref mut q) => {
                return q;
            },
            StateEntry::EntangledState(_) | StateEntry::EntangledStatePtr(_) => panic!("Impossible behavior")
        }
    }
    pub fn allocate_ket ( &mut self, id: &str ) -> &mut Qubit {
        println!("You should complete this process manually!\nQubits start in state |0>.");
        self.state.push(StateEntry::StandardQubit(Qubit::ket(id)));

        match self.state
            .last_mut()
            .expect("Since we just allocated an element, it should be impossible to have no element")
        {
            StateEntry::StandardQubit(ref mut q) => {
                return q;
            },
            StateEntry::EntangledState(_) | StateEntry::EntangledStatePtr(_) => panic!("Impossible behavior")
        }
    }
    
    /* Multi-Qubit Gates */
    #[allow(non_snake_case)]
    pub fn CNOT ( &mut self, register_1_ind: usize, register_2_ind: usize ) {
        match &self.state[register_1_ind] {
            StateEntry::EntangledState(_) => { todo!(); },
            StateEntry::EntangledStatePtr(_) => { todo!()},
            StateEntry::StandardQubit(qubit_1) => {
                match &self.state[register_1_ind] {
                    StateEntry::EntangledState(_) => { todo!(); },
                    StateEntry::EntangledStatePtr(_) => { todo!()},
                    StateEntry::StandardQubit(qubit_2) => {
                        let tensor_product = qubit_1.state.clone().tensor_product(&qubit_2.state);
                        let gate = Matrix::new(
                            vec![
                                vec![ComplexNumber { a: 1f32, b: 0f32 }, ComplexNumber { a: 0f32, b: 0f32 }, ComplexNumber { a: 0f32, b: 0f32 }, ComplexNumber { a: 0f32, b: 0f32 }],
                                vec![ComplexNumber { a: 0f32, b: 0f32 }, ComplexNumber { a: 1f32, b: 0f32 }, ComplexNumber { a: 0f32, b: 0f32 }, ComplexNumber { a: 0f32, b: 0f32 }],
                                vec![ComplexNumber { a: 0f32, b: 0f32 }, ComplexNumber { a: 0f32, b: 0f32 }, ComplexNumber { a: 0f32, b: 0f32 }, ComplexNumber { a: 1f32, b: 0f32 }],
                                vec![ComplexNumber { a: 0f32, b: 0f32 }, ComplexNumber { a: 0f32, b: 0f32 }, ComplexNumber { a: 1f32, b: 0f32 }, ComplexNumber { a: 0f32, b: 0f32 }],
                            ]
                        );
                        println!("{:?} | {:?}", tensor_product, gate);
                        let final_result = gate.clone() * tensor_product.clone();
                        println!("{:?}", final_result);

                        self.state[register_1_ind] = StateEntry::EntangledState(final_result);
                        self.state[register_2_ind] = StateEntry::EntangledStatePtr(register_2_ind);
                    }
                }
            }
        }
    }
}