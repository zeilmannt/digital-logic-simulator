
use eframe::egui::{self, CentralPanel, SidePanel, Pos2, Rect, Sense, Vec2, Color32, Stroke};
use egui::vec2;
//use egui::{Pos2, vec2, Color32, Rect};
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

    pub fn draw(&mut self, ctx: &egui::Context) {
        // Sidebar with gate buttons
        SidePanel::left("gate_selection_panel").show(ctx, |ui| {
            ui.heading("Select Gate Type");

            for &gate_type in &[GateType::And, GateType::Or, GateType::Not, GateType::Xor] {
                if ui.button(format!("{:?}", gate_type)).clicked() {
                    self.selected_gate = Some(gate_type);
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
            let painter = ui.painter();

            // Draw all gates
            for gate in &self.gate_widgets {
                let rect = Rect::from_min_size(gate.position, vec2(80.0, 50.0));
                let color = Color32::from_rgb(200, 200, 250);
                painter.rect_filled(rect, 5.0, color);
                painter.text(
                    gate.position + vec2(10.0, 10.0),
                    egui::Align2::LEFT_TOP,
                    format!("{:?}", gate.gate_type),
                    egui::TextStyle::Body.resolve(ui.style()),
                    Color32::BLACK,
                );
            }

            // If canvas clicked, add selected gate
            if ui.ctx().input(|i| i.pointer.primary_clicked()) {
                if let (Some(pos), Some(gate_type)) = (
                    ui.ctx().input(|i| i.pointer.interact_pos()),
                    self.selected_gate,
                ) {
                    self.add_gate(gate_type, pos);
                }
            }
        });
    }

}
