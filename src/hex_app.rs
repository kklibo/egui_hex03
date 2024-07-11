use egui::{Painter, Rect, Vec2};

pub struct HexApp {}

impl HexApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {}
    }
}

impl eframe::App for HexApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.heading("hex diff test (egui UI)");
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            let painter = Painter::new(
                ui.ctx().clone(),
                ui.layer_id(),
                ui.available_rect_before_wrap(),
            );
            painter.rect_filled(painter.clip_rect(), 10.0, egui::Color32::GRAY);

            let center = painter.clip_rect().center();

            let rect = Rect::from_two_pos(center, center + Vec2::new(-300.0, -100.0));
            painter.rect_filled(rect, 0.0, egui::Color32::GOLD);

            let rect = Rect::from_two_pos(center, center + Vec2::new(300.0, 100.0));
            painter.rect_filled(rect, 0.0, egui::Color32::DARK_RED);

            ui.expand_to_include_rect(painter.clip_rect());
        });
    }
}
