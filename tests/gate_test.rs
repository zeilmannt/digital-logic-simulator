use digital_logic_simulator::gate::{Gate, GateType};

#[test]
fn test_input_gate_returns_stored_output() {
    let gate = Gate {
        gate_type: GateType::Input,
        input_count: 0,
        output: true,
    };

    assert_eq!(gate.evaluate_with_inputs(&[]), true);

    let gate = Gate {
        gate_type: GateType::Input,
        input_count: 0,
        output: false,
    };

    assert_eq!(gate.evaluate_with_inputs(&[]), false);
}

#[test]
fn test_and_gate() {
    let gate = Gate::new(GateType::And, 2);
    assert_eq!(gate.evaluate_with_inputs(&[true, true]), true);
    assert_eq!(gate.evaluate_with_inputs(&[true, false]), false);
    assert_eq!(gate.evaluate_with_inputs(&[false, false]), false);
}

#[test]
fn test_or_gate() {
    let gate = Gate::new(GateType::Or, 2);
    assert_eq!(gate.evaluate_with_inputs(&[false, false]), false);
    assert_eq!(gate.evaluate_with_inputs(&[true, false]), true);
    assert_eq!(gate.evaluate_with_inputs(&[true, true]), true);
}

#[test]
fn test_not_gate() {
    let gate = Gate::new(GateType::Not, 1);
    assert_eq!(gate.evaluate_with_inputs(&[true]), false);
    assert_eq!(gate.evaluate_with_inputs(&[false]), true);
}

#[test]
fn test_not_gate_with_invalid_input_length() {
    let gate = Gate::new(GateType::Not, 1);
    assert_eq!(gate.evaluate_with_inputs(&[]), false);            // invalid
    assert_eq!(gate.evaluate_with_inputs(&[true, false]), false); // invalid
}

#[test]
fn test_xor_gate() {
    let gate = Gate::new(GateType::Xor, 2);
    assert_eq!(gate.evaluate_with_inputs(&[false, false]), false);
    assert_eq!(gate.evaluate_with_inputs(&[true, false]), true);
    assert_eq!(gate.evaluate_with_inputs(&[true, true]), false);
    assert_eq!(gate.evaluate_with_inputs(&[true, false, true]), false); // 2 true
    assert_eq!(gate.evaluate_with_inputs(&[true, true, true]), true);   // 3 true
}
