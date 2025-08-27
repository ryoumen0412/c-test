use eframe::egui;
use std::sync::Arc;
use tokio::sync::{Mutex, mpsc};
use crate::database::DatabaseManager;
use crate::models::*;

// Función para formatear RUT automáticamente
fn format_rut(input: &str) -> String {
    // Remover todo excepto números y K/k
    let clean: String = input.chars()
        .filter(|c| c.is_numeric() || c.to_uppercase().next() == Some('K'))
        .collect();
    
    if clean.is_empty() {
        return String::new();
    }
    
    // Si es muy corto, devolver como está
    if clean.len() < 2 {
        return clean;
    }
    
    // Separar número del dígito verificador
    let (numero, dv) = clean.split_at(clean.len() - 1);
    
    // Formatear solo si tenemos al menos 1 dígito más el verificador
    if numero.is_empty() {
        return clean;
    }
    
    format!("{}-{}", numero, dv.to_uppercase())
}

// Función para validar formato RUT chileno
fn validate_rut_format(rut: &str) -> bool {
    // Patrón: 7-8 dígitos, guión, dígito verificador (0-9 o K)
    let re = regex::Regex::new(r"^[0-9]{7,8}-[0-9Kk]$").unwrap();
    re.is_match(rut)
}

// Tipos de actualizaciones de catálogo
#[derive(Debug, Clone)]
enum CatalogUpdate {
    Generos(Vec<Genero>),
    Nacionalidades(Vec<Nacionalidad>),
    UnidadesVecinales(Vec<UnidadVecinal>),
    MacroSectores(Vec<MacroSector>),
}

#[derive(Debug, Clone, PartialEq)]
enum InsertionType {
    Persona,
    Organizacion,
    Actividad,
    MacroSector,
    UnidadVecinal,
    Taller,
}

pub struct InsertionsView {
    db_manager: Arc<Mutex<DatabaseManager>>,
    insertion_type: InsertionType,
    
    // Formularios
    persona_form: PersonaForm,
    organizacion_form: OrganizacionForm,
    actividad_form: ActividadForm,
    macro_sector_form: MacroSectorForm,
    unidad_vecinal_form: UnidadVecinalForm,
    taller_form: TallerForm,
    
    // Catálogos
    generos: Vec<Genero>,
    nacionalidades: Vec<Nacionalidad>,
    unidades_vecinales: Vec<UnidadVecinal>,
    macro_sectores: Vec<MacroSector>,
    
    // Estado
    loading: bool,
    catalogs_loaded: bool,
    
    // Canales asíncronos para inserciones
    insertion_receiver: Option<mpsc::UnboundedReceiver<Result<String, String>>>,
    
    // Canales para cargar catálogos
    catalog_receiver: Option<mpsc::UnboundedReceiver<CatalogUpdate>>,
}

#[derive(Debug, Clone, Default)]
struct PersonaForm {
    rut: String,
    primer_nombre: String,
    segundo_nombre: String,
    primer_apellido: String,
    segundo_apellido: String,
    genero_id: Option<i32>,
    nacionalidad_id: Option<i32>,
    fecha_nacimiento: String,
    direccion: String,
    email: String,
    unidad_vecinal_id: Option<i32>,
}

#[derive(Debug, Clone, Default)]
struct OrganizacionForm {
    nombre: String,
    direccion: String,
    fecha_constitucion: String,
    personalidad_juridica: String,
    email: String,
    unidad_vecinal_id: Option<i32>,
}

#[derive(Debug, Clone, Default)]
struct ActividadForm {
    nombre: String,
    fecha_inicio: String,
    fecha_fin: String,
    descripcion: String,
    unidad_vecinal_id: Option<i32>,
}

#[derive(Debug, Clone, Default)]
struct MacroSectorForm {
    nombre: String,
}

#[derive(Debug, Clone, Default)]
struct UnidadVecinalForm {
    nombre: String,
    macro_sector_id: Option<i32>,
}

#[derive(Debug, Clone, Default)]
struct TallerForm {
    nombre: String,
}

