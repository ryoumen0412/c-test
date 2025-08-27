use eframe::egui;
use std::sync::Arc;
use tokio::sync::{Mutex, mpsc};
use crate::database::DatabaseManager;
use crate::models::DashboardStats;
use crate::ui::theme::AppleMusicStyle;

pub struct DashboardView {
    db_manager: Arc<Mutex<DatabaseManager>>,
    stats: Option<DashboardStats>,
    loading: bool,
    last_refresh: std::time::Instant,
    stats_receiver: Option<mpsc::UnboundedReceiver<Result<DashboardStats, String>>>,
}

impl DashboardView {
    pub fn new(db_manager: Arc<Mutex<DatabaseManager>>) -> Self {
        let mut dashboard = Self {
            db_manager,
            stats: None,
            loading: false,
            last_refresh: std::time::Instant::now(),
            stats_receiver: None,
        };
        dashboard.refresh_stats();
        dashboard
    }

    pub fn check_stats_result(&mut self) -> bool {
        if let Some(receiver) = &mut self.stats_receiver {
            if let Ok(result) = receiver.try_recv() {
                self.loading = false;
                match result {
                    Ok(stats) => {
                        self.stats = Some(stats);
                        self.stats_receiver = None;
                        return true;
                    }
                    Err(_error_msg) => {
                        // En caso de error, mostrar datos vacíos
                        self.stats = Some(DashboardStats::default());
                        self.stats_receiver = None;
                        return true;
                    }
                }
            }
        }
        false
    }

