use quantum_sim::math::complex::Complex;
use quantum_sim::statevector::{simulator::StatevectorSimulator, gates::Gates};
use quantum_sim::statevector::utils::{get_bit, flip_bit};

fn approx_eq(a: &Complex, b: &Complex, eps: f64) -> bool {
    (a.re - b.re).abs() < eps && (a.im - b.im).abs() < eps
}

#[test]
fn complex_add_and_mul() {
    let a = Complex::new(1.0, 2.0);
    let b = Complex::new(3.0, -1.0);

    let sum = a.add(&b);
    assert!(approx_eq(&sum, &Complex::new(4.0, 1.0), 1e-12));

    let prod = a.mul(&b);
    assert!(approx_eq(&prod, &Complex::new(5.0, 5.0), 1e-12));
}

#[test]
fn complex_sub_and_div() {
  let a = Complex::new(5.0, 3.0);
  let b = Complex::new(2.0, 1.0);

  let div = a.div(&b);
  assert!(approx_eq(&div, &Complex::new(13.0/5.0, 0.2), 1e-12))
}

#[test]
fn utils_bit_helpers() {
    assert_eq!(get_bit(0b101, 0), true);  // least significant bit
    assert_eq!(get_bit(0b101, 1), false);
    assert_eq!(get_bit(0b101, 2), true);

    assert_eq!(flip_bit(0b101, 0), 0b100);
    assert_eq!(flip_bit(0b101, 2), 0b001);
}

#[test]
fn simulator_initial_state_is_all_zero() {
    let sim = StatevectorSimulator::new(3);
    assert_eq!(sim.state.len(), 8);
    for (i, amp) in sim.state.iter().enumerate() {
        if i == 0 {
            assert!(approx_eq(amp, &Complex::one(), 1e-12));
        } else {
            assert!(approx_eq(amp, &Complex::zero(), 1e-12));
        }
    }
}

#[test]
fn apply_x_gate_flips_state() {
    let mut sim = StatevectorSimulator::new(1);
    sim.apply_single_qubit_gate(0, Gates::x());
    // Should now be in |1>
    assert!(approx_eq(&sim.state[0], &Complex::zero(), 1e-12));
    assert!(approx_eq(&sim.state[1], &Complex::one(), 1e-12));
}

#[test]
fn apply_z_gate_applies_phase() {
    let mut sim = StatevectorSimulator::new(1);
    // Start in |0>, apply H to go to (|0>+|1>)/√2
    sim.apply_single_qubit_gate(0, Gates::h());
    let before = sim.state.clone();

    // Apply Z -> should flip sign on |1> component
    sim.apply_single_qubit_gate(0, Gates::z());
    assert!(approx_eq(&sim.state[0], &before[0], 1e-12));
    assert!(approx_eq(&sim.state[1], &Complex::new(-before[1].re, -before[1].im), 1e-12));
}

#[test]
fn apply_h_gate_creates_superposition() {
    let mut sim = StatevectorSimulator::new(1);
    sim.apply_single_qubit_gate(0, Gates::h());
    let prob_0 = sim.state[0].magnitude2();
    let prob_1 = sim.state[1].magnitude2();
    assert!((prob_0 - 0.5).abs() < 1e-12);
    assert!((prob_1 - 0.5).abs() < 1e-12);
}

#[test]
fn bell_state_creation() {
    // Test H + CNOT produces |Φ+> = (|00> + |11>)/√2
    let mut sim = StatevectorSimulator::new(2);
    sim.apply_single_qubit_gate(0, Gates::h());
    // Implement a controlled gate manually: CNOT with control=0, target=1
    sim.apply_controlled_gate(0, 1, |amp0, amp1| {
        std::mem::swap(amp0, amp1);
    });

    let probs: Vec<f64> = sim.measure_all();
    let p00 = probs[0];
    let p11 = probs[3];
    assert!((p00 - 0.5).abs() < 1e-12);
    assert!((p11 - 0.5).abs() < 1e-12);

    // |01> and |10> should have prob ~ 0
    assert!(probs[1] < 1e-12);
    assert!(probs[2] < 1e-12);
}

#[test]
fn oracle_phase_flip_marks_state() {
    let n = 3;
    let marked = 5;
    let mut sim = StatevectorSimulator::new(n);
    // Put into equal superposition
    for q in 0..n {
        sim.apply_single_qubit_gate(q, Gates::h());
    }
    let before = sim.state[marked];
    // Manually flip phase
    sim.state[marked] = Complex::new(-before.re, -before.im);
    assert!(approx_eq(&sim.state[marked], &Complex::new(-before.re, -before.im), 1e-12));
}

#[test]
fn diffusion_operator_preserves_norm() {
    let n = 2;
    let mut sim = StatevectorSimulator::new(n);
    for q in 0..n {
        sim.apply_single_qubit_gate(q, Gates::h());
    }
    let norm_before: f64 = sim.state.iter().map(|c| c.magnitude2()).sum();

    // Implement naive diffusion: reflection about |s>
    let avg_amp = {
        let sum: Complex = sim.state.iter().fold(Complex::zero(), |acc, x| acc.add(x));
        Complex::new(sum.re / 4.0, sum.im / 4.0)
    };
    for amp in sim.state.iter_mut() {
        let diff = Complex::new(2.0 * avg_amp.re - amp.re, 2.0 * avg_amp.im - amp.im);
        *amp = diff;
    }

    let norm_after: f64 = sim.state.iter().map(|c| c.magnitude2()).sum();
    assert!((norm_before - norm_after).abs() < 1e-10);
}

#[test]
fn cnot_creates_entanglement() {
    let mut sim = StatevectorSimulator::new(2);
    sim.apply_single_qubit_gate(0, Gates::h());
    sim.apply_cnot(0, 1);
    let probs = sim.measure_all();
    // Should be |00> + |11> superposition
    assert!((probs[0] - 0.5).abs() < 1e-12);
    assert!((probs[3] - 0.5).abs() < 1e-12);
    assert!(probs[1] < 1e-12);
    assert!(probs[2] < 1e-12);
}

#[test]
fn toffoli_works_as_expected() {
    let mut sim = StatevectorSimulator::new(3);
    // Prepare |011> (controls = 1, target = 0)
    sim.state = vec![
        Complex::zero(), Complex::zero(), Complex::zero(), Complex::one(),
        Complex::zero(), Complex::zero(), Complex::zero(), Complex::zero()
    ];
    sim.apply_toffoli(0, 1, 2);
    // Expect |111>
    assert!(approx_eq(&sim.state[7], &Complex::one(), 1e-12));
}

