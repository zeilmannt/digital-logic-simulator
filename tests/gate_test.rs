use digital_logic_simulator::gate::{Gate, GateType};

#[test]
fn test_and_gate() {
    let mut gate = Gate::new(GateType::And, 2);

    gate.inputs = vec![true, true];
    gate.evaluate();
    assert_eq!(gate.output, true);

    gate.inputs = vec![true, false];
    gate.evaluate();
    assert_eq!(gate.output, false);

    gate.inputs = vec![false, false];
    gate.evaluate();
    assert_eq!(gate.output, false);
}

#[test]
fn test_or_gate() {
    let mut gate = Gate::new(GateType::Or, 2);

    gate.inputs = vec![false, false];
    gate.evaluate();
    assert_eq!(gate.output, false);

    gate.inputs = vec![true, false];
    gate.evaluate();
    assert_eq!(gate.output, true);

    gate.inputs = vec![true, true];
    gate.evaluate();
    assert_eq!(gate.output, true);
}

#[test]
fn test_xor_gate() {
    let mut gate = Gate::new(GateType::Xor, 2);

    gate.inputs = vec![false, false];
    gate.evaluate();
    assert_eq!(gate.output, false);

    gate.inputs = vec![true, false];
    gate.evaluate();
    assert_eq!(gate.output, true);

    gate.inputs = vec![true, true];
    gate.evaluate();
    assert_eq!(gate.output, false);
}

#[test]
fn test_not_gate() {
    let mut gate = Gate::new(GateType::Not, 1);

    gate.inputs = vec![false];
    gate.evaluate();
    assert_eq!(gate.output, true);

    gate.inputs = vec![true];
    gate.evaluate();
    assert_eq!(gate.output, false);
}