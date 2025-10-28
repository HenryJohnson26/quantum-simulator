pub enum CliffordGate {
    H(usize),
    S(usize),
    CNOT(usize, usize),
}
