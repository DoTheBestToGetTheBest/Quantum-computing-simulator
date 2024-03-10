use crate::gate::double::Complex;
use crate::Qubit;
/// A representation of a quantum triple gate with a matrix.
pub struct TripleGate {
    pub matrix: Vec<Vec<Complex>>,
}

/// A trait for objects that can apply a triple gate.
pub trait TripleGateApplicator {
    /// Applies a triple gate operation.
    fn apply_triple(
        &mut self,
        matrix: &Vec<Vec<Complex>>,
        qubit1: &Qubit,
        qubit2: &Qubit,
        qubit3: &Qubit,
    );
}

impl TripleGate {
    /// Creates a CCNOT (Toffoli) gate.
    pub fn ccnot() -> Self {
        let mut matrix = vec![vec![Complex::new(0.0, 0.0); 8]; 8];
        for i in 0..8 {
            matrix[i][i] = Complex::new(1.0, 0.0);
        }
        // Flip the last two bits
        matrix[6][6] = Complex::new(0.0, 0.0);
        matrix[6][7] = Complex::new(1.0, 0.0);
        matrix[7][7] = Complex::new(0.0, 0.0);
        matrix[7][6] = Complex::new(1.0, 0.0);

        Self { matrix }
    }

    /// Creates a CSWAP (Fredkin) gate.
    pub fn cswap() -> Self {
        let mut matrix = vec![vec![Complex::new(0.0, 0.0); 8]; 8];
        for i in 0..8 {
            matrix[i][i] = Complex::new(1.0, 0.0);
        }
        // Swap the last two qubits conditioned on the first qubit
        matrix[5][5] = Complex::new(0.0, 0.0);
        matrix[5][6] = Complex::new(1.0, 0.0);
        matrix[6][6] = Complex::new(0.0, 0.0);
        matrix[6][5] = Complex::new(1.0, 0.0);

        Self { matrix }
    }
}
