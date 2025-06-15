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
    pub position: Pos2,
}

pub struct CircuitEditor {
    pub circuit: Circuit,
    pub gate_widgets: Vec<GateWidget>,
    pub selected_gate: Option<GateType>,
}

impl CircuitEditor {
    pub fn new() -> Self {
        Self {
            circuit: Circuit::new(),
            gate_widgets: vec![],
            selected_gate: None,
        }
    }

    pub fn add_gate(&mut self, gate_type: GateType, position: Pos2) {
        let id = self.circuit.add_gate(gate_type, match gate_type {
            GateType::Not => 1,
            _ => 2,
        });

        self.gate_widgets.push(GateWidget {
            id,
            gate_type,
            position,
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
        // Sidebar with gate buttons
        SidePanel::left("gate_selection_panel").show(ctx, |ui| {
            ui.heading("Select Gate Type");

            for gate_type in GateType::iter() {
                let selected = self.selected_gate == Some(gate_type);

                let button = egui::Button::new(format!("{:?}", gate_type));
                let response = ui.add(
                    button
                        .stroke(if selected {
                            Stroke::new(3.0, Color32::YELLOW)
                        } else {
                            Stroke::default()
                        })
                        .fill(if selected {
                            Color32::from_rgb(60, 60, 60)
                        } else {
                            Color32::from_rgb(40, 40, 40)
                        }),
                );

                if response.clicked() {
                    if selected {
                        self.selected_gate = None; // toggle off
                    } else {
                        self.selected_gate = Some(gate_type);
                    }
                }
            }

            if let Some(gate) = self.selected_gate {
                ui.label(format!("Selected: {:?}", gate));
            } else {
                ui.label("No gate selected");
            }
        });

        // Main panel to place gates
        CentralPanel::default().show(ctx, |ui| {
            let canvas_size = ui.available_size();
            let response = ui.allocate_rect(
                Rect::from_min_size(ui.min_rect().min, canvas_size),
                Sense::click(),
            );
            let canvas_rect = response.rect;

            let painter = ui.painter();

            // Draw all gates
            for gate in &self.gate_widgets {
                let rect = Rect::from_min_size(gate.position, vec2(80.0, 50.0));
                painter.rect_filled(rect, 5.0, Color32::from_rgb(200, 200, 250));
                painter.text(
                    gate.position + vec2(10.0, 10.0),
                    egui::Align2::LEFT_TOP,
                    format!("{:?}", gate.gate_type),
                    egui::TextStyle::Body.resolve(ui.style()),
                    Color32::BLACK,
                );
            }

            // Add gate if canvas clicked, click inside canvas, and no overlap
            if ui.input(|i| i.pointer.any_click()) && ui.rect_contains_pointer(canvas_rect) {
                if let (Some(pos), Some(gate_type)) = (
                    ui.ctx().input(|i| i.pointer.interact_pos()),
                    self.selected_gate,
                ) {
                    if self.is_position_free(pos) {
                        self.add_gate(gate_type, pos);
                    }
                }
            }
        });
    }
}
