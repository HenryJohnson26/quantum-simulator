use crate::math::complex::Complex;
use bitvec::prelude::*;

pub struct BitMatrix {
    size: usize,
    data: BitVec<u64,Lsb0>,
}

impl BitMatrix {
    pub fn zero(size: usize) -> Self {
        Self{
            size,
            data: bitvec![u64, Lsb0: 0; size * size],
        }
    }

    pub fn set(&mut self, row: usize, col: usize, val: bool){
        self.data.set(row * self.size + col, val);
    }

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

impl CHForm {
    // Initialize |0^n>
    pub fn zero_state(n: usize) -> Self{
        Self{
            n,
            s: bitvce![u64, Lsb0; 0; n],
            h_layer: bitvec![u64, Lsb0; 0; n],
            v_matrix: BitMatrix::zero(n),
            g_matrix: BitMatrix::zero(n),
            phase: Complex::one(),
        }
    }

    pub fn apply_clifford(&mut self, gate: CliffordGate) {
        match gate {
            CliffordGate::H(q) => self.apply_h(q),
            CliffordGate::S(q) => self.apply_s(q),
            CliffordGate::CX(c, t) => self.apply_cs(c, t),
        }
    }

    pub fn apply_h(&mut self, q: usize){
        self.h_layer.toggle(q);
    }
    pub fn apply_s(&mut self, q: usize){
        self.g_matrix.set(q, q, self.g_matrix.get(q,q) ^ true);
    }
    pub fn apply_cx(&mut self, c: usize, t: usize){
        self.v_matrix.xor_rows(control, target);
    }
}

pub enum CliffordGate {
    CX(usize, usize),
    S(usize),
    H(usize),
}

pub struct ExpanstionTerm {
    coefficient: Complex,
    clifford: Vec<CliffordGate>,
}

pub struct StabilizerDecomposition {
    // Amplitudes b_alpha for each stabilizer state
    amplitudes: Vec<Complex>,
    // Collection of stabilizer states
    states: Vec<CHForm>,
    pub chi_max: usize,
}

impl StabilizerDecomposition{
    // Initialize |0^n> with /chi=1
    pub fn new(n: usize, chi_max: usize) -> Self{
        Self {
            amplitudes: vec![Complex::one()],
            states: vec![CHForm::zero_state(n)],
            chi_max,
        }
    }

    pub fn apply_clifford(&mut self, gate: CliffordGate) {
        for state in self.states {
            state.apply_clifford(gate);
        }
    }

    pub fn apply_non_clifford(&mut self, expansion: &[ExpanstionTerm]) {
        let mut new_states = Vec::with_capacity(self.states.len() * expansion.len());
        let mut new_amps = Vec::with_capacity(self.amplitudes.len() * expansion.len());

        // Expand: create sum over cliffords with associated amplitudes
        for (amp, state) in self.amplitudes.iter().zip(self.states.iter()) {
            for term in expansion{
                let mut new_state = state.clone();
                for &cgate in &term.clifford {
                    new_state.apply_clifford(cgate);
                }
                new_states.push(new_state);
                new_amps.push(amp.mul(&term.coefficient));
            }
        }
        self.states = new_states;
        self.amplitudes = new_amps;

        if self.states.len() > self.chi_max {
            self.sparsify();
        }
    }
    // Applies sparsification-lemma to reduce number of terms
    pub fn sparsify(&mut self, delta: f64){

    }
}
