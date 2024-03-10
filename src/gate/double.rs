use std::ops::Neg;

use crate::Qubit;

///  complex number.
#[derive(Debug, Clone, Copy)]
pub struct Complex {
    pub re: f64,
    pub im: f64,
}
impl Neg for Complex {
    type Output = Complex;

    fn neg(self) -> Self::Output {
        Complex {
            re: -self.re,
            im: -self.im,
        }
    }
}

impl Complex {
    /// Creates a new complex number.
    pub fn new(re: f64, im: f64) -> Self {
        Self { re, im }
    }

    /// Returns the complex conjugate of the complex number.
    pub fn conj(self) -> Self {
        Self {
            re: self.re,
            im: -self.im,
        }
    }
}

impl std::ops::Add for Complex {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            re: self.re + other.re,
            im: self.im + other.im,
        }
    }
}

impl std::ops::Mul for Complex {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self {
            re: self.re * other.re - self.im * other.im,
            im: self.re * other.im + self.im * other.re,
        }
    }
}

/// A representation of a quantum double gate with a matrix.
pub struct DoubleGate {
    matrix: Vec<Vec<Complex>>,
}

/// A trait for objects that can apply a double gate.
pub trait DoubleGateApplicator {
    /// Applies a double gate operation.
    fn apply_double(&mut self, matrix: &Vec<Vec<Complex>>, qubit1: &Qubit, qubit2: &Qubit);
}

impl DoubleGate {
    /// Creates a CNOT gate.
    pub fn cnot() -> Self {
        Self {
            matrix: vec![
                vec![
                    Complex::new(1.0, 0.0),
                    Complex::new(0.0, 0.0),
                    Complex::new(0.0, 0.0),
                    Complex::new(0.0, 0.0),
                ],
                vec![
                    Complex::new(0.0, 0.0),
                    Complex::new(1.0, 0.0),
                    Complex::new(0.0, 0.0),
                    Complex::new(0.0, 0.0),
                ],
                vec![
                    Complex::new(0.0, 0.0),
                    Complex::new(0.0, 0.0),
                    Complex::new(0.0, 0.0),
                    Complex::new(1.0, 0.0),
                ],
                vec![
                    Complex::new(0.0, 0.0),
                    Complex::new(0.0, 0.0),
                    Complex::new(1.0, 0.0),
                    Complex::new(0.0, 0.0),
                ],
            ],
        }
    }

    /// Creates a SWAP gate.
    pub fn swap() -> Self {
        Self {
            matrix: vec![
                vec![
                    Complex::new(1.0, 0.0),
                    Complex::new(0.0, 0.0),
                    Complex::new(0.0, 0.0),
                    Complex::new(0.0, 0.0),
                ],
                vec![
                    Complex::new(0.0, 0.0),
                    Complex::new(0.0, 0.0),
                    Complex::new(1.0, 0.0),
                    Complex::new(0.0, 0.0),
                ],
                vec![
                    Complex::new(0.0, 0.0),
                    Complex::new(1.0, 0.0),
                    Complex::new(0.0, 0.0),
                    Complex::new(0.0, 0.0),
                ],
                vec![
                    Complex::new(0.0, 0.0),
                    Complex::new(0.0, 0.0),
                    Complex::new(0.0, 0.0),
                    Complex::new(1.0, 0.0),
                ],
            ],
        }
    }

    /// Creates a SQSWAP gate.
    pub fn sqswap() -> Self {
        let i = Complex::new(0.0, 1.0);
        Self {
            matrix: vec![
                vec![
                    Complex::new(1.0, 0.0),
                    Complex::new(0.0, 0.0),
                    Complex::new(0.0, 0.0),
                    Complex::new(0.0, 0.0),
                ],
                vec![
                    Complex::new(0.0, 0.0),
                    Complex::new(0.5, 0.0),
                    Complex::new(0.5, 0.0),
                    Complex::new(0.0, 0.0),
                ],
                vec![
                    Complex::new(0.0, 0.0),
                    Complex::new(0.5, 0.0),
                    Complex::new(0.5, 0.0),
                    Complex::new(0.0, 0.0),
                ],
                vec![
                    Complex::new(0.0, 0.0),
                    Complex::new(0.0, 0.0),
                    Complex::new(0.0, 0.0),
                    Complex::new(1.0, 0.0),
                ],
            ],
        }
        .add_imag(vec![
            vec![
                Complex::new(0.0, 0.0),
                Complex::new(0.0, 0.0),
                Complex::new(0.0, 0.0),
                Complex::new(0.0, 0.0),
            ],
            vec![
                Complex::new(0.0, 0.0),
                i * Complex::new(0.5, 0.0),
                -i * Complex::new(0.5, 0.0),
                Complex::new(0.0, 0.0),
            ],
            vec![
                Complex::new(0.0, 0.0),
                -i * Complex::new(0.5, 0.0),
                i * Complex::new(0.5, 0.0),
                Complex::new(0.0, 0.0),
            ],
            vec![
                Complex::new(0.0, 0.0),
                Complex::new(0.0, 0.0),
                Complex::new(0.0, 0.0),
                Complex::new(0.0, 0.0),
            ],
        ])
    }

    ///  imaginary parts to the gate matrix.
    fn add_imag(mut self, imag_matrix: Vec<Vec<Complex>>) -> Self {
        for (r, row) in imag_matrix.into_iter().enumerate() {
            for (c, val) in row.into_iter().enumerate() {
                self.matrix[r][c] = self.matrix[r][c] + val;
            }
        }
        self
    }
}
