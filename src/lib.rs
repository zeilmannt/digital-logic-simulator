//! # Digital Logic Simulator
//!
//! This crate provides core data structures and logic for simulating digital logic circuits.
//! It supports basic logic gates (`AND`, `OR`, `XOR`, `NOT`), circuits composed of these gates,
//! and connections between gates.
//!
//! ## Modules
//! - `gate`: Defines logic gate types and gate behavior.
//! - `circuit`: Represents a circuit as a collection of gates and manages signal propagation.
//! - `connection`: Manages connections between gates in the circuit.
pub mod gate;
pub mod circuit;
pub mod connection;
pub mod ui;
