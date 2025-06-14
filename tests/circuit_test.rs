use digital_logic_simulator::circuit::Circuit;
use digital_logic_simulator::gate::{GateType};

#[test]
fn test_circuit_with_primary_inputs() {
    let mut circuit = Circuit::new();
    let and_gate = circuit.add_gate(GateType::And, 2);
    
    circuit.set_primary_inputs(vec![true, true]);
    circuit.evaluate();
    
    assert_eq!(circuit.get_output(and_gate), true);
}

#[test]
fn test_circuit_with_connections() {
    let mut circuit = Circuit::new();
    
    let gate0 = circuit.add_gate(GateType::And, 2);
    let gate1 = circuit.add_gate(GateType::Not, 1);
    
    circuit.add_connection(gate0, gate1, 0);
    circuit.set_primary_inputs(vec![true, false]);
    circuit.evaluate();
    
    assert_eq!(circuit.get_output(gate0), false);
    assert_eq!(circuit.get_output(gate1), true);
}

#[test]
fn test_multiple_evaluations() {
    let mut circuit = Circuit::new();
    let and_gate = circuit.add_gate(GateType::And, 2);
    
    circuit.set_primary_inputs(vec![false, false]);
    circuit.evaluate();
    
    assert_eq!(circuit.get_output(and_gate), false);
    
    circuit.set_primary_inputs(vec![true, true]);
    circuit.evaluate();

    assert_eq!(circuit.get_output(and_gate), true);
}