impl InsertionsView {
    pub fn new(db_manager: Arc<Mutex<DatabaseManager>>) -> Self {
        Self {
            db_manager,
            insertion_type: InsertionType::Persona,
            persona_form: PersonaForm::default(),
            organizacion_form: OrganizacionForm::default(),
            actividad_form: ActividadForm::default(),
            macro_sector_form: MacroSectorForm::default(),
            unidad_vecinal_form: UnidadVecinalForm::default(),
            taller_form: TallerForm::default(),
            generos: Vec::new(),
            nacionalidades: Vec::new(),
            unidades_vecinales: Vec::new(),
            macro_sectores: Vec::new(),
            loading: false,
            catalogs_loaded: false,
            insertion_receiver: None,
            catalog_receiver: None,
        }
    }

    pub fn check_insertion_result(&mut self) -> Option<(bool, String)> {
        if let Some(receiver) = &mut self.insertion_receiver {
            if let Ok(result) = receiver.try_recv() {
                self.loading = false;
                self.insertion_receiver = None;
                match result {
                    Ok(success_msg) => {
                        // Limpiar formulario correspondiente después del éxito
                        match self.insertion_type {
                            InsertionType::Persona => self.persona_form = PersonaForm::default(),
                            InsertionType::Organizacion => self.organizacion_form = OrganizacionForm::default(),
                            InsertionType::Actividad => self.actividad_form = ActividadForm::default(),
                            InsertionType::MacroSector => self.macro_sector_form = MacroSectorForm::default(),
                            InsertionType::UnidadVecinal => self.unidad_vecinal_form = UnidadVecinalForm::default(),
                            InsertionType::Taller => self.taller_form = TallerForm::default(),
                        }
                        
                        // Refrescar catálogos después de inserción exitosa
                        self.load_catalogs();
                        
                        return Some((true, success_msg));
                    }
                    Err(error_msg) => {
                        return Some((false, error_msg));
                    }
                }
            }
        }
        None
    }

    // Función para procesar actualizaciones de catálogo
    pub fn check_catalog_updates(&mut self) {
        if let Some(receiver) = &mut self.catalog_receiver {
            while let Ok(update) = receiver.try_recv() {
                match update {
                    CatalogUpdate::Generos(generos) => {
                        self.generos = generos;
                    }
                    CatalogUpdate::Nacionalidades(nacionalidades) => {
                        self.nacionalidades = nacionalidades;
                    }
                    CatalogUpdate::UnidadesVecinales(unidades) => {
                        self.unidades_vecinales = unidades;
                    }
                    CatalogUpdate::MacroSectores(sectores) => {
                        self.macro_sectores = sectores;
                    }
                }
            }
        }
    }

