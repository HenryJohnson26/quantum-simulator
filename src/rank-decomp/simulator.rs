use crate::math::complex::Complex;
use bitvec::prelude::*;

pub struct BitMatrix {
    size: usize,
    data: BitVec<u64,Lsb0>,
}

impl BitMatrix {
    pub fn get(&self, row: usize, col: usize) -> bool{
        self.data[row*self.size+col]
    }

    pub fn xor_rows(&mut self, src: usize, dst: usize){
        for col i 0..self.size {
            let src_bit = self.data[src*self.size + col];
            lest dst_idx = dst * self.size + col;
            self.data.set(dst_idx, self.data[dst_idx] ^ src_bit);
        }
    }
}


pub struct CHForm {
    n: usize,
    // Computational basis state: |s>
    s: BitVec<u64, Lsb0>,
    // Hadamard stabilizer
    h_layer: BitVec<u64, Lsb0>,
    // Clifford layer U_C, stored as quadratic form(V, G)
    // where V is a binary matrix for CNOTs and G for CZ/S gates
    v_matrix: BitMatrix, // O(n^2) bit matrix fro CNOT
    g_matrix: BitMatrix, // Upper triangular matrix for CZ and S gates
    phase: Complex, // Global phase
}

pub enum CliffordGate {
    CX(usize, usize),
    S(usize),
    H(usize),
}

pub struct ExpanstionTerm {
    coefficient: Complex,
    clifford: CliffordGate,
}

pub struct StabilizerDecomposition {
    // Amplitudes b_alpha for each stabilizer state
    amplitudes: Vec<Complex>,
    // Collection of stabilizer states
    states: Vec<CHForm>,
}

impl StabilizerDecomposition{
    // Applies sparsification-lemma to reduce number of terms
    pub fn sparsify(&mut self, delta: f64){

    }
}
