use eframe::egui;
use std::sync::Arc;
use tokio::sync::{Mutex, mpsc};
use crate::database::DatabaseManager;
use crate::models::*;
use crate::utils;

#[derive(Debug, Clone, PartialEq)]
enum QueryType {
    Personas,
    Organizaciones,
    Actividades,
    Viajes,
}

#[derive(Debug)]
enum QueryResult {
    Personas(Vec<PersonaMayor>),
    Organizaciones(Vec<OrganizacionComunitaria>),
    Actividades(Vec<Actividad>),
    Viajes(Vec<Viaje>),
}

pub struct QueriesView {
    db_manager: Arc<Mutex<DatabaseManager>>,
    query_type: QueryType,
    
    // Filtros
    persona_filter: PersonaFilter,
    organizacion_filter: OrganizacionFilter,
    actividad_filter: ActividadFilter,
    
    // Resultados
    personas_results: Vec<PersonaMayor>,
    organizaciones_results: Vec<OrganizacionComunitaria>,
    actividades_results: Vec<Actividad>,
    viajes_results: Vec<Viaje>,
    
    // Catálogos para filtros
    generos: Vec<Genero>,
    nacionalidades: Vec<Nacionalidad>,
    unidades_vecinales: Vec<UnidadVecinal>,
    macro_sectores: Vec<MacroSector>,
    
    // Estado
    loading: bool,
    catalogs_loaded: bool,
    
    // Canales asíncronos
    query_receiver: Option<mpsc::UnboundedReceiver<Result<QueryResult, String>>>,
}

impl QueriesView {
    pub fn new(db_manager: Arc<Mutex<DatabaseManager>>) -> Self {
        let instance = Self {
            db_manager,
            query_type: QueryType::Personas,
            persona_filter: PersonaFilter::default(),
            organizacion_filter: OrganizacionFilter::default(),
            actividad_filter: ActividadFilter::default(),
            personas_results: Vec::new(),
            organizaciones_results: Vec::new(),
            actividades_results: Vec::new(),
            viajes_results: Vec::new(),
            generos: Vec::new(),
            nacionalidades: Vec::new(),
            unidades_vecinales: Vec::new(),
            macro_sectores: Vec::new(),
            loading: false,
            catalogs_loaded: false,
            query_receiver: None,
        };
        
        // NO ejecutar consultas automáticas aquí - se harán cuando haya conexión
        
        instance
    }

    // Función pública para inicializar datos una vez conectado
    pub fn initialize_data(&mut self) {
        self.load_catalogs();
        self.execute_initial_query();
    }

    pub fn check_query_result(&mut self) -> bool {
        if let Some(receiver) = &mut self.query_receiver {
            if let Ok(result) = receiver.try_recv() {
                self.loading = false;
                match result {
                    Ok(query_result) => {
                        match query_result {
                            QueryResult::Personas(personas) => {
                                self.personas_results = personas;
                            }
                            QueryResult::Organizaciones(organizaciones) => {
                                self.organizaciones_results = organizaciones;
                            }
                            QueryResult::Actividades(actividades) => {
                                self.actividades_results = actividades;
                            }
                            QueryResult::Viajes(viajes) => {
                                self.viajes_results = viajes;
                            }
                        }
                        self.query_receiver = None;
                        return true;
                    }
                    Err(_error_msg) => {
                        // En caso de error, limpiar resultados
                        self.personas_results.clear();
                        self.organizaciones_results.clear();
                        self.actividades_results.clear();
                        self.viajes_results.clear();
                        self.query_receiver = None;
                        return true;
                    }
                }
            }
        }
        false
    }

    pub fn show(&mut self, ui: &mut egui::Ui) {
        // Check for async query results
        self.check_query_result();

        ui.heading("🔍 Consultas con Filtros");
        ui.add_space(10.0);

        if !self.catalogs_loaded {
            self.load_catalogs();
        }

        // Selector de tipo de consulta
        ui.horizontal(|ui| {
            ui.label("Tipo de consulta:");
            let previous_query_type = self.query_type.clone();
            egui::ComboBox::from_id_source("query_type")
                .selected_text(format!("{:?}", self.query_type))
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.query_type, QueryType::Personas, "Personas Mayores");
                    ui.selectable_value(&mut self.query_type, QueryType::Organizaciones, "Organizaciones");
                    ui.selectable_value(&mut self.query_type, QueryType::Actividades, "Actividades");
                    ui.selectable_value(&mut self.query_type, QueryType::Viajes, "Viajes");
                });
            
