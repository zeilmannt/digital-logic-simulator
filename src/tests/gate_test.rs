use digital_logic_simulator::gate::{Gate, GateType};

#[test]
fn test_and_gate() {
    let mut gate = Gate::new(GateType::And, 2);
    gate.inputs = vec![true, true];
    assert_eq!(gate.evaluate(), true);

    gate.inputs = vec![true, false];
    assert_eq!(gate.evaluate(), false);

    gate.inputs = vec![false, false];
    assert_eq!(gate.evaluate(), false);
}

#[test]
fn test_or_gate() {
    let mut gate = Gate::new(GateType::Or, 2);
    gate.inputs = vec![false, false];
    assert_eq!(gate.evaluate(), false);

    gate.inputs = vec![true, false];
    assert_eq!(gate.evaluate(), true);

    gate.inputs = vec![true, true];
    assert_eq!(gate.evaluate(), true);
}

#[test]
fn test_xor_gate() {
    let mut gate = Gate::new(GateType::Xor, 2);
    gate.inputs = vec![false, false];
    assert_eq!(gate.evaluate(), false);

    gate.inputs = vec![true, false];
    assert_eq!(gate.evaluate(), true);

    gate.inputs = vec![true, true];
    assert_eq!(gate.evaluate(), false);
}

#[test]
fn test_not_gate() {
    let mut gate = Gate::new(GateType::Not, 1);
    gate.inputs = vec![false];
    assert_eq!(gate.evaluate(), true);

    gate.inputs = vec![true];
    assert_eq!(gate.evaluate(), false);
}