    pub fn show(&mut self, ui: &mut egui::Ui) {
        // Check for async stats results
        self.check_stats_result();

        // Header con estilo Apple Music
        AppleMusicStyle::card_frame().show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.add(egui::Label::new(AppleMusicStyle::header_text("Dashboard")));
                
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    // Botón de actualizar con estilo Apple Music
                    let button_text = if self.loading { "Actualizando..." } else { "Actualizar" };
                    let button = egui::Button::new(button_text)
                        .fill(AppleMusicStyle::CARD_BG)
                        .rounding(egui::Rounding::same(8.0))
                        .stroke(egui::Stroke::new(1.0, AppleMusicStyle::SECONDARY_BLUE))
                        .min_size(egui::vec2(100.0, 32.0));
                    
                    if ui.add_enabled(!self.loading, button).clicked() {
                        self.refresh_stats();
                    }
                    
                    ui.add_space(16.0);
                    
                    // Tiempo desde última actualización
                    let elapsed = self.last_refresh.elapsed().as_secs();
                    let minutes = elapsed / 60;
                    let remaining_seconds = elapsed % 60;
                    
                    let time_text = if minutes > 0 {
                        format!("Actualizado hace {}m {}s", minutes, remaining_seconds)
                    } else {
                        format!("Actualizado hace {}s", remaining_seconds)
                    };
                    
                    ui.add(egui::Label::new(AppleMusicStyle::secondary_text(&time_text)));
                });
            });
        });

        ui.add_space(20.0);

        if self.loading {
            ui.with_layout(egui::Layout::centered_and_justified(egui::Direction::TopDown), |ui| {
                ui.add(egui::widgets::Spinner::new().size(32.0));
                ui.label("Cargando estadísticas...");
            });
            return;
        }

        if let Some(ref stats) = self.stats {
            self.show_stats_cards(ui, stats);
            ui.add_space(20.0);
            self.show_charts(ui, stats);
        } else {
            self.show_placeholder(ui);
        }
    }

    fn show_stats_cards(&self, ui: &mut egui::Ui, stats: &DashboardStats) {
        ui.label("Resumen General");
        ui.add_space(10.0);

        // Grid de tarjetas de estadísticas
        egui::Grid::new("stats_grid")
            .num_columns(4)
            .spacing([15.0, 15.0])
            .show(ui, |ui| {
                // Tarjeta de Personas
                self.stat_card(ui, "👥", "Personas Mayores", stats.total_personas.to_string(), egui::Color32::LIGHT_BLUE);
                
                // Tarjeta de Organizaciones
                self.stat_card(ui, "🏢", "Organizaciones", stats.total_organizaciones.to_string(), egui::Color32::LIGHT_GREEN);
                
                // Tarjeta de Actividades
                self.stat_card(ui, "🎯", "Actividades", stats.total_actividades.to_string(), egui::Color32::from_rgb(255, 165, 0));
                
                // Tarjeta de Viajes
                self.stat_card(ui, "🚌", "Viajes", stats.total_viajes.to_string(), egui::Color32::LIGHT_RED);
                ui.end_row();
            });
    }

    fn stat_card(&self, ui: &mut egui::Ui, icon: &str, title: &str, value: String, color: egui::Color32) {
        egui::Frame::none()
            .fill(color.linear_multiply(0.1))
            .rounding(egui::Rounding::same(8.0))
            .inner_margin(egui::Margin::same(15.0))
            .show(ui, |ui| {
                ui.set_min_size(egui::vec2(150.0, 100.0));
                ui.vertical_centered(|ui| {
                    ui.label(egui::RichText::new(icon).size(24.0));
                    ui.add_space(5.0);
                    ui.label(egui::RichText::new(&value).size(28.0).color(color));
                    ui.label(egui::RichText::new(title).size(12.0).color(egui::Color32::GRAY));
                });
            });
    }

    fn show_charts(&self, ui: &mut egui::Ui, stats: &DashboardStats) {
        ui.horizontal(|ui| {
            // Columna izquierda - Distribución por macrosector
            ui.vertical(|ui| {
                ui.set_min_width(ui.available_width() / 2.0 - 10.0);
                ui.label("Distribución de Personas por Macrosector");
                ui.add_space(10.0);
                
                egui::Frame::none()
                    .fill(egui::Color32::from_gray(30))
                    .rounding(egui::Rounding::same(5.0))
                    .inner_margin(egui::Margin::same(10.0))
                    .show(ui, |ui| {
                        ui.set_min_height(200.0);
                        
                        if stats.personas_por_macro.is_empty() {
                            ui.centered_and_justified(|ui| {
                                ui.label("No hay datos disponibles");
                            });
                        } else {
                            for (macro_name, count) in &stats.personas_por_macro {
                                ui.horizontal(|ui| {
                                    ui.label(macro_name);
                                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                        ui.label(count.to_string());
                                    });
                                });
                                ui.separator();
                            }
                        }
                    });
            });

            ui.add_space(20.0);

            // Columna derecha - Estadísticas del mes
            ui.vertical(|ui| {
                ui.set_min_width(ui.available_width());
                ui.label("Actividad del Mes Actual");
                ui.add_space(10.0);
                
                egui::Frame::none()
                    .fill(egui::Color32::from_gray(30))
                    .rounding(egui::Rounding::same(5.0))
                    .inner_margin(egui::Margin::same(10.0))
                    .show(ui, |ui| {
                        ui.set_min_height(200.0);
                        
                        egui::Grid::new("monthly_stats")
                            .num_columns(2)
                            .spacing([10.0, 10.0])
                            .show(ui, |ui| {
                                ui.label("🎯 Actividades este mes:");
                                ui.label(stats.actividades_mes_actual.to_string());
                                ui.end_row();
                                
                                ui.label("👤 Nuevas personas:");
                                ui.label(stats.nuevas_personas_mes.to_string());
                                ui.end_row();
                            });
                    });
            });
        });
    }

    fn show_placeholder(&self, ui: &mut egui::Ui) {
        ui.with_layout(egui::Layout::centered_and_justified(egui::Direction::TopDown), |ui| {
            ui.label("📊");
            ui.label("Haga clic en 'Actualizar datos' para cargar las estadísticas");
        });
    }

    fn refresh_stats(&mut self) {
        self.loading = true;
        self.last_refresh = std::time::Instant::now();
        
        let (tx, rx) = mpsc::unbounded_channel();
        self.stats_receiver = Some(rx);
        
        let db_manager = self.db_manager.clone();
        tokio::spawn(async move {
            let db = db_manager.lock().await;
            let result = db.get_dashboard_stats().await;
            
            match result {
                Ok(stats) => {
                    let _ = tx.send(Ok(stats));
                }
                Err(e) => {
                    let _ = tx.send(Err(format!("Error al cargar estadísticas: {}", e)));
                }
            }
        });
    }
}
