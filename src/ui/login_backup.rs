use eframe::egui;
use std::path::PathBuf;
use crate::models::DatabaseConfig;
use crate::ui::app::App;

const CONFIG_FILE: &str = "db_config.json";

pub struct LoginView {
    pub config: DatabaseConfig,
    pub connecting: bool,
    show_password: bool,
    connection_error: Option<String>,
}

impl LoginView {
    pub fn new() -> Self {
        let config = Self::load_config().unwrap_or_default();
        Self {
            config,
            connecting: false,
            show_password: false,
            connection_error: None,
        }
    }

    fn get_config_path() -> PathBuf {
        let mut path = std::env::current_exe().unwrap();
        path.pop(); // Remover el nombre del ejecutable
        path.push(CONFIG_FILE);
        path
    }

    fn load_config() -> Result<DatabaseConfig, Box<dyn std::error::Error>> {
        let path = Self::get_config_path();
        if path.exists() {
            let content = std::fs::read_to_string(path)?;
            let config: DatabaseConfig = serde_json::from_str(&content)?;
            Ok(config)
        } else {
            Err("Config file not found".into())
        }
    }

    fn save_config(&self) -> Result<(), Box<dyn std::error::Error>> {
        let path = Self::get_config_path();
        let content = serde_json::to_string_pretty(&self.config)?;
        std::fs::write(path, content)?;
        Ok(())
    }

    pub fn show(&mut self, ui: &mut egui::Ui, _app: &mut App) -> Option<bool> {
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

                    // Mostrar error de conexión si existe
                    if let Some(ref error) = self.connection_error {
                        ui.colored_label(egui::Color32::RED, format!("Error de conexión: {}", error));
                        ui.add_space(10.0);
                    }

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
                                    let ip_response = ui.text_edit_singleline(&mut self.config.host);
                                    ui.end_row();

                                    ui.label("Puerto:");
                                    ui.add(egui::DragValue::new(&mut self.config.port).range(1..=65535));
                                    ui.end_row();

                                    ui.label("Base de Datos:");
                                    let db_response = ui.text_edit_singleline(&mut self.config.database);
                                    ui.end_row();

                                    ui.label("Usuario:");
                                    let user_response = ui.text_edit_singleline(&mut self.config.username);
                                    ui.end_row();

                                    ui.label("Contraseña:");
                                    ui.horizontal(|ui| {
                                        let password_response = if self.show_password {
                                            ui.text_edit_singleline(&mut self.config.password)
                                        } else {
                                            ui.add(egui::TextEdit::singleline(&mut self.config.password).password(true))
                                        };
                                        
                                        // Detectar Enter en cualquier campo
                                        if (ip_response.lost_focus() || db_response.lost_focus() || 
                                            user_response.lost_focus() || password_response.lost_focus()) && 
                                           ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                                            if !self.connecting {
                                                connection_result = Some(true);
                                            }
                                        }

                                        if ui.small_button(if self.show_password { "Ocultar" } else { "Mostrar" }).clicked() {
                                            self.show_password = !self.show_password;
                                        }
                                    });
                                    ui.end_row();
                                });

                            ui.add_space(20.0);

                            ui.horizontal(|ui| {
                                if ui.button("Conectar").clicked() && !self.connecting {
                                    connection_result = Some(true);
                                }

                                if self.connecting {
                                    ui.add(egui::widgets::Spinner::new().size(16.0));
                                    ui.label("Conectando...");
                                }
                            });
                        });
                });
            },
        );

        // Procesar resultado de conexión
        if let Some(should_connect) = connection_result {
            if should_connect && !self.connecting {
                self.connecting = true;
                self.connection_error = None;
                
                match self.attempt_connection() {
                    Ok(success) => {
                        if success {
                            // Guardar configuración en caso de éxito
                            if let Err(e) = self.save_config() {
                                eprintln!("Error al guardar configuración: {}", e);
                            }
                            self.connecting = false;
                            return Some(true);
                        } else {
                            self.connection_error = Some("Credenciales incorrectas o servidor no disponible".to_string());
                            self.connecting = false;
                        }
                    }
                    Err(e) => {
                        self.connection_error = Some(format!("Error de autenticación: {}", e));
                        self.connecting = false;
                    }
                }
            }
        }

        None
    }

    fn attempt_connection(&mut self) -> Result<bool, String> {
        // Validaciones básicas
        if self.config.host.is_empty() || 
           self.config.username.is_empty() || 
           self.config.database.is_empty() {
            return Err("Todos los campos son obligatorios excepto la contraseña".to_string());
        }

        // Validación básica de puerto
        if self.config.port == 0 || self.config.port > 65535 {
            return Err("Puerto debe estar entre 1 y 65535".to_string());
        }

        Ok(true) // Simulamos éxito por ahora
    }
}
