use eframe::egui;
use std::sync::Arc;
use tokio::sync::{Mutex, mpsc};
use crate::database::DatabaseManager;
use crate::models::DatabaseConfig;
use super::{login::LoginView, dashboard::DashboardView, sidebar::Sidebar, queries::QueriesView, insertions::InsertionsView, about::AboutView};

#[derive(Debug, Clone, PartialEq)]
pub enum AppState {
    Login,
    Dashboard,
    Queries,
    Insertions,
    About,
}

pub struct App {
    pub state: AppState,
    pub db_manager: Arc<Mutex<DatabaseManager>>,
    
    // Views
    login_view: LoginView,
    dashboard_view: DashboardView,
    sidebar: Sidebar,
    queries_view: QueriesView,
    insertions_view: InsertionsView,
    about_view: AboutView,
    
    // App state
    is_connected: bool,
    error_message: Option<String>,
    success_message: Option<String>,
    
    // Async connection handling
    connection_receiver: Option<mpsc::UnboundedReceiver<Result<String, String>>>,
}

impl App {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let db_manager = Arc::new(Mutex::new(DatabaseManager::new()));
        
        Self {
            state: AppState::Login,
            db_manager: db_manager.clone(),
            login_view: LoginView::new(),
            dashboard_view: DashboardView::new(db_manager.clone()),
            sidebar: Sidebar::new(),
            queries_view: QueriesView::new(db_manager.clone()),
            insertions_view: InsertionsView::new(db_manager.clone()),
            about_view: AboutView::new(),
            is_connected: false,
            error_message: None,
            success_message: None,
            connection_receiver: None,
        }
    }

    pub fn set_state(&mut self, state: AppState) {
        self.state = state;
        self.clear_messages();
    }

    pub fn set_connected(&mut self, connected: bool) {
        self.is_connected = connected;
        if connected && self.state == AppState::Login {
            self.set_state(AppState::Dashboard);
        } else if !connected {
            self.set_state(AppState::Login);
        }
    }

    pub fn set_error(&mut self, message: String) {
        self.error_message = Some(message);
        self.success_message = None;
    }

    pub fn set_success(&mut self, message: String) {
        self.success_message = Some(message);
        self.error_message = None;
    }

    pub fn clear_messages(&mut self) {
        self.error_message = None;
        self.success_message = None;
    }

    pub fn start_connection(&mut self, config: DatabaseConfig) {
        let (tx, rx) = mpsc::unbounded_channel();
        self.connection_receiver = Some(rx);
        
        let db_manager = self.db_manager.clone();
        tokio::spawn(async move {
            let mut manager = db_manager.lock().await;
            let result = manager.connect(&config).await;
            
            match result {
                Ok(_) => {
                    // Test the connection
                    match manager.test_connection().await {
                        Ok(true) => {
                            let _ = tx.send(Ok("Conexión establecida exitosamente".to_string()));
                        }
                        Ok(false) => {
                            let _ = tx.send(Err("Error al probar la conexión".to_string()));
                        }
                        Err(e) => {
                            let _ = tx.send(Err(format!("Error en test de conexión: {}", e)));
                        }
                    }
                }
                Err(e) => {
                    let _ = tx.send(Err(format!("Error de conexión: {}", e)));
                }
            }
        });
    }

    pub fn check_connection_result(&mut self) -> bool {
        if let Some(receiver) = &mut self.connection_receiver {
            if let Ok(result) = receiver.try_recv() {
                self.login_view.connecting = false;
                match result {
                    Ok(success_msg) => {
                        self.set_connected(true);
                        self.set_success(success_msg);
                        self.connection_receiver = None;
                        return true;
                    }
                    Err(error_msg) => {
                        self.set_error(error_msg);
                        self.connection_receiver = None;
                    }
                }
            }
        }
        false
    }

    fn show_messages(&mut self, ui: &mut egui::Ui) {
        if let Some(ref error) = self.error_message.clone() {
            ui.colored_label(egui::Color32::RED, format!("❌ {}", error));
        }
        
        if let Some(ref success) = self.success_message.clone() {
            ui.colored_label(egui::Color32::GREEN, format!("✅ {}", success));
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Check for async connection results
        if self.check_connection_result() {
            ctx.request_repaint();
        }
        
        // Configurar el estilo
        let mut style = (*ctx.style()).clone();
        style.spacing.button_padding = egui::vec2(12.0, 8.0);
        style.spacing.item_spacing = egui::vec2(8.0, 8.0);
        ctx.set_style(style);

        match self.state {
            AppState::Login => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    self.login_view.show(ui, self);
                });
            }

                                                ui.label("Usuario:");
                                                ui.text_edit_singleline(&mut self.login_view.config.username);
                                                ui.end_row();

                                                ui.label("Contraseña:");
                                                ui.add(egui::TextEdit::singleline(&mut self.login_view.config.password).password(true));
                                                ui.end_row();
                                            });

                                        ui.add_space(20.0);

                                        ui.horizontal(|ui| {
                                            ui.with_layout(egui::Layout::centered_and_justified(egui::Direction::LeftToRight), |ui| {
                                                let button_text = if self.login_view.connecting { "Conectando..." } else { "🔗 Conectar" };
                                                let button = egui::Button::new(button_text)
                                                    .min_size(egui::vec2(120.0, 40.0));
                                                
                                                if ui.add_enabled(!self.login_view.connecting, button).clicked() {
                                                    if !self.login_view.config.host.is_empty() && 
                                                       !self.login_view.config.username.is_empty() && 
                                                       !self.login_view.config.database.is_empty() {
                                                        self.login_view.connecting = true;
                                                        let config = self.login_view.config.clone();
                                                        self.start_connection(config);
                                                        ctx.request_repaint();
                                                    } else {
                                                        self.set_error("Por favor complete todos los campos requeridos".to_string());
                                                    }
                                                }
                                            });
                                        });
                                    });
                            });
                        },
                    );
                });
            }
            _ => {
                // Layout principal con sidebar
                let mut new_state = None;
                egui::SidePanel::left("sidebar")
                    .resizable(false)
                    .min_width(200.0)
                    .max_width(200.0)
                    .show(ctx, |ui| {
                        new_state = self.sidebar.show(ui, &self.state);
                    });

                // Cambiar de estado si se seleccionó uno nuevo
                if let Some(state) = new_state {
                    self.set_state(state);
                }

                egui::CentralPanel::default().show(ctx, |ui| {
                    // Mostrar mensajes en la parte superior
                    if self.error_message.is_some() || self.success_message.is_some() {
                        ui.horizontal(|ui| {
                            self.show_messages(ui);
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                if ui.small_button("❌").clicked() {
                                    self.clear_messages();
                                }
                            });
                        });
                        ui.separator();
                    }

                    // Mostrar la vista correspondiente
                    match self.state {
                        AppState::Dashboard => {
                            if self.dashboard_view.check_stats_result() {
                                ctx.request_repaint();
                            }
                            self.dashboard_view.show(ui);
                        }
                        AppState::Queries => {
                            if self.queries_view.check_query_result() {
                                ctx.request_repaint();
                            }
                            self.queries_view.show(ui);
                        }
                        AppState::Insertions => {
                            if let Some((success, message)) = self.insertions_view.show(ui) {
                                if success {
                                    self.set_success(message);
                                } else {
                                    self.set_error(message);
                                }
                                ctx.request_repaint();
                            }
                        }
                        AppState::About => {
                            self.about_view.show(ui);
                        }
                        _ => {}
                    }
                });
            }
        }
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        // Cerrar conexión de base de datos al salir
        let db_manager = self.db_manager.clone();
        tokio::spawn(async move {
            let mut db = db_manager.lock().await;
            db.disconnect().await;
        });
    }
}
