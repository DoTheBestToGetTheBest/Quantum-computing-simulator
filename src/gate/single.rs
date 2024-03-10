use crate::gate::double::Complex;
use crate::Qubit;


/// A representation of a quantum single gate with a matrix.
pub struct SingleGate {
    pub matrix: Vec<Vec<Complex>>,
}

/// A trait for objects that can apply a single gate.
pub trait SingleGateApplicator {
    /// Applies a single gate operation.
    fn apply_single(&mut self, matrix: &Vec<Vec<Complex>>, qubit: &Qubit);
}

impl SingleGate {
    /// Creates a Hadamard gate.
    pub fn h() -> Self {
        let sqrt_2_inv = (2f64).sqrt().recip();
        Self {
            matrix: vec![
                vec![Complex::new(sqrt_2_inv, 0.0), Complex::new(sqrt_2_inv, 0.0)],
                vec![
                    Complex::new(sqrt_2_inv, 0.0),
                    Complex::new(-sqrt_2_inv, 0.0),
                ],
            ],
        }
    }

    /// Creates a Pauli-X (NOT) gate.
    pub fn x() -> Self {
        Self {
            matrix: vec![
                vec![Complex::new(0.0, 0.0), Complex::new(1.0, 0.0)],
                vec![Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)],
            ],
        }
    }

    /// Creates a Pauli-Y gate.
    pub fn y() -> Self {
        Self {
            matrix: vec![
                vec![Complex::new(0.0, 0.0), Complex::new(0.0, -1.0)],
                vec![Complex::new(0.0, 1.0), Complex::new(0.0, 0.0)],
            ],
        }
    }

    /// Creates a Pauli-Z gate.
    pub fn z() -> Self {
        Self {
            matrix: vec![
                vec![Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)],
                vec![Complex::new(0.0, 0.0), Complex::new(-1.0, 0.0)],
            ],
        }
    }

    /// Creates an Identity gate.
    pub fn id() -> Self {
        Self {
            matrix: vec![
                vec![Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)],
                vec![Complex::new(0.0, 0.0), Complex::new(1.0, 0.0)],
            ],
        }
    }

    /// Creates a SQNOT gate.
    pub fn sqnot() -> Self {
        let half = 0.5;
        Self {
            matrix: vec![
                vec![Complex::new(half, half), Complex::new(half, -half)],
                vec![Complex::new(half, half), Complex::new(half, -half)],
            ],
        }
    }

    /// Creates a phase shift gate.
    pub fn phase(phi: f64) -> Self {
        Self {
            matrix: vec![
                vec![Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)],
                vec![Complex::new(0.0, 0.0), Complex::new(phi.cos(), phi.sin())],
            ],
        }
    }
}