            // Si cambió el tipo de consulta, ejecutar automáticamente
            if previous_query_type != self.query_type {
                self.execute_auto_query();
            }
        });

        ui.add_space(15.0);

        // Panel de filtros
        egui::CollapsingHeader::new("🎛️ Filtros")
            .default_open(true)
            .show(ui, |ui| {
                match self.query_type {
                    QueryType::Personas => self.show_persona_filters(ui),
                    QueryType::Organizaciones => self.show_organizacion_filters(ui),
                    QueryType::Actividades => self.show_actividad_filters(ui),
                    QueryType::Viajes => self.show_viaje_filters(ui),
                }
            });

        ui.add_space(10.0);

        // Botón de búsqueda
        ui.horizontal(|ui| {
            if ui.button("🔍 Buscar").clicked() {
                self.execute_query();
            }
            
            if ui.button("🧹 Limpiar filtros").clicked() {
                self.clear_filters();
            }

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if self.loading {
                    ui.add(egui::widgets::Spinner::new().size(16.0));
                    ui.label("Buscando...");
                }
            });
        });

        ui.add_space(15.0);
        ui.separator();
        ui.add_space(15.0);

        // Resultados
        self.show_results(ui);
    }

    fn show_persona_filters(&mut self, ui: &mut egui::Ui) {
        egui::Grid::new("persona_filters")
            .num_columns(4)
            .spacing([10.0, 10.0])
            .show(ui, |ui| {
                ui.label("Nombre:");
                ui.text_edit_singleline(&mut self.persona_filter.nombre);
                
                ui.label("Apellido:");
                ui.text_edit_singleline(&mut self.persona_filter.apellido);
                ui.end_row();

                ui.label("RUT:");
                ui.text_edit_singleline(&mut self.persona_filter.rut);
                
                ui.label("Género:");
                egui::ComboBox::from_id_source("genero_filter")
                    .selected_text(
                        self.persona_filter.genero_id
                            .and_then(|id| self.generos.iter().find(|g| g.gen_id == id))
                            .map(|g| g.gen_genero.clone())
                            .unwrap_or_else(|| "Todos".to_string())
                    )
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.persona_filter.genero_id, None, "Todos");
                        for genero in &self.generos {
                            ui.selectable_value(
                                &mut self.persona_filter.genero_id,
                                Some(genero.gen_id),
                                &genero.gen_genero
                            );
                        }
                    });
                ui.end_row();

                ui.label("Macrosector:");
                egui::ComboBox::from_id_source("macro_filter")
                    .selected_text(
                        self.persona_filter.macro_sector_id
                            .and_then(|id| self.macro_sectores.iter().find(|m| m.mac_id == id))
                            .map(|m| m.mac_nombre.clone())
                            .unwrap_or_else(|| "Todos".to_string())
                    )
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.persona_filter.macro_sector_id, None, "Todos");
                        for macro_sector in &self.macro_sectores {
                            ui.selectable_value(
                                &mut self.persona_filter.macro_sector_id,
                                Some(macro_sector.mac_id),
                                &macro_sector.mac_nombre
                            );
                        }
                    });

                ui.label("Unidad Vecinal:");
                egui::ComboBox::from_id_source("uv_filter")
                    .selected_text(
                        self.persona_filter.unidad_vecinal_id
                            .and_then(|id| self.unidades_vecinales.iter().find(|u| u.uv_id == id))
                            .map(|u| u.uv_nombre.clone())
                            .unwrap_or_else(|| "Todas".to_string())
                    )
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.persona_filter.unidad_vecinal_id, None, "Todas");
                        for uv in &self.unidades_vecinales {
                            // Filtrar por macrosector si está seleccionado
                            if let Some(mac_id) = self.persona_filter.macro_sector_id {
                                if uv.uv_macid != mac_id {
                                    continue;
                                }
                            }
                            ui.selectable_value(
                                &mut self.persona_filter.unidad_vecinal_id,
                                Some(uv.uv_id),
                                &uv.uv_nombre
                            );
                        }
                    });
                ui.end_row();
            });
    }

    fn show_organizacion_filters(&mut self, ui: &mut egui::Ui) {
        egui::Grid::new("org_filters")
            .num_columns(2)
            .spacing([10.0, 10.0])
            .show(ui, |ui| {
                ui.label("Nombre:");
                ui.text_edit_singleline(&mut self.organizacion_filter.nombre);
                ui.end_row();

                ui.label("Unidad Vecinal:");
                egui::ComboBox::from_id_source("org_uv_filter")
                    .selected_text(
                        self.organizacion_filter.unidad_vecinal_id
                            .and_then(|id| self.unidades_vecinales.iter().find(|u| u.uv_id == id))
                            .map(|u| u.uv_nombre.clone())
                            .unwrap_or_else(|| "Todas".to_string())
                    )
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.organizacion_filter.unidad_vecinal_id, None, "Todas");
                        for uv in &self.unidades_vecinales {
                            ui.selectable_value(
                                &mut self.organizacion_filter.unidad_vecinal_id,
                                Some(uv.uv_id),
                                &uv.uv_nombre
                            );
                        }
                    });
                ui.end_row();
            });
    }

    fn show_actividad_filters(&mut self, ui: &mut egui::Ui) {
        egui::Grid::new("act_filters")
            .num_columns(2)
            .spacing([10.0, 10.0])
            .show(ui, |ui| {
                ui.label("Nombre:");
                ui.text_edit_singleline(&mut self.actividad_filter.nombre);
                ui.end_row();

                ui.label("Unidad Vecinal:");
                egui::ComboBox::from_id_source("act_uv_filter")
                    .selected_text(
                        self.actividad_filter.unidad_vecinal_id
                            .and_then(|id| self.unidades_vecinales.iter().find(|u| u.uv_id == id))
                            .map(|u| u.uv_nombre.clone())
                            .unwrap_or_else(|| "Todas".to_string())
                    )
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.actividad_filter.unidad_vecinal_id, None, "Todas");
                        for uv in &self.unidades_vecinales {
                            ui.selectable_value(
                                &mut self.actividad_filter.unidad_vecinal_id,
                                Some(uv.uv_id),
                                &uv.uv_nombre
                            );
                        }
                    });
                ui.end_row();
            });
    }

    fn show_viaje_filters(&mut self, ui: &mut egui::Ui) {
        ui.label("Filtros de viajes disponibles próximamente...");
    }

    fn show_results(&self, ui: &mut egui::Ui) {
        match self.query_type {
            QueryType::Personas => self.show_personas_results(ui),
            QueryType::Organizaciones => self.show_organizaciones_results(ui),
            QueryType::Actividades => self.show_actividades_results(ui),
            QueryType::Viajes => self.show_viajes_results(ui),
        }
    }

    fn show_personas_results(&self, ui: &mut egui::Ui) {
        ui.label(format!("Resultados: {} personas encontradas", self.personas_results.len()));
        ui.add_space(10.0);

        egui::ScrollArea::vertical().show(ui, |ui| {
            egui::Grid::new("personas_results")
                .striped(true)
                .spacing([10.0, 8.0])
                .show(ui, |ui| {
                    // Encabezados
                    ui.strong("RUT");
                    ui.strong("Nombre");
                    ui.strong("Apellidos");
                    ui.strong("Edad");
                    ui.strong("Género");
                    ui.strong("UV");
                    ui.end_row();

                    // Datos
                    for persona in &self.personas_results {
                        ui.label(&persona.per_rut);
                        ui.label(&persona.per_prinombre);
                        ui.label(&format!("{} {}", 
                            persona.per_priapellido, 
                            persona.per_segapellido.as_deref().unwrap_or("")
                        ));
                        ui.label(utils::calculate_age(&persona.per_fechadenac).to_string());
                        ui.label(persona.gen_genero.as_deref().unwrap_or("N/A"));
                        ui.label(persona.uv_nombre.as_deref().unwrap_or("N/A"));
                        ui.end_row();
                    }
                });
        });
    }

    fn show_organizaciones_results(&self, ui: &mut egui::Ui) {
        ui.label(format!("Resultados: {} organizaciones encontradas", self.organizaciones_results.len()));
        ui.add_space(10.0);

        egui::ScrollArea::vertical().show(ui, |ui| {
            egui::Grid::new("org_results")
                .striped(true)
                .spacing([10.0, 8.0])
                .show(ui, |ui| {
                    // Encabezados
                    ui.strong("Nombre");
                    ui.strong("Dirección");
                    ui.strong("Fecha Const.");
                    ui.strong("UV");
                    ui.end_row();

                    // Datos
                    for org in &self.organizaciones_results {
                        ui.label(&org.org_nombre);
                        ui.label(utils::truncate_text(&org.org_direccion, 30));
                        ui.label(utils::format_date(&org.org_fechaconst));
                        ui.label(org.uv_nombre.as_deref().unwrap_or("N/A"));
                        ui.end_row();
                    }
                });
        });
    }

    fn show_actividades_results(&self, ui: &mut egui::Ui) {
        ui.label(format!("Resultados: {} actividades encontradas", self.actividades_results.len()));
        ui.add_space(10.0);

        egui::ScrollArea::vertical().show(ui, |ui| {
            egui::Grid::new("act_results")
                .striped(true)
                .spacing([10.0, 8.0])
                .show(ui, |ui| {
                    // Encabezados
                    ui.strong("Nombre");
                    ui.strong("Fecha Inicio");
                    ui.strong("Fecha Fin");
                    ui.strong("UV");
                    ui.end_row();

                    // Datos
                    for actividad in &self.actividades_results {
                        ui.label(&actividad.act_nombre);
                        ui.label(utils::format_date(&actividad.act_fecha_ini));
                        ui.label(utils::format_optional_date(&actividad.act_fecha_fin));
                        ui.label(actividad.uv_nombre.as_deref().unwrap_or("N/A"));
                        ui.end_row();
                    }
                });
        });
    }

    fn show_viajes_results(&self, ui: &mut egui::Ui) {
        ui.label("Resultados de viajes mostrarán aquí...");
    }

    fn load_catalogs(&mut self) {
        self.catalogs_loaded = true;
        
        // Simular datos de catálogos para demo
        self.generos = vec![
            Genero { gen_id: 1, gen_genero: "Masculino".to_string() },
            Genero { gen_id: 2, gen_genero: "Femenino".to_string() },
            Genero { gen_id: 3, gen_genero: "Otro".to_string() },
        ];

        self.nacionalidades = vec![
            Nacionalidad { nac_id: 1, nac_nacionalidad: "Chilena".to_string() },
            Nacionalidad { nac_id: 2, nac_nacionalidad: "Peruana".to_string() },
            Nacionalidad { nac_id: 3, nac_nacionalidad: "Boliviana".to_string() },
        ];

        self.macro_sectores = vec![
            MacroSector { mac_id: 1, mac_nombre: "Centro".to_string() },
            MacroSector { mac_id: 2, mac_nombre: "Norte".to_string() },
            MacroSector { mac_id: 3, mac_nombre: "Sur".to_string() },
        ];

        self.unidades_vecinales = vec![
            UnidadVecinal { uv_id: 1, uv_nombre: "Villa Los Álamos".to_string(), uv_macid: 1, mac_nombre: Some("Centro".to_string()) },
            UnidadVecinal { uv_id: 2, uv_nombre: "Barrio Norte".to_string(), uv_macid: 2, mac_nombre: Some("Norte".to_string()) },
            UnidadVecinal { uv_id: 3, uv_nombre: "Villa Sur".to_string(), uv_macid: 3, mac_nombre: Some("Sur".to_string()) },
        ];
    }

    fn execute_query(&mut self) {
        self.loading = true;
        
        let (tx, rx) = mpsc::unbounded_channel();
        self.query_receiver = Some(rx);
        
        let db_manager = self.db_manager.clone();
        let query_type = self.query_type.clone();
        let persona_filter = self.persona_filter.clone();
        let organizacion_filter = self.organizacion_filter.clone();
        let actividad_filter = self.actividad_filter.clone();
        
        tokio::spawn(async move {
            let db = db_manager.lock().await;
            let result = match query_type {
                QueryType::Personas => {
                    match db.get_personas_mayores(&persona_filter).await {
                        Ok(personas) => Ok(QueryResult::Personas(personas)),
                        Err(e) => Err(format!("Error al consultar personas: {}", e)),
                    }
                }
                QueryType::Organizaciones => {
                    match db.get_organizaciones(&organizacion_filter).await {
                        Ok(organizaciones) => Ok(QueryResult::Organizaciones(organizaciones)),
                        Err(e) => Err(format!("Error al consultar organizaciones: {}", e)),
                    }
                }
                QueryType::Actividades => {
                    match db.get_actividades(&actividad_filter).await {
                        Ok(actividades) => Ok(QueryResult::Actividades(actividades)),
                        Err(e) => Err(format!("Error al consultar actividades: {}", e)),
                    }
                }
                QueryType::Viajes => {
                    // Por ahora, devolver lista vacía
                    Ok(QueryResult::Viajes(Vec::new()))
                }
            };
            
            let _ = tx.send(result);
        });
    }

    // Función para cargar todos los datos inicialmente sin filtros
    fn execute_initial_query(&mut self) {
        self.loading = true;
        
        let (tx, rx) = mpsc::unbounded_channel();
        self.query_receiver = Some(rx);
        
        let db_manager = self.db_manager.clone();
        
        tokio::spawn(async move {
            let db = db_manager.lock().await;
            
            // Verificar si hay conexión antes de ejecutar la consulta
            match db.test_connection().await {
                Ok(false) | Err(_) => {
                    let _ = tx.send(Err("No hay conexión a la base de datos".to_string()));
                    return;
                }
                Ok(true) => {}
            }
            
            // Cargar todos los datos de personas sin filtros
            let empty_persona_filter = PersonaFilter::default();
            let result = match db.get_personas_mayores(&empty_persona_filter).await {
                Ok(personas) => Ok(QueryResult::Personas(personas)),
                Err(e) => Err(format!("Error al cargar datos iniciales: {}", e)),
            };
            
            let _ = tx.send(result);
        });
    }

    // Función para ejecutar consulta automática cuando cambia el tipo
    fn execute_auto_query(&mut self) {
        self.loading = true;
        
        let (tx, rx) = mpsc::unbounded_channel();
        self.query_receiver = Some(rx);
        
        let db_manager = self.db_manager.clone();
        let query_type = self.query_type.clone();
        
        tokio::spawn(async move {
            let db = db_manager.lock().await;
            
            // Verificar si hay conexión antes de ejecutar la consulta
            match db.test_connection().await {
                Ok(false) | Err(_) => {
                    let _ = tx.send(Err("No hay conexión a la base de datos".to_string()));
                    return;
                }
                Ok(true) => {}
            }
            
            let result = match query_type {
                QueryType::Personas => {
                    let empty_filter = PersonaFilter::default();
                    match db.get_personas_mayores(&empty_filter).await {
                        Ok(personas) => Ok(QueryResult::Personas(personas)),
                        Err(e) => Err(format!("Error al consultar personas: {}", e)),
                    }
                }
                QueryType::Organizaciones => {
                    let empty_filter = OrganizacionFilter::default();
                    match db.get_organizaciones(&empty_filter).await {
                        Ok(organizaciones) => Ok(QueryResult::Organizaciones(organizaciones)),
                        Err(e) => Err(format!("Error al consultar organizaciones: {}", e)),
                    }
                }
                QueryType::Actividades => {
                    let empty_filter = ActividadFilter::default();
                    match db.get_actividades(&empty_filter).await {
                        Ok(actividades) => Ok(QueryResult::Actividades(actividades)),
                        Err(e) => Err(format!("Error al consultar actividades: {}", e)),
                    }
                }
                QueryType::Viajes => {
                    Ok(QueryResult::Viajes(Vec::new()))
                }
            };
            
            let _ = tx.send(result);
        });
    }

    fn clear_filters(&mut self) {
        self.persona_filter = PersonaFilter::default();
        self.organizacion_filter = OrganizacionFilter::default();
        self.actividad_filter = ActividadFilter::default();
    }
}
