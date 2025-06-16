use strum_macros::EnumIter;

/// Represents the different types of logic gates supported by the simulator.
#[derive(EnumIter, PartialEq, Debug, Clone, Copy)]
pub enum GateType {
    And,
    Or,
    Not,
    Xor,
    Input,
}

/// A logic gate with a specific type, input signals, and an output signal.
///
/// The gate evaluates its output based on the type and the current inputs.
#[derive(Debug)]
pub struct Gate {
    pub gate_type: GateType,
    pub input_count: usize,
    pub output: bool,
}


impl Gate {
    pub fn new(gate_type: GateType, input_count: usize) -> Self {
        Self {
            gate_type,
            input_count,
            output: false,
        }
    }

    /// Evaluate gate output based on given inputs
    pub fn evaluate_with_inputs(&self, inputs: &[bool]) -> bool {
        match self.gate_type {
            GateType::Input => {
                // For inputs, output is externally set, so return stored output
                self.output
            }
            GateType::And => inputs.iter().all(|&x| x),
            GateType::Or => inputs.iter().any(|&x| x),
            GateType::Not => {
                if inputs.len() != 1 {
                    false
                } else {
                    !inputs[0]
                }
            }
            GateType::Xor => inputs.iter().filter(|&&x| x).count() % 2 == 1,
        }
    }
}
