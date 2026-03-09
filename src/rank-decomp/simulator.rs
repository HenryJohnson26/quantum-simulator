use crate::math::complex::Complex;
use bitvec::prelude::*;
use rand::Rng;
use rand::distributions::WeightedIndex;
use rand::prelude::Distribution;

pub struct BitMatrix {
    size: usize,
    data: BitVec<u64, Lsb0>,
}

impl BitMatrix {
    pub fn zero(size: usize) -> Self {
        Self {
            size,
            data: bitvec![u64, Lsb0; 0; size * size],
        }
    }

    pub fn set(&mut self, row: usize, col: usize, val: bool) {
        self.data.set(row * self.size + col, val);
    }

    pub fn get(&self, row: usize, col: usize) -> bool {
        self.data[row * self.size + col]
    }

    pub fn xor_rows(&mut self, src: usize, dst: usize) {
        for col in 0..self.size {
            let src_bit = self.data[src * self.size + col];
            let dst_idx = dst * self.size + col;
            self.data.set(dst_idx, self.data[dst_idx] ^ src_bit);
        }
    }
}

impl Clone for BitMatrix {
    fn clone(&self) -> Self {
        Self {
            size: self.size,
            data: self.data.clone(),
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
    v_matrix: BitMatrix, // O(n^2) bit matrix for CNOT
    g_matrix: BitMatrix, // Upper triangular matrix for CZ and S gates
    phase: Complex,      // Global phase
}

impl CHForm {
    // Initialize |0^n>
    pub fn zero_state(n: usize) -> Self {
        Self {
            n,
            s: bitvec![u64, Lsb0; 0; n],
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
            CliffordGate::CX(c, t) => self.apply_cx(c, t),
        }
    }

    pub fn apply_h(&mut self, q: usize) {
        self.h_layer.set(q, !self.h_layer[q]);
    }

    pub fn apply_s(&mut self, q: usize) {
        self.g_matrix.set(q, q, self.g_matrix.get(q, q) ^ true);
    }

    pub fn apply_cx(&mut self, c: usize, t: usize) {
        self.v_matrix.xor_rows(c, t);
    }
}

impl Clone for CHForm {
    fn clone(&self) -> Self {
        Self {
            n: self.n,
            s: self.s.clone(),
            h_layer: self.h_layer.clone(),
            v_matrix: self.v_matrix.clone(),
            g_matrix: self.g_matrix.clone(),
            phase: self.phase,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum CliffordGate {
    CX(usize, usize),
    S(usize),
    H(usize),
}

pub struct ExpansionTerm {
    pub coefficient: Complex,
    pub clifford: Vec<CliffordGate>,
}

pub struct StabilizerDecomposition {
    // Amplitudes b_alpha for each stabilizer state
    amplitudes: Vec<Complex>,
    // Collection of stabilizer states
    states: Vec<CHForm>,
    pub chi_max: usize,
}

impl StabilizerDecomposition {
    // Initialize |0^n> with chi=1
    pub fn new(n: usize, chi_max: usize) -> Self {
        Self {
            amplitudes: vec![Complex::one()],
            states: vec![CHForm::zero_state(n)],
            chi_max,
        }
    }

    // Compute L1 norm of amplitudes: ||c||_1 = Σ|c_α|
    pub fn l1_norm(&self) -> f64 {
        self.amplitudes
            .iter()
            .map(|c| c.magnitude())// |c| = sqrt(|c|^2)
            .sum()
    }

    pub fn apply_clifford(&mut self, gate: CliffordGate) {
        for state in &mut self.states {
            state.apply_clifford(gate);
        }
    }

    pub fn apply_non_clifford(&mut self, expansion: &[ExpansionTerm]) {  // Fixed typo
        let mut new_states = Vec::with_capacity(self.states.len() * expansion.len());
        let mut new_amps = Vec::with_capacity(self.amplitudes.len() * expansion.len());

        // Expand: create sum over cliffords with associated amplitudes
        for (amp, state) in self.amplitudes.iter().zip(self.states.iter()) {
            for term in expansion {
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
            self.sparsify(0.01); // Need to provide delta parameter
        }
    }

    // Applies sparsification lemma to reduce number of terms
    pub fn sparsify(&mut self, delta: f64) {
        if self.states.is_empty() {
            return;
        }

        // Compute L1 norm: ||c||_1 = Σ|c_α|
        let l1_norm = self.l1_norm();

        // Determine target rank k using Theorem 1: χ_δ(ψ) ≤ 1 + ||c||_1^2 / δ^2
        let k_target = ((l1_norm * l1_norm) / (delta * delta)).ceil() as usize;
        let k = k_target.clamp(1, self.chi_max);  // More idiomatic than .min().max()

        // If already below target rank, we don't need to do anything
        if self.states.len() <= k {
            return;
        }

        // Compute sampling probabilities: p_j = |c_j| / ||c||_1
        let probabilities: Vec<f64> = self
            .amplitudes
            .iter()
            .map(|c| c.magnitude() / l1_norm)  // |c| not |c|^2
            .collect();

        // Sample k stabilizer states according to probabilities
        let mut rng = rand::thread_rng();
        let dist = WeightedIndex::new(&probabilities)
            .expect("Failed to create weighted distribution");

        let mut new_states = Vec::with_capacity(k);

        for _ in 0..k {
            let idx = dist.sample(&mut rng);
            new_states.push(self.states[idx].clone());
        }

        // Set new amplitudes to ||c||_1 / k (equal weights)
        // This gives |Ω⟩ = (||c||_1 / k) Σ_α |ω_α⟩
        let new_amplitude = Complex::new(l1_norm / (k as f64), 0.0);
        let new_amplitudes = vec![new_amplitude; k];

        self.states = new_states;
        self.amplitudes = new_amplitudes;
    }
}
