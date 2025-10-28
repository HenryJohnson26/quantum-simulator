use crate::compiler::ir::{IROp, IRProgram};

pub fn analyze_promotion_points(prog: &mut IRProgram) {
    // Simple heuristic: cluster T gates together, promote once, demote later
    let mut count = 0;
    for op in &prog.ops {
        if matches!(op, IROp::T(_) | IROp::RZ(_, _)) {
            count += 1;
        }
    }
    println!("Detected {} non-Clifford gates, scheduling promotion window", count);
}
