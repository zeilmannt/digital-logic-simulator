use crate::connection::{Connection, GateId};
use crate::gate::{Gate, GateType};
use std::collections::HashMap;

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

    /// Recursively evaluates the output of a gate
    pub fn evaluate_gate(&self, gate_id: GateId, cache: &mut HashMap<GateId, bool>) -> bool {
        if let Some(&cached_output) = cache.get(&gate_id) {
            return cached_output;
        }

        let gate = &self.gates[gate_id];

        // Input gates output is stored directly
        if gate.gate_type == GateType::Input {
            cache.insert(gate_id, gate.output);
            return gate.output;
        }

        // Gather inputs by following connections
        let mut inputs = vec![false; gate.input_count];
        for conn in self.connections.iter().filter(|c| c.to == gate_id) {
            inputs[conn.input_index] = self.evaluate_gate(conn.from, cache);
        }

        // Evaluate this gate with its input values
        let output = gate.evaluate_with_inputs(&inputs);

        cache.insert(gate_id, output);
        output
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

    /// Evaluate the entire circuit by evaluating all gates in order
    pub fn evaluate(&mut self) {
        let mut cache = HashMap::new();

        for gate_id in 0..self.gates.len() {
            let output = self.evaluate_gate(gate_id, &mut cache);
            self.gates[gate_id].output = output;
        }
    }

    /// Set the output value of an input gate
    pub fn set_primary_input_value(&mut self, gate_id: GateId, value: bool) {
        if self.gates[gate_id].gate_type == GateType::Input {
            self.gates[gate_id].output = value;
        } else {
            panic!("Gate {} is not an input gate", gate_id);
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

    pub fn connect(&mut self, from: GateId, to: GateId, input_index: usize) {
        self.connections.push(Connection {
            from,
            to,
            input_index,
        });
    }

    pub fn connections(&self) -> Vec<(GateId, GateId, usize)> {
        self.connections.iter().map(|c| (c.from, c.to, c.input_index)).collect()
    }

}
