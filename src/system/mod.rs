use crate::{
    matrix,
    complex,
    qubit
};
use qubit::Qubit;

pub struct System {
    state: Vec<Qubit>,
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
}