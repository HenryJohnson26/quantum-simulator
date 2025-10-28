use crate::compiler::ir::IROp;
use crate::runtime::{tableau_backend::TableauSimulator, statevector_backend::StatevectorSimulator};

pub enum BackendType {
    Tableau,
    Statevector,
    RankDecomposition,
}

pub struct RuntimeController {
    backend_type: BackendType,
    tableau: Option<TableauSimulator>,
    statevector: Option<StatevectorSimulator>,
}

impl RuntimeController {
    pub fn new(num_qubits: usize) -> Self {
        Self {
            backend_type: BackendType::Tableau,
            tableau: Some(TableauSimulator::new(num_qubits)),
            statevector: None,
        }
    }

    pub fn execute(&mut self, ops: &[IROp]) {
        for op in ops {
            match op {
                IROp::H(q) | IROp::S(q) | IROp::CNOT(_, _) => self.apply_clifford(op),
                IROp::T(q) | IROp::RZ(q, _) => self.handle_nonclifford(op),
                _ => {}
            }
        }
    }

    fn apply_clifford(&mut self, op: &IROp) {
        if let BackendType::Tableau = self.backend_type {
            self.tableau.as_mut().unwrap().apply_ir(op);
        } else {
            self.statevector.as_mut().unwrap().apply_ir(op);
        }
    }

    fn handle_nonclifford(&mut self, op: &IROp) {
        if matches!(self.backend_type, BackendType::Tableau) {
            self.promote_to_rank_decomposition();
        }
        self.statevector.as_mut().unwrap().apply_ir(op);
    }

    fn promote_to_rank_decomposition(&mut self) {
        // Convert tableau stabilizers into statevector amplitudes
        let vec = self.tableau.as_ref().unwrap().to_statevector();
        self.statevector = Some(StatevectorSimulator::from(vec));
        self.tableau = None;
        self.backend_type = BackendType::RankDecomposition;
    }

    fn maybe_demote(&mut self) {
        // TODO: heuristic based on gate locality / entanglement entropy
    }
}
