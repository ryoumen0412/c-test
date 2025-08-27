use eframe::egui;
use crate::ui::app::AppState;
use crate::ui::theme::AppleMusicStyle;

pub struct Sidebar {
    // Estado del sidebar si es necesario
}

impl Sidebar {
    pub fn new() -> Self {
        Self {}
    }

    pub fn show(&mut self, ui: &mut egui::Ui, current_state: &AppState) -> Option<AppState> {
        let mut new_state = None;
        
        // Aplicar frame de sidebar estilo Apple Music
        AppleMusicStyle::sidebar_frame().show(ui, |ui| {
            ui.add_space(20.0);
            
            // Header del sidebar con estilo Apple Music
            ui.vertical_centered(|ui| {
                ui.add(egui::Label::new(AppleMusicStyle::header_text("Menu")));
            });
            
            ui.add_space(24.0);
            ui.separator();
            ui.add_space(24.0);

            // Botones de navegación con estilo Apple Music
            ui.vertical(|ui| {
                ui.spacing_mut().item_spacing.y = 8.0;

                let dashboard_button = AppleMusicStyle::nav_button("Dashboard", *current_state == AppState::Dashboard);
                if ui.add(dashboard_button).clicked() {
                    new_state = Some(AppState::Dashboard);
                }

                let queries_button = AppleMusicStyle::nav_button("Consultas", *current_state == AppState::Queries);
                if ui.add(queries_button).clicked() {
                    new_state = Some(AppState::Queries);
                }

                let insertions_button = AppleMusicStyle::nav_button("Inserciones", *current_state == AppState::Insertions);
                if ui.add(insertions_button).clicked() {
                    new_state = Some(AppState::Insertions);
                }

                let about_button = AppleMusicStyle::nav_button("About", *current_state == AppState::About);
                if ui.add(about_button).clicked() {
                    new_state = Some(AppState::About);
                }
            });

            // Separador y estado de conexión en la parte inferior
            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                ui.add_space(20.0);
                
                // Botón de desconexión con estilo
                let disconnect_button = egui::Button::new(
                    egui::RichText::new("Desconectar").color(AppleMusicStyle::TEXT_SECONDARY)
                )
                .fill(egui::Color32::TRANSPARENT)
                .rounding(egui::Rounding::same(8.0))
                .stroke(egui::Stroke::new(1.0, AppleMusicStyle::TEXT_SECONDARY))
                .min_size(egui::vec2(160.0, 36.0));
                
                if ui.add(disconnect_button).clicked() {
                    new_state = Some(AppState::Login);
                }
                
                ui.add_space(16.0);
                
                // Estado de conexión
                ui.horizontal(|ui| {
                    ui.add(egui::widgets::Spinner::new().size(12.0).color(AppleMusicStyle::PRIMARY_BLUE));
                    ui.add_space(8.0);
                    ui.add(egui::Label::new(AppleMusicStyle::secondary_text("Conectado")));
                });
                
                ui.add_space(20.0);
                ui.separator();
                ui.add_space(10.0);
            });
        });
        
        new_state
    }

    #[allow(dead_code)]
    fn nav_button(&self, ui: &mut egui::Ui, text: &str, is_selected: bool, size: egui::Vec2) -> bool {
        let button = egui::Button::new(text)
            .min_size(size)
            .fill(if is_selected { 
                egui::Color32::from_rgb(70, 130, 180) 
            } else { 
                egui::Color32::TRANSPARENT 
            });

        ui.add(button).clicked()
    }
}
