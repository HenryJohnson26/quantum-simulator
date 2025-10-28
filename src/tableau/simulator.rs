use rand::Rng;
use std::fmt;

pub struct Row{
    xmask: Vec<u64>,
    zmask: Vec<u64>,

    phase: bool,

}

impl fmt::Debug for Row {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Row {{ ")?;
        
        write!(f, "xmask: [")?;
        for (i, mask) in self.xmask.iter().enumerate() {
            if i > 0 { write!(f, ", ")?; }
            write!(f, "{:#016x}", mask)?; // Hexadecimal format for better readability
        }
        write!(f, "], ")?;
        
        write!(f, "zmask: [")?;
        for (i, mask) in self.zmask.iter().enumerate() {
            if i > 0 { write!(f, ", ")?; }
            write!(f, "{:#016x}", mask)?;
        }
        write!(f, "], ")?;
        
        write!(f, "phase: {} }}", self.phase)
    }
}
pub struct Tableau {
    num_qubits: usize,
    // Row-major: 2n rows, n+1 columns (X|Z|phase)
    // Could store as Vec<Vec<bool>>, but you might later pack into bitmasks for speed.
    data: Vec<Row>,
}

impl Tableau {
    pub fn new(num_qubits: usize) -> Self {
        let chunks = (num_qubits + 63) / 64;

        let mut data: Vec<Row> = (0..2 * num_qubits)
            .map(|_| Row {
                xmask: vec![0; chunks],
                zmask: vec![0; chunks],
                phase: false,
            })
            .collect();

        // Set the first n rows to Z stabilizers
        for q in 0..num_qubits {
            let chunk = q / 64;
            let bit = q % 64;
            data[q].zmask[chunk] |= 1 << bit;
        }

        Self { num_qubits, data }
    }

    pub fn apply_h(&mut self, qubit: usize) {
        let chunk = qubit / 64;
        let bit = qubit % 64;
        let mask = 1u64 << bit;
        
        for row in 0..(2 * self.num_qubits) {
            let x_set = (self.data[row].xmask[chunk] & mask) != 0;
            let z_set = (self.data[row].zmask[chunk] & mask) != 0;
            
            // Update phase if both X and Z are set
            if x_set && z_set {
                self.data[row].phase ^= true;
            }
            
            // Swap the bits
            if x_set != z_set { // Only need to change if they're different
                self.data[row].xmask[chunk] ^= mask;
                self.data[row].zmask[chunk] ^= mask;
            }
        }
    }

    pub fn apply_s(&mut self, qubit: usize) {
        let chunk = qubit / 64;
        let bit = qubit % 64;
        let mask = 1u64<<bit;

        for row in 0..(2*self.num_qubits){
            let x_set = (self.data[row].xmask[chunk] & mask) != 0;
            let z_set = (self.data[row].zmask[chunk] & mask) != 0;

            // (x=1,z=0) -> (x=1,z=1)
            if x_set && !z_set{
                self.data[row].zmask[chunk] |= mask;
            } 
            // (x=1,z=1) -> (x=1,z=0) phase flip
            else if x_set && z_set{
                self.data[row].zmask[chunk] ^= mask;
                self.data[row].phase ^= true;
            }
        }
    }

    pub fn apply_cnot(&mut self, control: usize, target: usize) {
        let c_chunk = control / 64;
        let c_bit = control % 64;
        let c_mask = 1u64 << c_bit;

        let t_chunk = target / 64;
        let t_bit = target % 64;
        let t_mask = 1u64 << t_bit;

        for row in 0..(2*self.num_qubits){
            let c_x_set = (self.data[row].xmask[c_chunk] & c_mask) != 0;
            let c_z_set = (self.data[row].zmask[c_chunk] & c_mask) != 0;
            let t_x_set = (self.data[row].xmask[t_chunk] & t_mask) != 0;
            let t_z_set = (self.data[row].zmask[t_chunk] & t_mask) != 0;

            if c_x_set{
                //toggle target x
                self.data[row].xmask[t_chunk] ^= t_mask;
            }
            if c_z_set{
                //toggle target z
                self.data[row].zmask[t_chunk] ^= t_mask;
            }
            // Target → Control transformations  
            if t_z_set {
                self.data[row].zmask[c_chunk] ^= c_mask;
            }
            if t_x_set {
                self.data[row].xmask[c_chunk] ^= c_mask;
            }
        }
    }

    pub fn measure_z(&mut self, qubit: usize) -> bool {
        let chunk = qubit / 64;
        let bit = qubit % 64;
        let mask = 1u64 << bit;
        let mut rng = rand::thread_rng();
        
        // Look for anticommuting stabilizer
        for row in 0..(2 * self.num_qubits) {
            let x = (self.data[row].xmask[chunk] & mask) != 0;
            if x {
                // Found anticommuting stabilizer, return random outcome
                let outcome = rng.gen_bool(0.5);
                
                // Replace this row with Z_qubit stabilizer
                for i in 0..self.data[row].xmask.len() {
                    self.data[row].xmask[i] = 0;
                    self.data[row].zmask[i] = 0;
                }
                self.data[row].zmask[chunk] = mask;
                self.data[row].phase = outcome;
                
                // Eliminate X_qubit from other stabilizers using this row
                for other_row in 0..(2 * self.num_qubits) {
                    if other_row != row {
                        let other_x = (self.data[other_row].xmask[chunk] & mask) != 0;
                        if other_x {
                            // XOR this row into other_row to eliminate X_qubit
                            for i in 0..self.data[row].xmask.len() {
                                self.data[other_row].xmask[i] ^= self.data[row].xmask[i];
                                self.data[other_row].zmask[i] ^= self.data[row].zmask[i];
                            }
                            self.data[other_row].phase ^= self.data[row].phase;
                        }
                    }
                }
                
                return outcome;
            }
        }
        
        // No anticommuting stabilizer found - deterministic outcome
        // Check if any stabilizer has Z on this qubit with odd phase
        for row in 0..(2 * self.num_qubits) {
            let z = (self.data[row].zmask[chunk] & mask) != 0;
            if z && self.data[row].phase {
                return true;  // -Z stabilizer means measurement gives -1 (true)
            }
        }
        
        return false;  // Fixed: added return
    }

    pub fn dump(&self) {
        println!("Tableau: {:?}", self.data);
    }
}
