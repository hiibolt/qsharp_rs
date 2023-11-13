use crate::{
    matrix,
    complex,
    qubit
};
use qubit::Qubit;
use matrix::Matrix;
use matrix::Gate;
use matrix::Gates;
use complex::ComplexNumber;

pub struct System {
    state: Vec<StateEntry>,
}
#[derive(Debug)]
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
        println!("{:?}", self.state[self.state.len() - 1]);
        match &self.state[self.state.len() - 1] {
            StateEntry::StandardQubit(q) => {
                base = q.state.clone();
            },
            StateEntry::EntangledState(s) => {
                base = s.clone();
            }
            StateEntry::EntangledStatePtr(_) => panic!("Impossible")
        }
        println!("Base: {:?}", base);
        for i in 1..self.state.len() {
            println!("{}: {:?}", self.state.len() - i - 1, self.state[self.state.len() - i - 1]);
            match &self.state[self.state.len() - i - 1] {
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

    /* Gate Build and Application Logic */
    pub fn build_gate( &self, inputs: Vec<(usize, Matrix)> ) -> Matrix {
        let mut current_idx: usize = 0usize;
        let mut base_matrix = Matrix::new(vec![vec![ComplexNumber { a: 1f32, b: 0f32 }]]);
        
        for (idx, matrix) in inputs {
            while idx > current_idx {
                base_matrix = base_matrix.tensor_product( &Gates::I() );
                current_idx += 1;
            }

            base_matrix = base_matrix.tensor_product( &matrix );
            current_idx += 1;
        }

        while current_idx < self.state.len() {
            let to_mul: &Matrix = &Gates::I();

            base_matrix = to_mul.tensor_product( &base_matrix );

            current_idx += 1;
        }

        base_matrix
    }

    /* - MULTI-QUBIT GATES - */
    // SWAP - 'Switch Q_1 and Q_2'
    #[allow(non_snake_case)]
    pub fn SWAP ( &mut self, register_1_ind: usize, register_2_ind: usize ) {
        let qubit_1_state: Matrix = self.state[register_1_ind].unwrap_qubit().state.clone();
        let qubit_2_state: Matrix = self.state[register_2_ind].unwrap_qubit().state.clone();

        let tensor_product = qubit_1_state.tensor_product(&qubit_2_state);
        let gate = Matrix::new(
            vec![
                vec![ComplexNumber { a: 1f32, b: 0f32 }, ComplexNumber { a: 0f32, b: 0f32 }, ComplexNumber { a: 0f32, b: 0f32 }, ComplexNumber { a: 0f32, b: 0f32 }],
                vec![ComplexNumber { a: 0f32, b: 0f32 }, ComplexNumber { a: 0f32, b: 0f32 }, ComplexNumber { a: 1f32, b: 0f32 }, ComplexNumber { a: 0f32, b: 0f32 }],
                vec![ComplexNumber { a: 0f32, b: 0f32 }, ComplexNumber { a: 1f32, b: 0f32 }, ComplexNumber { a: 0f32, b: 0f32 }, ComplexNumber { a: 0f32, b: 0f32 }],
                vec![ComplexNumber { a: 0f32, b: 0f32 }, ComplexNumber { a: 0f32, b: 0f32 }, ComplexNumber { a: 0f32, b: 0f32 }, ComplexNumber { a: 1f32, b: 0f32 }],
            ]
        );
        let final_result = gate.clone() * tensor_product.clone();

        self.state[register_1_ind] = StateEntry::EntangledState(final_result);
        self.state[register_2_ind] = StateEntry::EntangledStatePtr(register_1_ind);
    }

    /* - CONTROLLED STANDARD GATES - */
    // CNOT - 'Swap Q_2 if Q_1' 
    #[allow(non_snake_case)]
    pub fn CNOT ( &mut self, register_1_ind: usize, register_2_ind: usize ) {
        let qubit_1_state: Matrix = self.state[register_1_ind].unwrap_qubit().state.clone();
        let qubit_2_state: Matrix = self.state[register_2_ind].unwrap_qubit().state.clone();

        let tensor_product = qubit_1_state.tensor_product(&qubit_2_state);
        let gate = Matrix::new(
            vec![
                vec![ComplexNumber { a: 1f32, b: 0f32 }, ComplexNumber { a: 0f32, b: 0f32 }, ComplexNumber { a: 0f32, b: 0f32 }, ComplexNumber { a: 0f32, b: 0f32 }],
                vec![ComplexNumber { a: 0f32, b: 0f32 }, ComplexNumber { a: 1f32, b: 0f32 }, ComplexNumber { a: 0f32, b: 0f32 }, ComplexNumber { a: 0f32, b: 0f32 }],
                vec![ComplexNumber { a: 0f32, b: 0f32 }, ComplexNumber { a: 0f32, b: 0f32 }, ComplexNumber { a: 0f32, b: 0f32 }, ComplexNumber { a: 1f32, b: 0f32 }],
                vec![ComplexNumber { a: 0f32, b: 0f32 }, ComplexNumber { a: 0f32, b: 0f32 }, ComplexNumber { a: 1f32, b: 0f32 }, ComplexNumber { a: 0f32, b: 0f32 }],
            ]
        );
        let final_result = gate.clone() * tensor_product.clone();

        self.state[register_1_ind] = StateEntry::EntangledState(final_result);
        self.state[register_2_ind] = StateEntry::EntangledStatePtr(register_1_ind);
    }
    pub fn CONTROLLED_X ( &mut self, control_idx: usize, target_idx: usize ) -> &Self {
        let control: Matrix = self[control_idx].unwrap_qubit().state.clone();
        let target: Matrix = self[target_idx].unwrap_qubit().state.clone();
        
        let control_percentage: f32 = control[1][0].a.powi(2) + control[1][0].b.powi(2); 
        println!("Attempting Controlled X with {:?} ({}%) as control on {:?}", control, control_percentage, target);

        let mut identity_portion: Matrix = Gates::I();
        for row in 0..identity_portion.value.len() {
            for col in 0..identity_portion[row].len() {
                identity_portion.value[row][col] *= (1f32 - control_percentage).sqrt();
            }
        }
        println!("Identity Portion: {:?}", identity_portion);

        let mut gate_portion: Matrix = Gates::X();
        for row in 0..gate_portion.value.len() {
            for col in 0..gate_portion[row].len() {
                gate_portion[row][col] *= control_percentage.sqrt();
            }
        }
        println!("Gate Portion: {:?}", gate_portion);

        self[target_idx].unwrap_qubit().state = (identity_portion + gate_portion) * self[target_idx].unwrap_qubit().state.clone();
        self
    }

    pub fn apply ( &mut self, gates: Vec<(usize, Gate)> ) -> &mut Self {
        let mut control_percentage = 1f32;
        let mut swap_idx: Option<usize> = None;
        for (qubit_idx, gate) in gates {
            match gate {
                Gate::Control => {
                    let control = self[qubit_idx].unwrap_qubit().state.clone();
                    control_percentage *= control[1][0].a.powi(2) + control[1][0].b.powi(2); 
                },
                Gate::AntiControl => {
                    let control = self[qubit_idx].unwrap_qubit().state.clone();
                    control_percentage *= 1f32 - (control[1][0].a.powi(2) + control[1][0].b.powi(2)); 
                },
                Gate::Swap(incoming_idx) => {
                    /*
                    if let Some(outgoing_idx) = swap_idx {
                        let 
                    } else { 
                        swap_idx = Some(incoming_idx);
                    }*/
                },
                Gate::Standard(gate_matrix) => {
                    let mut identity_portion: Matrix = Gates::I();
                    let mut gate_portion: Matrix = gate_matrix.clone();
                    
                    for row in 0..identity_portion.value.len() {
                        for col in 0..identity_portion[row].len() {
                            identity_portion.value[row][col] *= (1f32 - control_percentage).sqrt();
                        }
                    }
                    
                    for row in 0..gate_portion.value.len() {
                        for col in 0..gate_portion[row].len() {
                            gate_portion[row][col] *= control_percentage.sqrt();
                        }
                    }

                    println!("Applying to qubit register {} with control: {} - {:?}", qubit_idx, control_percentage, gate_matrix);
                    self[qubit_idx].unwrap_qubit().apply( identity_portion + gate_portion );
                    self.dump_register(qubit_idx);
                }
            }
        }
        self
    }

    // CR_x - 'Controlled R_x'
    #[allow(non_snake_case)]
    pub fn CR_x ( &mut self, control: Vec<usize>, args: (usize, f32) ) {
        
    }
}
