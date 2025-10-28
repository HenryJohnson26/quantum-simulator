use crate::math::complex::Complex;


pub struct StatevectorSimulator {
    pub num_qubits: usize,
    pub state: Vec<Complex>, // length = 2^num_qubits
}

impl StatevectorSimulator {
    pub fn new(num_qubits: usize) -> Self {
        let mut state = vec![Complex::zero(); 1 << num_qubits];
        state[0] = Complex::one(); // |0...0>
        Self { num_qubits, state }
    }

    pub fn apply_single_qubit_gate(&mut self, qubit: usize, matrix: [[Complex; 2]; 2]) {
        //stub: loop over amplitudes, apply gate to pairs
        let stride = 1 << qubit;
        for blockstart in (0..self.state.len()).step_by(2*stride){
          for i in 0..stride {
            let idx0 = blockstart + i;
            let idx1 = idx0 + stride;

            //read complex
            let a0 = self.state[idx0];
            let a1 = self.state[idx1];

            //apply gate
            let new_a0 = a0.mul(&matrix[0][0]).add(&a1.mul(&matrix[0][1]));
            let new_a1 =a0.mul(&matrix[1][0]).add(&a1.mul(&matrix[1][1]));

            self.state[idx0] = new_a0;
            self.state[idx1] = new_a1;
          }
        }

    }

    pub fn apply_controlled_gate<F>(&mut self, control: usize, target: usize, op: F)
    where F: Fn(&mut Complex, &mut Complex) {
        // stub: conditional amplitude swap/update
        let stride = 1 << target;
        for blockstart in (0..self.state.len()).step_by(2*stride){
          for i in 0..stride {
            let idx0 = blockstart + i;
            let idx1 = idx0 + stride;

            if(i >> control)&1 == 1{
              let (left, right) = self.state.split_at_mut(idx1);
              op(&mut left[idx0], &mut right[0]);
            }
          }
        }
    }

    pub fn measure_all(&self) -> Vec<f64> {
        self.state.iter().map(|c| c.magnitude2()).collect()
    }

    pub fn dump_state(&self) {
        for (i, amp) in self.state.iter().enumerate() {
            println!("{:0width$b}: {:?}", i, amp, width = self.num_qubits);
        }
    }

    pub fn apply_cnot(&mut self, control: usize, target: usize) {
        // TODO: iterate over all basis indices, if control bit == 1, flip target bit
        // This is a direct amplitude swap implementation.
        let stride = 1 << target;
        for blockstart in (0..self.state.len()).step_by(2*stride){
          for i in 0..stride {
            let idx0 = blockstart + i;
            let idx1 = idx0 + stride;

            if(idx0 >> control)&1 == 1{
              self.state.swap(idx0,idx1)
            }
          }
        }
    }

    pub fn apply_toffoli(&mut self, control1: usize, control2: usize, target: usize) {
      let stride = 1 << target;
      for blockstart in (0..self.state.len()).step_by(2*stride){
          for i in 0..stride {
              let idx0 = blockstart + i;
              let idx1 = idx0 + stride;
              // Check control bits against the actual basis state index (idx0)
              if ((idx0 >> control1) & 1 == 1) && ((idx0 >> control2) & 1 == 1) {
                  self.state.swap(idx0, idx1);
              }
          }
      }
    }

}
