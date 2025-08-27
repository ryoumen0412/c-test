use serde::{Deserialize, Serialize};
use chrono::NaiveDate;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub database: String,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            host: "localhost".to_string(),
            port: 5432,
            username: "postgres".to_string(),
            password: "".to_string(),
            database: "comunidad".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MacroSector {
    pub mac_id: i32,
    pub mac_nombre: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnidadVecinal {
    pub uv_id: i32,
    pub uv_nombre: String,
    pub uv_macid: i32,
    pub mac_nombre: Option<String>, // Para joins
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Genero {
    pub gen_id: i32,
    pub gen_genero: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Nacionalidad {
    pub nac_id: i32,
    pub nac_nacionalidad: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrganizacionComunitaria {
    pub org_id: i32,
    pub org_nombre: String,
    pub org_direccion: String,
    pub org_uvid: i32,
    pub org_fechaconst: NaiveDate,
    pub org_perjuridica: String,
    pub org_email: Option<String>,
    pub uv_nombre: Option<String>, // Para joins
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonaMayor {
    pub per_id: i32,
    pub per_rut: String,
    pub per_prinombre: String,
    pub per_segnombre: Option<String>,
    pub per_priapellido: String,
    pub per_segapellido: Option<String>,
    pub per_genid: i32,
    pub per_nacid: i32,
    pub per_fechadenac: NaiveDate,
    pub per_direccion: String,
    pub per_email: Option<String>,
    pub per_uvid: i32,
    pub gen_genero: Option<String>, // Para joins
    pub nac_nacionalidad: Option<String>, // Para joins
    pub uv_nombre: Option<String>, // Para joins
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Taller {
    pub tal_id: i32,
    pub tal_nombre: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Actividad {
    pub act_id: i32,
    pub act_nombre: String,
    pub act_uvid: i32,
    pub act_fecha_ini: NaiveDate,
    pub act_fecha_fin: Option<NaiveDate>,
    pub act_descripcion: Option<String>,
    pub uv_nombre: Option<String>, // Para joins
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Viaje {
    pub via_id: i32,
    pub via_nombre: String,
    pub via_destino: String,
    pub via_fecha_salida: NaiveDate,
    pub via_fecha_regreso: Option<NaiveDate>,
    pub via_uvid: i32,
    pub uv_nombre: Option<String>, // Para joins
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Beneficio {
    pub ben_id: i32,
    pub ben_codigo: String,
    pub ben_descripcion: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CentroComunitario {
    pub cen_id: i32,
    pub cen_nombre: String,
    pub cen_direccion: String,
    pub cen_uvid: i32,
    pub uv_nombre: Option<String>, // Para joins
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Telefono {
    pub id: i32,
    pub entity_id: i32,
    pub tipo: String,
    pub numero: String,
}

// Estructuras para filtros de consultas
#[derive(Debug, Clone, Default)]
pub struct PersonaFilter {
    pub nombre: String,
    pub apellido: String,
    pub rut: String,
    pub genero_id: Option<i32>,
    #[allow(dead_code)]
    pub nacionalidad_id: Option<i32>,
    pub unidad_vecinal_id: Option<i32>,
    pub macro_sector_id: Option<i32>,
    #[allow(dead_code)]
    pub edad_min: Option<i32>,
    #[allow(dead_code)]
    pub edad_max: Option<i32>,
}

#[derive(Debug, Clone, Default)]
pub struct OrganizacionFilter {
    pub nombre: String,
    pub unidad_vecinal_id: Option<i32>,
    #[allow(dead_code)]
    pub macro_sector_id: Option<i32>,
    #[allow(dead_code)]
    pub fecha_const_desde: Option<NaiveDate>,
    #[allow(dead_code)]
    pub fecha_const_hasta: Option<NaiveDate>,
}

#[derive(Debug, Clone, Default)]
pub struct ActividadFilter {
    pub nombre: String,
    pub unidad_vecinal_id: Option<i32>,
    #[allow(dead_code)]
    pub macro_sector_id: Option<i32>,
    #[allow(dead_code)]
    pub fecha_desde: Option<NaiveDate>,
    #[allow(dead_code)]
    pub fecha_hasta: Option<NaiveDate>,
}

// Tipos para estadísticas del dashboard
#[derive(Debug, Clone, Default)]
pub struct DashboardStats {
    pub total_personas: i64,
    pub total_organizaciones: i64,
    pub total_actividades: i64,
    pub total_viajes: i64,
    pub personas_por_macro: Vec<(String, i64)>,
    pub actividades_mes_actual: i64,
    pub nuevas_personas_mes: i64,
}
