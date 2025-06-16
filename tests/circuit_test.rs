use digital_logic_simulator::gate::GateType;
use digital_logic_simulator::circuit::Circuit;

#[test]
fn test_add_gate_and_get_output() {
    let mut circuit = Circuit::new();
    let gate_id = circuit.add_gate(GateType::Input, 0);
    circuit.set_primary_input_value(gate_id, true);
    assert_eq!(circuit.get_output(gate_id), true);
}

#[test]
#[should_panic(expected = "Gate 0 is not an input gate")]
fn test_set_primary_input_on_non_input_gate_panics() {
    let mut circuit = Circuit::new();
    let gate_id = circuit.add_gate(GateType::And, 2);
    circuit.set_primary_input_value(gate_id, true); // Should panic
}

#[test]
fn test_and_gate_evaluation() {
    let mut circuit = Circuit::new();

    let input1 = circuit.add_gate(GateType::Input, 0);
    let input2 = circuit.add_gate(GateType::Input, 0);
    let and_gate = circuit.add_gate(GateType::And, 2);

    circuit.connect(input1, and_gate, 0);
    circuit.connect(input2, and_gate, 1);

    circuit.set_primary_input_value(input1, true);
    circuit.set_primary_input_value(input2, true);

    circuit.evaluate();
    assert_eq!(circuit.get_output(and_gate), true);

    circuit.set_primary_input_value(input2, false);
    circuit.evaluate();
    assert_eq!(circuit.get_output(and_gate), false);
}

#[test]
fn test_not_gate() {
    let mut circuit = Circuit::new();

    let input = circuit.add_gate(GateType::Input, 0);
    let not_gate = circuit.add_gate(GateType::Not, 1);

    circuit.connect(input, not_gate, 0);

    circuit.set_primary_input_value(input, false);
    circuit.evaluate();
    assert_eq!(circuit.get_output(not_gate), true);

    circuit.set_primary_input_value(input, true);
    circuit.evaluate();
    assert_eq!(circuit.get_output(not_gate), false);
}

#[test]
fn test_connection_tracking() {
    let mut circuit = Circuit::new();

    let a = circuit.add_gate(GateType::Input, 0);
    let b = circuit.add_gate(GateType::Input, 0);
    let and_gate = circuit.add_gate(GateType::And, 2);

    circuit.connect(a, and_gate, 0);
    circuit.connect(b, and_gate, 1);

    let connections = circuit.connections();
    assert_eq!(connections.len(), 2);
    assert!(connections.contains(&(a, and_gate, 0)));
    assert!(connections.contains(&(b, and_gate, 1)));
}

#[test]
fn test_circuit_with_nested_gates() {
    let mut circuit = Circuit::new();

    let a = circuit.add_gate(GateType::Input, 0);
    let b = circuit.add_gate(GateType::Input, 0);
    let and1 = circuit.add_gate(GateType::And, 2);
    let not1 = circuit.add_gate(GateType::Not, 1);

    circuit.connect(a, and1, 0);
    circuit.connect(b, and1, 1);
    circuit.connect(and1, not1, 0);

    circuit.set_primary_input_value(a, true);
    circuit.set_primary_input_value(b, true);
    circuit.evaluate();

    assert_eq!(circuit.get_output(and1), true);
    assert_eq!(circuit.get_output(not1), false);
}