    pub fn show(&mut self, ui: &mut egui::Ui) -> Option<(bool, String)> {
        // Check for async insertion results
        if let Some((success, message)) = self.check_insertion_result() {
            return Some((success, message));
        }
        
        // Check for catalog updates
        self.check_catalog_updates();

        ui.heading("➕ Inserción de Datos");
        ui.add_space(10.0);

        // Cargar catálogos solo una vez al inicio
        if !self.catalogs_loaded {
            self.catalogs_loaded = true;
            self.load_catalogs();
        }

        // Selector de tipo de inserción
        ui.horizontal(|ui| {
            ui.label("Tipo de registro:");
            egui::ComboBox::from_id_source("insertion_type")
                .selected_text(format!("{:?}", self.insertion_type))
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.insertion_type, InsertionType::Persona, "Persona Mayor");
                    ui.selectable_value(&mut self.insertion_type, InsertionType::Organizacion, "Organización");
                    ui.selectable_value(&mut self.insertion_type, InsertionType::Actividad, "Actividad");
                    ui.selectable_value(&mut self.insertion_type, InsertionType::MacroSector, "Macrosector");
                    ui.selectable_value(&mut self.insertion_type, InsertionType::UnidadVecinal, "Unidad Vecinal");
                    ui.selectable_value(&mut self.insertion_type, InsertionType::Taller, "Taller");
                });
        });

        ui.add_space(15.0);

        // Formularios
        egui::ScrollArea::vertical().show(ui, |ui| {
            match self.insertion_type {
                InsertionType::Persona => self.show_persona_form(ui),
                InsertionType::Organizacion => self.show_organizacion_form(ui),
                InsertionType::Actividad => self.show_actividad_form(ui),
                InsertionType::MacroSector => self.show_macro_sector_form(ui),
                InsertionType::UnidadVecinal => self.show_unidad_vecinal_form(ui),
                InsertionType::Taller => self.show_taller_form(ui),
            }
        });
        
        None
    }

    fn show_persona_form(&mut self, ui: &mut egui::Ui) {
        egui::Frame::none()
            .fill(egui::Color32::from_gray(25))
            .rounding(egui::Rounding::same(5.0))
            .inner_margin(egui::Margin::same(15.0))
            .show(ui, |ui| {
                ui.heading("👤 Nueva Persona Mayor");
                ui.add_space(10.0);

                egui::Grid::new("persona_form")
                    .num_columns(2)
                    .spacing([15.0, 10.0])
                    .show(ui, |ui| {
                        ui.label("RUT:");
                        ui.horizontal(|ui| {
                            let response = ui.text_edit_singleline(&mut self.persona_form.rut);
                            
                            // Formatear RUT automáticamente al perder foco
                            if response.lost_focus() {
                                self.persona_form.rut = format_rut(&self.persona_form.rut);
                            }
                            
                            ui.small("(ej: 12345678-9)");
                        });
                        ui.end_row();

                        ui.label("Primer Nombre:");
                        ui.text_edit_singleline(&mut self.persona_form.primer_nombre);
                        ui.end_row();

                        ui.label("Segundo Nombre:");
                        ui.text_edit_singleline(&mut self.persona_form.segundo_nombre);
                        ui.end_row();

                        ui.label("Primer Apellido:");
                        ui.text_edit_singleline(&mut self.persona_form.primer_apellido);
                        ui.end_row();

                        ui.label("Segundo Apellido:");
                        ui.text_edit_singleline(&mut self.persona_form.segundo_apellido);
                        ui.end_row();

                        ui.label("Género:");
                        egui::ComboBox::from_id_source("persona_genero")
                            .selected_text(
                                self.persona_form.genero_id
                                    .and_then(|id| self.generos.iter().find(|g| g.gen_id == id))
                                    .map(|g| g.gen_genero.clone())
                                    .unwrap_or_else(|| "Seleccionar...".to_string())
                            )
                            .show_ui(ui, |ui| {
                                for genero in &self.generos {
                                    ui.selectable_value(
                                        &mut self.persona_form.genero_id,
                                        Some(genero.gen_id),
                                        &genero.gen_genero
                                    );
                                }
                            });
                        ui.end_row();

                        ui.label("Nacionalidad:");
                        egui::ComboBox::from_id_source("persona_nacionalidad")
                            .selected_text(
                                self.persona_form.nacionalidad_id
                                    .and_then(|id| self.nacionalidades.iter().find(|n| n.nac_id == id))
                                    .map(|n| n.nac_nacionalidad.clone())
                                    .unwrap_or_else(|| "Seleccionar...".to_string())
                            )
                            .show_ui(ui, |ui| {
                                for nacionalidad in &self.nacionalidades {
                                    ui.selectable_value(
                                        &mut self.persona_form.nacionalidad_id,
                                        Some(nacionalidad.nac_id),
                                        &nacionalidad.nac_nacionalidad
                                    );
                                }
                            });
                        ui.end_row();

                        ui.label("Fecha de Nacimiento:");
                        ui.horizontal(|ui| {
                            ui.text_edit_singleline(&mut self.persona_form.fecha_nacimiento);
                            ui.small("(dd/mm/yyyy)");
                        });
                        ui.end_row();

                        ui.label("Dirección:");
                        ui.text_edit_singleline(&mut self.persona_form.direccion);
                        ui.end_row();

                        ui.label("Email:");
                        ui.text_edit_singleline(&mut self.persona_form.email);
                        ui.end_row();

                        ui.label("Unidad Vecinal:");
                        egui::ComboBox::from_id_source("persona_uv")
                            .selected_text(
                                self.persona_form.unidad_vecinal_id
                                    .and_then(|id| self.unidades_vecinales.iter().find(|u| u.uv_id == id))
                                    .map(|u| u.uv_nombre.clone())
                                    .unwrap_or_else(|| "Seleccionar...".to_string())
                            )
                            .show_ui(ui, |ui| {
                                for uv in &self.unidades_vecinales {
                                    ui.selectable_value(
                                        &mut self.persona_form.unidad_vecinal_id,
                                        Some(uv.uv_id),
                                        &uv.uv_nombre
                                    );
                                }
                            });
                        ui.end_row();
                    });

                ui.add_space(20.0);

                ui.horizontal(|ui| {
                    if ui.button("💾 Guardar Persona").clicked() {
                        if self.validate_persona_form() {
                            self.save_persona();
                        } else {
                            // Aquí podrías mostrar un mensaje de error específico
                            if self.persona_form.email.trim().len() > 0 && (!self.persona_form.email.contains('@') || !self.persona_form.email.contains('.')) {
                                println!("Email inválido");
                            }
                        }
                    }

                    if ui.button("🧹 Limpiar Formulario").clicked() {
                        self.persona_form = PersonaForm::default();
                    }

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if self.loading {
                            ui.add(egui::widgets::Spinner::new().size(16.0));
                            ui.label("Guardando...");
                        }
                    });
                });
            });
    }

    fn show_organizacion_form(&mut self, ui: &mut egui::Ui) {
        egui::Frame::none()
            .fill(egui::Color32::from_gray(25))
            .rounding(egui::Rounding::same(5.0))
            .inner_margin(egui::Margin::same(15.0))
            .show(ui, |ui| {
                ui.heading("🏢 Nueva Organización Comunitaria");
                ui.add_space(10.0);

                egui::Grid::new("org_form")
                    .num_columns(2)
                    .spacing([15.0, 10.0])
                    .show(ui, |ui| {
                        ui.label("Nombre:");
                        ui.text_edit_singleline(&mut self.organizacion_form.nombre);
                        ui.end_row();

                        ui.label("Dirección:");
                        ui.text_edit_singleline(&mut self.organizacion_form.direccion);
                        ui.end_row();

                        ui.label("Fecha Constitución:");
                        ui.horizontal(|ui| {
                            ui.text_edit_singleline(&mut self.organizacion_form.fecha_constitucion);
                            ui.small("(dd/mm/yyyy)");
                        });
                        ui.end_row();

                        ui.label("Personalidad Jurídica:");
                        ui.text_edit_singleline(&mut self.organizacion_form.personalidad_juridica);
                        ui.end_row();

                        ui.label("Email:");
                        ui.text_edit_singleline(&mut self.organizacion_form.email);
                        ui.end_row();

                        ui.label("Unidad Vecinal:");
                        egui::ComboBox::from_id_source("org_uv")
                            .selected_text(
                                self.organizacion_form.unidad_vecinal_id
                                    .and_then(|id| self.unidades_vecinales.iter().find(|u| u.uv_id == id))
                                    .map(|u| u.uv_nombre.clone())
                                    .unwrap_or_else(|| "Seleccionar...".to_string())
                            )
                            .show_ui(ui, |ui| {
                                for uv in &self.unidades_vecinales {
                                    ui.selectable_value(
                                        &mut self.organizacion_form.unidad_vecinal_id,
                                        Some(uv.uv_id),
                                        &uv.uv_nombre
                                    );
                                }
                            });
                        ui.end_row();
                    });

                ui.add_space(20.0);

                ui.horizontal(|ui| {
                    if ui.button("💾 Guardar Organización").clicked() {
                        self.save_organizacion();
                    }

                    if ui.button("🧹 Limpiar Formulario").clicked() {
                        self.organizacion_form = OrganizacionForm::default();
                    }
                });
            });
    }

    fn show_actividad_form(&mut self, ui: &mut egui::Ui) {
        egui::Frame::none()
            .fill(egui::Color32::from_gray(25))
            .rounding(egui::Rounding::same(5.0))
            .inner_margin(egui::Margin::same(15.0))
            .show(ui, |ui| {
                ui.heading("🎯 Nueva Actividad");
                ui.add_space(10.0);

                egui::Grid::new("act_form")
                    .num_columns(2)
                    .spacing([15.0, 10.0])
                    .show(ui, |ui| {
                        ui.label("Nombre:");
                        ui.text_edit_singleline(&mut self.actividad_form.nombre);
                        ui.end_row();

                        ui.label("Fecha Inicio:");
                        ui.text_edit_singleline(&mut self.actividad_form.fecha_inicio);
                        ui.end_row();

                        ui.label("Fecha Fin:");
                        ui.text_edit_singleline(&mut self.actividad_form.fecha_fin);
                        ui.end_row();

                        ui.label("Descripción:");
                        ui.text_edit_multiline(&mut self.actividad_form.descripcion);
                        ui.end_row();

                        ui.label("Unidad Vecinal:");
                        egui::ComboBox::from_id_source("act_uv")
                            .selected_text(
                                self.actividad_form.unidad_vecinal_id
                                    .and_then(|id| self.unidades_vecinales.iter().find(|u| u.uv_id == id))
                                    .map(|u| u.uv_nombre.clone())
                                    .unwrap_or_else(|| "Seleccionar...".to_string())
                            )
                            .show_ui(ui, |ui| {
                                for uv in &self.unidades_vecinales {
                                    ui.selectable_value(
                                        &mut self.actividad_form.unidad_vecinal_id,
                                        Some(uv.uv_id),
                                        &uv.uv_nombre
                                    );
                                }
                            });
                        ui.end_row();
                    });

                ui.add_space(20.0);

                ui.horizontal(|ui| {
                    if ui.button("💾 Guardar Actividad").clicked() {
                        self.save_actividad();
                    }

                    if ui.button("🧹 Limpiar Formulario").clicked() {
                        self.actividad_form = ActividadForm::default();
                    }
                });
            });
    }

    fn show_macro_sector_form(&mut self, ui: &mut egui::Ui) {
        egui::Frame::none()
            .fill(egui::Color32::from_gray(25))
            .rounding(egui::Rounding::same(5.0))
            .inner_margin(egui::Margin::same(15.0))
            .show(ui, |ui| {
                ui.heading("🏘️ Nuevo Macrosector");
                ui.add_space(10.0);

                ui.horizontal(|ui| {
                    ui.label("Nombre:");
                    ui.text_edit_singleline(&mut self.macro_sector_form.nombre);
                });

                ui.add_space(20.0);

                ui.horizontal(|ui| {
                    if ui.button("💾 Guardar Macrosector").clicked() {
                        self.save_macro_sector();
                    }

                    if ui.button("🧹 Limpiar").clicked() {
                        self.macro_sector_form = MacroSectorForm::default();
                    }
                });
            });
    }

    fn show_unidad_vecinal_form(&mut self, ui: &mut egui::Ui) {
        egui::Frame::none()
            .fill(egui::Color32::from_gray(25))
            .rounding(egui::Rounding::same(5.0))
            .inner_margin(egui::Margin::same(15.0))
            .show(ui, |ui| {
                ui.heading("🏘️ Nueva Unidad Vecinal");
                ui.add_space(10.0);

                egui::Grid::new("uv_form")
                    .num_columns(2)
                    .spacing([15.0, 10.0])
                    .show(ui, |ui| {
                        ui.label("Nombre:");
                        ui.text_edit_singleline(&mut self.unidad_vecinal_form.nombre);
                        ui.end_row();

                        ui.label("Macrosector:");
                        egui::ComboBox::from_id_source("uv_macro")
                            .selected_text(
                                self.unidad_vecinal_form.macro_sector_id
                                    .and_then(|id| self.macro_sectores.iter().find(|m| m.mac_id == id))
                                    .map(|m| m.mac_nombre.clone())
                                    .unwrap_or_else(|| "Seleccionar...".to_string())
                            )
                            .show_ui(ui, |ui| {
                                for macro_sector in &self.macro_sectores {
                                    ui.selectable_value(
                                        &mut self.unidad_vecinal_form.macro_sector_id,
                                        Some(macro_sector.mac_id),
                                        &macro_sector.mac_nombre
                                    );
                                }
                            });
                        ui.end_row();
                    });

                ui.add_space(20.0);

                ui.horizontal(|ui| {
                    if ui.button("💾 Guardar Unidad Vecinal").clicked() {
                        self.save_unidad_vecinal();
                    }

                    if ui.button("🧹 Limpiar").clicked() {
                        self.unidad_vecinal_form = UnidadVecinalForm::default();
                    }
                });
            });
    }

    fn show_taller_form(&mut self, ui: &mut egui::Ui) {
        egui::Frame::none()
            .fill(egui::Color32::from_gray(25))
            .rounding(egui::Rounding::same(5.0))
            .inner_margin(egui::Margin::same(15.0))
            .show(ui, |ui| {
                ui.heading("🎨 Nuevo Taller");
                ui.add_space(10.0);

                ui.horizontal(|ui| {
                    ui.label("Nombre:");
                    ui.text_edit_singleline(&mut self.taller_form.nombre);
                });

                ui.add_space(20.0);

                ui.horizontal(|ui| {
                    if ui.button("💾 Guardar Taller").clicked() {
                        self.save_taller();
                    }

                    if ui.button("🧹 Limpiar").clicked() {
                        self.taller_form = TallerForm::default();
                    }
                });
            });
    }

    fn load_catalogs(&mut self) {
        // Crear canal para recibir actualizaciones de catálogo
        let (tx, rx) = mpsc::unbounded_channel();
        self.catalog_receiver = Some(rx);
        
        let db_manager = self.db_manager.clone();
        
        // Cargar géneros
        let tx_generos = tx.clone();
        let db_generos = db_manager.clone();
        tokio::spawn(async move {
            let db = db_generos.lock().await;
            if let Ok(generos) = db.get_generos().await {
                let _ = tx_generos.send(CatalogUpdate::Generos(generos));
            }
        });
        
        // Cargar nacionalidades
        let tx_nacionalidades = tx.clone();
        let db_nacionalidades = db_manager.clone();
        tokio::spawn(async move {
            let db = db_nacionalidades.lock().await;
            if let Ok(nacionalidades) = db.get_nacionalidades().await {
                let _ = tx_nacionalidades.send(CatalogUpdate::Nacionalidades(nacionalidades));
            }
        });
        
        // Cargar unidades vecinales
        let tx_unidades = tx.clone();
        let db_unidades = db_manager.clone();
        tokio::spawn(async move {
            let db = db_unidades.lock().await;
            if let Ok(unidades) = db.get_unidades_vecinales().await {
                let _ = tx_unidades.send(CatalogUpdate::UnidadesVecinales(unidades));
            }
        });
        
        // Cargar macrosectores
        let tx_sectores = tx;
        let db_sectores = db_manager;
        tokio::spawn(async move {
            let db = db_sectores.lock().await;
            if let Ok(sectores) = db.get_macro_sectores().await {
                let _ = tx_sectores.send(CatalogUpdate::MacroSectores(sectores));
            }
        });
    }

    fn save_persona(&mut self) {
        if self.validate_persona_form() {
            self.loading = true;
            
            let (tx, rx) = mpsc::unbounded_channel();
            self.insertion_receiver = Some(rx);
            
            // Crear objeto PersonaMayor desde el formulario
            let persona = PersonaMayor {
                per_id: 0, // Se asignará automáticamente
                per_rut: self.persona_form.rut.clone(),
                per_prinombre: self.persona_form.primer_nombre.clone(),
                per_segnombre: if self.persona_form.segundo_nombre.is_empty() { 
                    None 
                } else { 
                    Some(self.persona_form.segundo_nombre.clone()) 
                },
                per_priapellido: self.persona_form.primer_apellido.clone(),
                per_segapellido: if self.persona_form.segundo_apellido.is_empty() { 
                    None 
                } else { 
                    Some(self.persona_form.segundo_apellido.clone()) 
                },
                per_genid: self.persona_form.genero_id.unwrap_or(1),
                per_nacid: self.persona_form.nacionalidad_id.unwrap_or(1),
                per_fechadenac: chrono::NaiveDate::parse_from_str(&self.persona_form.fecha_nacimiento, "%Y-%m-%d")
                    .unwrap_or_else(|_| chrono::NaiveDate::from_ymd_opt(1950, 1, 1).unwrap()),
                per_direccion: self.persona_form.direccion.clone(),
                per_email: if self.persona_form.email.trim().is_empty() { 
                    None 
                } else {
                    // Validación muy básica - dejar que PostgreSQL haga la validación final
                    let email = self.persona_form.email.trim();
                    if email.len() > 0 {
                        Some(email.to_string())
                    } else {
                        None
                    }
                },
                per_uvid: self.persona_form.unidad_vecinal_id.unwrap_or(1),
                gen_genero: None,
                nac_nacionalidad: None,
                uv_nombre: None,
            };
            
            let db_manager = self.db_manager.clone();
            tokio::spawn(async move {
                let db = db_manager.lock().await;
                let result = db.insert_persona(&persona).await;
                
                match result {
                    Ok(id) => {
                        let _ = tx.send(Ok(format!("Persona guardada exitosamente con ID: {}", id)));
                    }
                    Err(e) => {
                        let _ = tx.send(Err(format!("Error al guardar persona: {}", e)));
                    }
                }
            });
        }
    }

    fn save_organizacion(&mut self) {
        if self.validate_organizacion_form() {
            // Crear canal para comunicación asíncrona
            let (tx, rx) = mpsc::unbounded_channel();
            self.insertion_receiver = Some(rx);
            
            let organizacion = OrganizacionComunitaria {
                org_id: 0, // Se generará automáticamente
                org_nombre: self.organizacion_form.nombre.clone(),
                org_direccion: self.organizacion_form.direccion.clone(),
                org_uvid: self.organizacion_form.unidad_vecinal_id.unwrap_or(1),
                org_fechaconst: chrono::NaiveDate::parse_from_str(&self.organizacion_form.fecha_constitucion, "%Y-%m-%d")
                    .unwrap_or_else(|_| chrono::NaiveDate::from_ymd_opt(2024, 1, 1).unwrap()),
                org_perjuridica: self.organizacion_form.personalidad_juridica.clone(),
                org_email: if self.organizacion_form.email.trim().is_empty() { 
                    None 
                } else {
                    Some(self.organizacion_form.email.trim().to_string())
                },
                // Campos adicionales que no están en el formulario
                uv_nombre: None,
            };
            
            let db_manager = self.db_manager.clone();
            tokio::spawn(async move {
                let db = db_manager.lock().await;
                let result = db.insert_organizacion(&organizacion).await;
                
                match result {
                    Ok(id) => {
                        let _ = tx.send(Ok(format!("Organización guardada exitosamente con ID: {}", id)));
                    }
                    Err(e) => {
                        let _ = tx.send(Err(format!("Error al guardar organización: {}", e)));
                    }
                }
            });
        }
    }

    fn save_actividad(&mut self) {
        if self.validate_actividad_form() {
            // Crear canal para comunicación asíncrona
            let (tx, rx) = mpsc::unbounded_channel();
            self.insertion_receiver = Some(rx);
            
            let actividad = Actividad {
                act_id: 0, // Se generará automáticamente
                act_nombre: self.actividad_form.nombre.clone(),
                act_uvid: self.actividad_form.unidad_vecinal_id.unwrap_or(1),
                act_fecha_ini: chrono::NaiveDate::parse_from_str(&self.actividad_form.fecha_inicio, "%Y-%m-%d")
                    .unwrap_or_else(|_| chrono::NaiveDate::from_ymd_opt(2024, 1, 1).unwrap()),
                act_fecha_fin: if self.actividad_form.fecha_fin.trim().is_empty() {
                    None
                } else {
                    chrono::NaiveDate::parse_from_str(&self.actividad_form.fecha_fin, "%Y-%m-%d").ok()
                },
                act_descripcion: if self.actividad_form.descripcion.trim().is_empty() {
                    None
                } else {
                    Some(self.actividad_form.descripcion.clone())
                },
                // Campos adicionales
                uv_nombre: None,
            };
            
            let db_manager = self.db_manager.clone();
            tokio::spawn(async move {
                let db = db_manager.lock().await;
                let result = db.insert_actividad(&actividad).await;
                
                match result {
                    Ok(id) => {
                        let _ = tx.send(Ok(format!("Actividad guardada exitosamente con ID: {}", id)));
                    }
                    Err(e) => {
                        let _ = tx.send(Err(format!("Error al guardar actividad: {}", e)));
                    }
                }
            });
        }
    }

    fn save_macro_sector(&mut self) {
        if !self.macro_sector_form.nombre.trim().is_empty() {
            // Crear canal para comunicación asíncrona
            let (tx, rx) = mpsc::unbounded_channel();
            self.insertion_receiver = Some(rx);
            
            let nombre = self.macro_sector_form.nombre.trim().to_string();
            let db_manager = self.db_manager.clone();
            
            tokio::spawn(async move {
                let db = db_manager.lock().await;
                let result = db.insert_macro_sector(&nombre).await;
                
                match result {
                    Ok(id) => {
                        let _ = tx.send(Ok(format!("Macrosector guardado exitosamente con ID: {}", id)));
                    }
                    Err(e) => {
                        let _ = tx.send(Err(format!("Error al guardar macrosector: {}", e)));
                    }
                }
            });
        }
    }

    fn save_unidad_vecinal(&mut self) {
        if self.validate_unidad_vecinal_form() {
            // Crear canal para comunicación asíncrona
            let (tx, rx) = mpsc::unbounded_channel();
            self.insertion_receiver = Some(rx);
            
            let nombre = self.unidad_vecinal_form.nombre.trim().to_string();
            let macro_sector_id = self.unidad_vecinal_form.macro_sector_id.unwrap_or(1);
            let db_manager = self.db_manager.clone();
            
            tokio::spawn(async move {
                let db = db_manager.lock().await;
                let result = db.insert_unidad_vecinal(&nombre, macro_sector_id).await;
                
                match result {
                    Ok(id) => {
                        let _ = tx.send(Ok(format!("Unidad Vecinal guardada exitosamente con ID: {}", id)));
                    }
                    Err(e) => {
                        let _ = tx.send(Err(format!("Error al guardar unidad vecinal: {}", e)));
                    }
                }
            });
        }
    }

    fn save_taller(&mut self) {
        if !self.taller_form.nombre.trim().is_empty() {
            // Crear canal para comunicación asíncrona
            let (tx, rx) = mpsc::unbounded_channel();
            self.insertion_receiver = Some(rx);
            
            let nombre = self.taller_form.nombre.trim().to_string();
            let db_manager = self.db_manager.clone();
            
            tokio::spawn(async move {
                let db = db_manager.lock().await;
                let result = db.insert_taller(&nombre).await;
                
                match result {
                    Ok(id) => {
                        let _ = tx.send(Ok(format!("Taller guardado exitosamente con ID: {}", id)));
                    }
                    Err(e) => {
                        let _ = tx.send(Err(format!("Error al guardar taller: {}", e)));
                    }
                }
            });
        }
    }

    fn validate_persona_form(&self) -> bool {
        let email_valid = if self.persona_form.email.trim().is_empty() {
            true // Email vacío es válido (será NULL)
        } else {
            // Validación muy básica - debe tener @ y .
            let email = self.persona_form.email.trim();
            email.contains('@') && email.contains('.') && email.len() > 5
        };
        
        // Validar formato de RUT chileno usando la función específica
        let rut_valid = validate_rut_format(self.persona_form.rut.trim());
        
        rut_valid &&
        !self.persona_form.primer_nombre.is_empty() &&
        !self.persona_form.primer_apellido.is_empty() &&
        self.persona_form.genero_id.is_some() &&
        self.persona_form.nacionalidad_id.is_some() &&
        !self.persona_form.fecha_nacimiento.is_empty() &&
        !self.persona_form.direccion.is_empty() &&
        self.persona_form.unidad_vecinal_id.is_some() &&
        email_valid
    }

    fn validate_organizacion_form(&self) -> bool {
        !self.organizacion_form.nombre.is_empty() &&
        !self.organizacion_form.direccion.is_empty() &&
        !self.organizacion_form.fecha_constitucion.is_empty() &&
        !self.organizacion_form.personalidad_juridica.is_empty() &&
        self.organizacion_form.unidad_vecinal_id.is_some()
    }

    fn validate_actividad_form(&self) -> bool {
        !self.actividad_form.nombre.is_empty() &&
        !self.actividad_form.fecha_inicio.is_empty() &&
        self.actividad_form.unidad_vecinal_id.is_some()
    }

    fn validate_unidad_vecinal_form(&self) -> bool {
        !self.unidad_vecinal_form.nombre.is_empty() &&
        self.unidad_vecinal_form.macro_sector_id.is_some()
    }
}
