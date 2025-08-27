use eframe::egui;

pub struct AboutView {
    show_tech_details: bool,
}

impl AboutView {
    pub fn new() -> Self {
        Self {
            show_tech_details: false,
        }
    }

    pub fn show(&mut self, ui: &mut egui::Ui) {
        ui.heading("ⓘ Acerca de la Aplicación");
        ui.add_space(20.0);

        egui::ScrollArea::vertical().show(ui, |ui| {
            // Información principal
            egui::Frame::none()
                .fill(egui::Color32::from_gray(25))
                .rounding(egui::Rounding::same(8.0))
                .inner_margin(egui::Margin::same(20.0))
                .show(ui, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.add_space(10.0);
                        ui.label(egui::RichText::new("🏢").size(48.0));
                        ui.add_space(10.0);
                        ui.heading("Gestor Base de Datos Comunitaria");
                        ui.add_space(5.0);
                        ui.label("Versión 1.0.0");
                        ui.add_space(15.0);
                    });

                    ui.separator();
                    ui.add_space(15.0);

                    ui.label(egui::RichText::new("Descripción").size(16.0).strong());
                    ui.add_space(5.0);
                    ui.label("Esta aplicación de escritorio permite gestionar de forma eficiente la base de datos comunitaria, proporcionando herramientas para consultar, insertar y administrar información de personas mayores, organizaciones comunitarias, actividades y demás entidades relacionadas.");
                    
                    ui.add_space(15.0);

                    ui.label(egui::RichText::new("Características Principales").size(16.0).strong());
                    ui.add_space(5.0);
                    
                    let features = [
                        " Autenticación segura con credenciales de base de datos",
                        " Dashboard con estadísticas en tiempo real",
                        " Sistema de consultas avanzadas con filtros",
                        " Formularios de inserción para todos los tipos de datos",
                        " Gestión de macrosectores y unidades vecinales",
                        " Registro de personas mayores y organizaciones",
                        " Seguimiento de actividades y eventos",
                        " Administración de viajes comunitarios",
                        " Interfaz moderna y responsiva",
                        " Rendimiento optimizado para grandes volúmenes de datos",
                    ];

                    for feature in &features {
                        ui.horizontal(|ui| {
                            ui.add_space(10.0);
                            ui.label(*feature);
                        });
                    }
                });

            ui.add_space(20.0);

            // Información técnica
            egui::CollapsingHeader::new("Detalles Técnicos")
                .default_open(self.show_tech_details)
                .show(ui, |ui| {
                    self.show_tech_details = true;
                    
                    egui::Grid::new("tech_grid")
                        .num_columns(2)
                        .spacing([15.0, 8.0])
                        .show(ui, |ui| {
                            ui.strong("Lenguaje:");
                            ui.label("Rust");
                            ui.end_row();

                            ui.strong("Framework UI:");
                            ui.label("egui");
                            ui.end_row();

                            ui.strong("Base de Datos:");
                            ui.label("PostgreSQL");
                            ui.end_row();

                            ui.strong("Driver DB:");
                            ui.label("tokio-postgres");
                            ui.end_row();

                            ui.strong("Runtime Async:");
                            ui.label("Tokio");
                            ui.end_row();

                            ui.strong("Serialización:");
                            ui.label("Serde");
                            ui.end_row();

                            ui.strong("Logging:");
                            ui.label("log + env_logger");
                            ui.end_row();

                            ui.strong("Manejo de Fechas:");
                            ui.label("chrono");
                            ui.end_row();
                        });

                    ui.add_space(10.0);
                    
                    ui.label(egui::RichText::new("Arquitectura").strong());
                    ui.label("La aplicación está diseñada con una arquitectura modular que separa claramente:");
                    
                    let arch_points = [
                        "• Capa de presentación (UI) con egui",
                        "• Capa de lógica de negocio (models)",
                        "• Capa de acceso a datos (database)",
                        "• Utilidades y helpers (utils)",
                    ];

                    for point in &arch_points {
                        ui.horizontal(|ui| {
                            ui.add_space(10.0);
                            ui.label(*point);
                        });
                    }
                });

            ui.add_space(20.0);

            // Información de contacto o desarrollo
            egui::Frame::none()
                .fill(egui::Color32::from_gray(20))
                .rounding(egui::Rounding::same(5.0))
                .inner_margin(egui::Margin::same(15.0))
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.label("Fecha de desarrollo:");
                        ui.label("Agosto 2025");
                    });
                    
                    ui.horizontal(|ui| {
                        ui.label("Estado:");
                        ui.colored_label(egui::Color32::GREEN, "Activo");
                    });

                    ui.horizontal(|ui| {
                        ui.label("Licencia:");
                        ui.label("MIT License");
                    });
                });

            ui.add_space(20.0);

            // Footer
            ui.separator();
            ui.add_space(10.0);
            ui.with_layout(egui::Layout::centered_and_justified(egui::Direction::LeftToRight), |ui| {
                ui.small("© 2025 - Gestor Base de Datos Comunitaria");
            });
            ui.add_space(10.0);
        });
    }
}
