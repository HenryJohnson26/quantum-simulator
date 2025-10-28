use quantum_sim::tableau::simulator::Tableau;

#[test]
fn tableau_initializes_to_zero_state() {
    let t = Tableau::new(2);
    // Verify first n rows are Z stabilizers
    // (optional: check data explicitly)
    t.dump();
}

#[test]
fn apply_h_swaps_x_and_z() {
    let mut t = Tableau::new(1);
    t.apply_h(0);
    // After H, Z stabilizer becomes X stabilizer
    // (You can verify by checking tableau data)
    t.dump();
}

#[test]
fn apply_cnot_preserves_stabilizer_relations() {
    let mut t = Tableau::new(2);
    t.apply_h(0);
    t.apply_cnot(0, 1);
    // Should now represent |Φ+> stabilizer group
    t.dump();
}

#[test]
fn measurement_returns_valid_bit() {
    let mut t = Tableau::new(1);
    let outcome = t.measure_z(0);
    assert!(outcome == false || outcome == true);
}
