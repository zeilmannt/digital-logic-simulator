use eframe::egui::{self, CentralPanel, SidePanel, Pos2, Rect, Sense, Color32, Stroke};
use egui::vec2;
use strum::IntoEnumIterator;
use crate::circuit::Circuit;
use crate::gate::GateType;

pub type GateId = usize;

#[derive(Debug)]
pub struct PlacedGate {
    pub id: GateId,
    pub gate_type: GateType,
    pub position: Pos2,
}

#[derive(Debug)]
pub struct GateWidget {
    pub id: usize,
    pub gate_type: GateType,
    pub position: Pos2,        // Top-left corner of gate rectangle
    pub input_state: Option<bool>,
}

pub struct CircuitEditor {
    pub circuit: Circuit,
    pub gate_widgets: Vec<GateWidget>,
    pub selected_gate: Option<GateType>,
    pub connect_from: Option<GateId>,
}

impl CircuitEditor {
    pub fn new() -> Self {
        Self {
            circuit: Circuit::new(),
            gate_widgets: vec![],
            selected_gate: None,
            connect_from: None,
        }
    }

    pub fn add_gate(&mut self, gate_type: GateType, position: Pos2) {
        let id = self.circuit.add_gate(
            gate_type,
            match gate_type {
                GateType::Not => 1,
                GateType::Input => 0,
                _ => 2,
            },
        );

        let input_state = if gate_type == GateType::Input { Some(false) } else { None };

        self.gate_widgets.push(GateWidget {
            id,
            gate_type,
            position,
            input_state,
        });
    }

    fn is_position_free(&self, pos: Pos2) -> bool {
        let new_rect = Rect::from_min_size(pos, vec2(80.0, 50.0));
        !self.gate_widgets.iter().any(|gate| {
            let gate_rect = Rect::from_min_size(gate.position, vec2(80.0, 50.0));
            gate_rect.intersects(new_rect)
        })
    }

