use strum_macros::EnumIter;

/// Represents the different types of logic gates supported by the simulator.
#[derive(EnumIter, PartialEq, Debug, Clone, Copy)]
pub enum GateType {
    And,
    Or,
    Not,
    Xor,
}

/// A logic gate with a specific type, input signals, and an output signal.
///
/// The gate evaluates its output based on the type and the current inputs.
#[derive(Debug)]
pub struct Gate {
    pub gate_type: GateType,
    pub inputs: Vec<bool>,
    pub output: bool,
}

impl Gate {
    /// Creates a new `Gate` of the specified type with a given number of inputs.
    ///
    /// All inputs are initially set to `false`, and output is initialized to `false`.
    ///
    /// # Arguments
    ///
    /// * `gate_type` - The type of logic gate to create.
    /// * `input_count` - The number of inputs this gate will have.
    pub fn new(gate_type: GateType, input_count: usize) -> Self {
        Gate {
            gate_type,
            inputs: vec![false; input_count],
            output: false,
        }
    }

    /// Evaluates the gate’s output based on its inputs and gate type.
    ///
    /// The result is stored in the `output` field.
    ///
    /// # Behavior by GateType
    /// - `And`: outputs `true` if **all** inputs are `true`.
    /// - `Or`: outputs `true` if **any** input is `true`.
    /// - `Xor`: outputs `true` if an **odd number** of inputs are `true`.
    /// - `Not`: outputs the logical negation of the **first** input.
    ///
    /// # Panics
    ///
    /// Panics if the gate is a `Not` gate but has no inputs.
    pub fn evaluate(&mut self) {
        self.output = match self.gate_type {
            GateType::And => self.inputs.iter().all(|&b| b),
            GateType::Or => self.inputs.iter().any(|&b| b),
            GateType::Xor => self.inputs.iter().fold(false, |a, b| a ^ b),
            GateType::Not => !self.inputs[0],
        };
    }
}
