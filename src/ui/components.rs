// Componentes UI reutilizables
use eframe::egui;

#[allow(dead_code)]
pub struct LoadingSpinner {
    size: f32,
}

#[allow(dead_code)]
impl LoadingSpinner {
    #[allow(dead_code)]
    pub fn new(size: f32) -> Self {
        Self { size }
    }

    #[allow(dead_code)]
    pub fn show(&self, ui: &mut egui::Ui) {
        ui.add(egui::widgets::Spinner::new().size(self.size));
    }
}

#[allow(dead_code)]
pub struct InfoCard {
    title: String,
    content: String,
    icon: String,
    color: egui::Color32,
}

#[allow(dead_code)]
impl InfoCard {
    #[allow(dead_code)]
    pub fn new(title: &str, content: &str, icon: &str, color: egui::Color32) -> Self {
        Self {
            title: title.to_string(),
            content: content.to_string(),
            icon: icon.to_string(),
            color,
        }
    }

    #[allow(dead_code)]
    pub fn show(&self, ui: &mut egui::Ui, min_size: egui::Vec2) {
        egui::Frame::none()
            .fill(self.color.linear_multiply(0.1))
            .rounding(egui::Rounding::same(8.0))
            .inner_margin(egui::Margin::same(15.0))
            .show(ui, |ui| {
                ui.set_min_size(min_size);
                ui.vertical_centered(|ui| {
                    ui.label(egui::RichText::new(&self.icon).size(24.0));
                    ui.add_space(5.0);
                    ui.label(egui::RichText::new(&self.content).size(28.0).color(self.color));
                    ui.label(egui::RichText::new(&self.title).size(12.0).color(egui::Color32::GRAY));
                });
            });
    }
}

#[allow(dead_code)]
pub fn show_error_dialog(ui: &mut egui::Ui, message: &str) {
    egui::Frame::popup(ui.style())
        .fill(egui::Color32::from_rgb(60, 20, 20))
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label("❌");
                ui.label(message);
            });
        });
}

pub fn show_success_dialog(ui: &mut egui::Ui, message: &str) {
    egui::Frame::popup(ui.style())
        .fill(egui::Color32::from_rgb(20, 60, 20))
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label("✅");
                ui.label(message);
            });
        });
}