    pub fn draw(&mut self, ctx: &egui::Context) {
        // Sidebar for gate selection
        SidePanel::left("gate_selection_panel").show(ctx, |ui| {
            ui.heading("Select Gate Type");

            for gate_type in GateType::iter() {
                let selected = self.selected_gate == Some(gate_type);
                let button = egui::Button::new(format!("{:?}", gate_type))
                    .fill(if selected { Color32::DARK_GREEN } else { Color32::DARK_GRAY })
                    .stroke(if selected {
                        Stroke::new(2.0, Color32::YELLOW)
                    } else {
                        Stroke::NONE
                    });

                if ui.add(button).clicked() {
                    self.selected_gate = if selected { None } else { Some(gate_type) };
                }
            }

            if let Some(gate) = self.selected_gate {
                ui.label(format!("Selected: {:?}", gate));
            } else {
                ui.label("No gate selected");
            }
        });

        // Main canvas panel
        CentralPanel::default().show(ctx, |ui| {
            let canvas_size = ui.available_size();
            let response = ui.allocate_rect(
                Rect::from_min_size(ui.min_rect().min, canvas_size),
                Sense::click(),
            );

            // Allocate toggle buttons for Input gates, store their responses and rects
            let mut toggle_responses = Vec::new();
            for gate in &mut self.gate_widgets {
                if gate.gate_type == GateType::Input {
                    let toggle_rect = Rect::from_min_size(gate.position + vec2(10.0, 30.0), vec2(60.0, 15.0));
                    let toggle_response = ui.allocate_rect(toggle_rect, Sense::click());
                    toggle_responses.push((gate.id, toggle_response, toggle_rect));
                }
            }

            let painter = ui.painter();

            // Constants for drawing pins
            let gate_size = vec2(80.0, 50.0);
            let pin_radius = 6.0;

            // Draw all gates
            for gate in &self.gate_widgets {
                let rect = Rect::from_min_size(gate.position, gate_size);
                let is_input = gate.gate_type == GateType::Input;

                // Filled rectangle for the gate
                painter.rect_filled(
                    rect,
                    5.0,
                    if is_input { Color32::LIGHT_GREEN } else { Color32::LIGHT_BLUE },
                );

                // Outline for debugging
                painter.rect_stroke(rect, 5.0, Stroke::new(1.0, Color32::BLACK));

                // Draw gate type text
                painter.text(
                    gate.position + vec2(10.0, 10.0),
                    egui::Align2::LEFT_TOP,
                    format!("{:?}", gate.gate_type),
                    egui::TextStyle::Body.resolve(ui.style()),
                    Color32::BLACK,
                );

                // Draw input pins on left side
                let input_count = match gate.gate_type {
                    GateType::Not => 1,
                    GateType::Input => 0,
                    _ => 2,
                };
                let input_spacing = gate_size.y / (input_count as f32 + 1.0);

                for i in 0..input_count {
                    let y = gate.position.y + input_spacing * (i as f32 + 1.0);
                    let input_pos = Pos2::new(gate.position.x, y);
                    // Determine input signal from circuit connections
                    let input_signal = {
                        // Find the connection feeding this input
                        let connections = self.circuit.connections(); // Bind to a variable to keep it alive
                        let conn = connections.iter().find(|&&(_, to_id, input_idx)| {
                            to_id == gate.id && input_idx == i
                        });
                        if let Some((from_id, _, _)) = conn {
                            self.circuit.get_output(*from_id)
                        } else {
                            false
                        }

                    };
                    let color = if input_signal { Color32::GREEN } else { Color32::RED };
                    painter.circle_filled(input_pos, pin_radius, color);
                    // Label inputs
                    painter.text(
                        input_pos - vec2(10.0, 0.0),
                        egui::Align2::RIGHT_CENTER,
                        format!("In{}", i),
                        egui::TextStyle::Small.resolve(ui.style()),
                        Color32::BLACK,
                    );
                }

                // Draw output pin on right side (center vertically)
                let output_pos = Pos2::new(gate.position.x + gate_size.x, gate.position.y + gate_size.y / 2.0);
                let output_signal = self.circuit.get_output(gate.id);
                let output_color = if output_signal { Color32::GREEN } else { Color32::RED };
                painter.circle_filled(output_pos, pin_radius, output_color);
                painter.text(
                    output_pos + vec2(10.0, 0.0),
                    egui::Align2::LEFT_CENTER,
                    "Out",
                    egui::TextStyle::Small.resolve(ui.style()),
                    Color32::BLACK,
                );
            }

            // Handle toggle button clicks and draw toggles
            for (gate_id, toggle_response, toggle_rect) in toggle_responses {
                if toggle_response.clicked() {
                    let current = self.circuit.get_output(gate_id);
                    self.circuit.set_primary_input_value(gate_id, !current);
                    self.circuit.evaluate();

                    if let Some(gate_widget) = self.gate_widgets.iter_mut().find(|g| g.id == gate_id) {
                        gate_widget.input_state = Some(!current);
                    }
                }

                painter.rect_filled(toggle_rect, 2.0, Color32::DARK_GRAY);
                painter.text(
                    toggle_rect.center(),
                    egui::Align2::CENTER_CENTER,
                    if self.circuit.get_output(gate_id) { "TRUE" } else { "FALSE" },
                    egui::TextStyle::Body.resolve(ui.style()),
                    Color32::WHITE,
                );
            }

            // Draw connection lines
            for (from_id, to_id, input_index) in self.circuit.connections() {
                let from_gate = self.gate_widgets.iter().find(|g| g.id == from_id);
                let to_gate = self.gate_widgets.iter().find(|g| g.id == to_id);

                if let (Some(from), Some(to)) = (from_gate, to_gate) {
                    let from_pos = from.position + vec2(gate_size.x, gate_size.y / 2.0); // Right edge middle
                    // Inputs spaced vertically by input_spacing to align with pins
                    let input_count = match to.gate_type {
                        GateType::Not => 1,
                        GateType::Input => 0,
                        _ => 2,
                    };
                    let input_spacing = gate_size.y / (input_count as f32 + 1.0);
                    let to_pos = to.position + vec2(0.0, input_spacing * (input_index as f32 + 1.0));

                    let output_value = self.circuit.get_output(from_id);
                    let color = if output_value { Color32::GREEN } else { Color32::RED };

                    painter.line_segment([from_pos, to_pos], Stroke::new(2.0, color));
                }
            }

            // Toggle input gates with click on gate rectangle area as fallback
            let pointer_pos = ui.ctx().input(|i| i.pointer.interact_pos());

            if let Some(pos) = pointer_pos {
                if ui.input(|i| i.pointer.any_click()) {
                    for gate in &mut self.gate_widgets {
                        let rect = Rect::from_min_size(gate.position, gate_size);
                        if rect.contains(pos) && gate.gate_type == GateType::Input {
                            gate.input_state = Some(!gate.input_state.unwrap_or(false));
                            self.circuit.set_primary_input_value(gate.id, gate.input_state.unwrap());
                            self.circuit.evaluate();
                        }
                    }
                }
            }

            // Handle gate placement or connection
            if response.clicked() {
                if let Some(click_pos) = response.interact_pointer_pos() {
                    // Adjust position so that gate is centered at click position
                    let adjusted_pos = click_pos - vec2(gate_size.x / 2.0, gate_size.y / 2.0);

                    let clicked_gate = self.gate_widgets.iter().find(|g| {
                        let rect = Rect::from_min_size(g.position, gate_size);
                        rect.contains(click_pos)
                    });

                    if let Some(gate) = clicked_gate {
                        if let Some(from_id) = self.connect_from {
                            self.circuit.connect(from_id, gate.id, 0);
                            self.connect_from = None;
                            self.circuit.evaluate();
                        } else {
                            self.connect_from = Some(gate.id);
                        }
                    } else if let Some(gate_type) = self.selected_gate {
                        if self.is_position_free(adjusted_pos) {
                            self.add_gate(gate_type, adjusted_pos);
                            self.circuit.evaluate();
                        }
                    }
                }
            }
        });
    }
}
