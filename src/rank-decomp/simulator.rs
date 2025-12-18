use crate::math::complex::Complex;

pub struct RankDecompositionSimulator {
    pub num_qubits: usize,
    pub stabilizers: Vec<Vec<bool>>, // simplified representation
    pub coefficients: Vec<f64>,
}

pub struct CliffordTerm{
    pub coeff: Complex,
    pub gates: Vec<Gate>,
}

pub enum Gate{
    H(usize),
    S(usize),
    CNOT(usize,usize),
    CZ(usize,usize),
}


// multiply two CliffordTerm collections: outer product (used when composing gate expansions)
fn compose_terms(a: &[CliffordTerm], b: &[CliffordTerm]) -> Vec<CliffordTerm> {
    let mut out = Vec::with_capacity(a.len() * b.len());
    for ta in a {
        for tb in b {
            let mut gates = ta.gates.clone();
            gates.extend(tb.gates.iter().cloned()); // apply ta then tb
            let coeff = ta.coeff * tb.coeff;
            out.push(CliffordTerm { coeff, gates });
        }
    }
    out
}


impl RankDecompositionSimulator {
    pub fn approximate_nonclifford(&mut self, gate: &str) {
        // TODO: use a fixed stabilizer rank approximation for T or CCZ gates
    }

    pub fn combine_terms(&mut self) {
        // TODO: prune low-amplitude terms to control cost
    }
}
