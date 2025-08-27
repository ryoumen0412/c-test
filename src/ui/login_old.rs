use eframe::egui;
use crate::models::DatabaseConfig;
use crate::ui::app::App;

pub struct LoginView {
    pub config: DatabaseConfig,
    pub connecting: bool,
    show_password: bool,
}

impl LoginView {
    pub fn new() -> Self {
        Self {
            config: DatabaseConfig::default(),
            connecting: false,
            show_password: false,
        }
    }

    pub fn show(&mut self, ui: &mut egui::Ui, app: &mut App) -> Option<bool> {
        let mut connection_result = None;
        
        ui.with_layout(
            egui::Layout::centered_and_justified(egui::Direction::TopDown),
            |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(50.0);
                    
                    // Logo y título
                    ui.heading("GESTOR Base de Datos Comunitaria");
                    ui.add_space(20.0);
                    ui.label("Ingrese las credenciales de conexión a la base de datos");
                    ui.add_space(30.0);

                    // Formulario de login
                    egui::Frame::none()
                        .fill(egui::Color32::from_gray(40))
                        .rounding(egui::Rounding::same(10.0))
                        .inner_margin(egui::Margin::same(20.0))
                        .show(ui, |ui| {
                            ui.set_max_width(400.0);
                            
                            egui::Grid::new("login_grid")
                                .num_columns(2)
                                .spacing([10.0, 15.0])
                                .show(ui, |ui| {
                                    ui.label("IP Address:");
                                    ui.text_edit_singleline(&mut self.config.host);
                                    ui.end_row();

                                    ui.label("Puerto:");
                                        ui.add(egui::DragValue::new(&mut self.config.port).range(1..=65535));
                                        ui.end_row();

                                        ui.label("Base de Datos:");
                                        ui.text_edit_singleline(&mut self.config.database);
                                        ui.end_row();

                                        ui.label("Usuario:");
                                        ui.text_edit_singleline(&mut self.config.username);
                                        ui.end_row();

                                        ui.label("Contraseña:");
                                        ui.horizontal(|ui| {
                                            if self.show_password {
                                                ui.text_edit_singleline(&mut self.config.password);
                                            } else {
                                                ui.add(egui::TextEdit::singleline(&mut self.config.password).password(true));
                                            }
                                            if ui.small_button(if self.show_password { "👁" } else { "👁‍🗨" }).clicked() {
                                                self.show_password = !self.show_password;
                                            }
                                        });
                                        ui.end_row();
                                    });

                                ui.add_space(20.0);

                                ui.horizontal(|ui| {
                                    ui.with_layout(egui::Layout::centered_and_justified(egui::Direction::LeftToRight), |ui| {
                                        let button_text = if self.connecting { "Conectando..." } else { "🔗 Conectar" };
                                        let button = egui::Button::new(button_text)
                                            .min_size(egui::vec2(120.0, 40.0));
                                        
                                        if ui.add_enabled(!self.connecting, button).clicked() {
                                            connection_result = Some(self.attempt_connection());
                                        }
                                    });
                                });

                                ui.add_space(10.0);

                                // Mostrar configuración de prueba
                                ui.collapsing("🔧 Configuración de prueba", |ui| {
                                    ui.small("Haga clic aquí para usar configuración de prueba:");
                                    if ui.small_button("Configurar PostgreSQL local").clicked() {
                                        self.config = DatabaseConfig {
                                            host: "localhost".to_string(),
                                            port: 5432,
                                            username: "postgres".to_string(),
                                            password: "password".to_string(),
                                            database: "comunidad".to_string(),
                                        };
                                    }
                                });
                            });

                        ui.add_space(30.0);
                        
                    // Información adicional
                    ui.small("Esta aplicación requiere una base de datos PostgreSQL");
                    ui.small("con el esquema de datos comunitarios cargado.");
                });
            },
        );
        
        connection_result
    }

    fn attempt_connection(&mut self) -> bool {
        self.connecting = true;
        
        // Validar que todos los campos estén llenos
        if self.config.host.is_empty() || 
           self.config.username.is_empty() || 
           self.config.database.is_empty() {
            self.connecting = false;
            return false;
        }

        // Aquí normalmente haríamos la conexión real
        // Por ahora simularemos el éxito
        self.connecting = false;
        true
    }
}
