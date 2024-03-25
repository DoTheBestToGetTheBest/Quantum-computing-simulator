pub mod gate;
pub mod simulator;
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Qubit {
    pub index: usize,
}
