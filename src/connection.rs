/// Alias for identifying a gate within a circuit by its index.
///
/// Using `GateId` improves code readability by clarifying that the `usize`
/// represents a unique gate identifier.
pub type GateId = usize;

/// Represents a directed connection from the output of one gate to the input of another.
///
/// Each connection specifies:
/// - `from`: The source gate ID whose output is sent.
/// - `to`: The destination gate ID receiving the signal as input.
/// - `input_index`: The input slot index on the destination gate that this connection drives.
///
/// This struct is used to model wiring between gates inside a circuit.
#[derive(Debug)]
pub struct Connection {
    pub from: GateId,
    pub to: GateId,
    pub input_index: usize,
}
