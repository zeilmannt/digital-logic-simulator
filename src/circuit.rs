use crate::connection::{Connection, GateId};
use crate::gate::{Gate, GateType};

/// Represents a digital logic circuit composed of gates, connections, and primary inputs.
///
/// The circuit manages the evaluation of gate outputs based on connections and external inputs.
///
/// # Fields
/// - `gates`: All logic gates contained in the circuit.
/// - `connections`: Links representing connections between gate outputs and inputs.
/// - `primary_inputs`: External inputs to the circuit (e.g., switches).
pub struct Circuit {
    gates: Vec<Gate>,
    connections: Vec<Connection>,
    primary_inputs: Vec<bool>, // inputs like switches
}

impl Circuit {
    /// Creates a new empty circuit with no gates, connections, or primary inputs.
    pub fn new() -> Self {
        Self {
            gates: vec![],
            connections: vec![],
            primary_inputs: vec![],
        }
    }

    /// Adds a new gate of the specified type and input count to the circuit.
    ///
    /// Returns the `GateId` (index) of the newly added gate.
    ///
    /// # Arguments
    ///
    /// * `gate_type` - The type of logic gate to add.
    /// * `input_count` - Number of inputs this gate accepts.
    pub fn add_gate(&mut self, gate_type: GateType, input_count: usize) -> GateId {
        let gate = Gate::new(gate_type, input_count);
        self.gates.push(gate);
        self.gates.len() - 1
    }

    /// Adds a connection from one gate's output to another gate's input.
    ///
    /// # Arguments
    ///
    /// * `from` - The source gate ID (output).
    /// * `to` - The target gate ID (input).
    /// * `input_index` - The index of the input on the target gate.
    pub fn add_connection(&mut self, from: GateId, to: GateId, input_index: usize) {
        self.connections.push(Connection { from, to, input_index });
    }

    /// Sets the external primary inputs of the circuit.
    ///
    /// These inputs typically represent switches or sensors feeding signals into the circuit.
    ///
    /// # Arguments
    ///
    /// * `inputs` - Vector of boolean input values.
    pub fn set_primary_inputs(&mut self, inputs: Vec<bool>) {
        self.primary_inputs = inputs;
    }

    /// Evaluates the entire circuit.
    ///
    /// This method:
    /// 1. Applies primary inputs to the first gates’ inputs.
    /// 2. Evaluates all gates.
    /// 3. Propagates outputs through connections.
    /// 4. Evaluates gates again to update outputs based on propagated signals.
    ///
    /// This simulates signal flow through the circuit until steady state.
    ///
    /// # Panics
    ///
    /// Panics if gate IDs in connections are invalid or input indices are out of range.
    pub fn evaluate(&mut self) {
        // Step 1: Apply primary inputs to the first few gates
        for (i, &input) in self.primary_inputs.iter().enumerate() {
            if i < self.gates[0].inputs.len() {
                self.gates[0].inputs[i] = input;
            }
        }

        // Step 2: Evaluate gates once
        for gate in self.gates.iter_mut() {
            gate.evaluate();
        }

        // Step 3: Propagate output to inputs of next gates
        for conn in &self.connections {
            let from_output = self.gates[conn.from].output;
            self.gates[conn.to].inputs[conn.input_index] = from_output;
        }

        // Step 4: Evaluate gates again to update outputs with new inputs
        for gate in self.gates.iter_mut() {
            gate.evaluate();
        }
    }

    /// Returns the output value of the gate with the given `GateId`.
    ///
    /// # Arguments
    ///
    /// * `gate_id` - The index of the gate whose output is requested.
    ///
    /// # Returns
    ///
    /// Boolean value representing the gate's output.
    ///
    /// # Panics
    ///
    /// Panics if `gate_id` is out of bounds.
    pub fn get_output(&self, gate_id: GateId) -> bool {
        self.gates[gate_id].output
    }
}
