use digital_logic_simulator::{circuit::Circuit, gate::GateType};

fn main() {
    let mut circuit = Circuit::new();
    
    circuit.set_primary_inputs(vec![true, false]);
    
    // Gate 0: AND gate with 2 inputs
    let and_gate = circuit.add_gate(GateType::And, 2);
    
    // Gate 1: NOT gate with 1 input
    let not_gate = circuit.add_gate(GateType::Not, 1);
    
    circuit.add_connection(and_gate, not_gate, 0);
    
    circuit.evaluate();
    
    println!("AND gate output: {}", circuit.get_output(and_gate));
    println!("NOT gate output: {}", circuit.get_output(not_gate));
    
}

