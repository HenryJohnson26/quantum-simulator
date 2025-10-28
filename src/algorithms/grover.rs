use crate::statevector::{simulator::StatevectorSimulator, gates::Gates};
use crate::math::complex::Complex;

pub fn run_demo() {
    let n = 3;
    let marked = 5;
    let mut sim = StatevectorSimulator::new(n);

    // Step 1: apply Hadamard to all qubits
    for q in 0..n {
        sim.apply_single_qubit_gate(q, Gates::h());
    }

    // Step 2: apply oracle (phase flip on |marked>)
    apply_oracle(&mut sim, marked);

    // Step 3: apply diffusion operator
    apply_diffusion(&mut sim);

    sim.dump_state();
}

fn apply_oracle(sim: &mut StatevectorSimulator, marked: usize) {
    // stub: multiply amplitude at `marked` index by -1
    sim.state[marked] = sim.state[marked].mul(&Complex::new(-1.0,0.0));
}

fn apply_diffusion(sim: &mut StatevectorSimulator) {
    // stub: implement 2|s><s| - I
        let n = sim.state.len();
    let mut sum = Complex::zero();
    
    for amplitude in &sim.state {
        sum = sum.add(amplitude);
    }
    
    // Average amplitude = sum / sqrt(n) / sqrt(n) = sum / n
    // (each coefficient in <s| is 1/sqrt(n))
    let average = sum.div(&Complex::new(n as f64, 0.0));
    
    // Step 2: Apply the transformation: new_amp = 2*average - old_amp
    // This is the "inversion about average" operation
    for amplitude in &mut sim.state {
        // new_amplitude = 2 * average - old_amplitude
        let two_avg = average.mul(&Complex::new(2.0, 0.0));
        *amplitude = two_avg.sub(amplitude);
    }
}
