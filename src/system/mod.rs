use crate::{
    matrix,
    complex,
    qubit
};
use qubit::Qubit;

pub struct System {
    state: Vec<Qubit>,
}
impl std::ops::Index<usize> for System {
    type Output = Qubit;

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
        for qubit in self.state {
            qubit.measure();
        }
    }
    pub fn new () -> Self {
        return System {
            state: Vec::new()
        }
    }

    pub fn allocate ( &mut self ) -> &mut Qubit {
        self.state.push(Qubit::new());
        self.state.last_mut().expect("Since we just allocated an element, it should be impossible to have no element")
    }
    pub fn allocate_ket ( &mut self, id: &str ) -> &mut Qubit {
        println!("You should complete this process manually!\nQubits start in state |0>.");
        self.state.push(Qubit::ket(id));
        self.state.last_mut().expect("Since we just allocated an element, it should be impossible to have no element")
    }
}