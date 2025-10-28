use quantum_sim::compiler::runtime::compile_and_run;

#[test]
fn hybrid_execution_switches_modes() {
    let src = "
        qubit q0, q1;
        h q0;
        cnot q0, q1;
        t q0;
        s q1;
        measure q0, q1;
    ";
    let result = compile_and_run(src);
    assert_eq!(result.len(), 2);
}
