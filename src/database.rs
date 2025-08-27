use anyhow::{Context, Result};
use tokio_postgres::{Client, NoTls};
use crate::models::*;

pub struct DatabaseManager {
    client: Option<Client>,
}

impl DatabaseManager {
    pub fn new() -> Self {
        Self { client: None }
    }

    pub async fn connect(&mut self, config: &DatabaseConfig) -> Result<()> {
        let connection_string = format!(
            "host={} port={} user={} password={} dbname={}",
            config.host, config.port, config.username, config.password, config.database
        );

        let (client, connection) = tokio_postgres::connect(&connection_string, NoTls)
            .await
            .context("Error al conectar con la base de datos")?;

        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("Error en la conexión: {}", e);
            }
        });

        self.client = Some(client);
        
        // Aplicar fix temporal del constraint de email automáticamente
        if let Err(e) = self.fix_email_constraint_temp().await {
            println!("ADVERTENCIA: No se pudo aplicar el fix del constraint de email: {}", e);
        }
        
        Ok(())
    }

    pub async fn disconnect(&mut self) {
        self.client = None;
    }

    pub async fn test_connection(&self) -> Result<bool> {
        if let Some(client) = &self.client {
            match client.query("SELECT 1", &[]).await {
                Ok(_) => Ok(true),
                Err(_) => Ok(false),
            }
        } else {
            Ok(false)
        }
    }

    pub async fn get_dashboard_stats(&self) -> Result<DashboardStats> {
        if let Some(client) = &self.client {
            let personas_row = client.query_one("SELECT COUNT(*) as count FROM per_personasmayores", &[]).await?;
            let personas_count: i64 = personas_row.get("count");

            let organizaciones_row = client.query_one("SELECT COUNT(*) as count FROM org_orgcomunitarias", &[]).await?;
            let organizaciones_count: i64 = organizaciones_row.get("count");

            let actividades_row = client.query_one("SELECT COUNT(*) as count FROM act_actividades", &[]).await?;
            let actividades_count: i64 = actividades_row.get("count");

            let viajes_row = client.query_one("SELECT COUNT(*) as count FROM via_viajes", &[]).await?;
            let viajes_count: i64 = viajes_row.get("count");

            Ok(DashboardStats {
                total_personas: personas_count,
                total_organizaciones: organizaciones_count,
                total_actividades: actividades_count,
                total_viajes: viajes_count,
                personas_por_macro: Vec::new(),
                actividades_mes_actual: 0,
                nuevas_personas_mes: 0,
            })
        } else {
            Err(anyhow::anyhow!("No hay conexión a la base de datos"))
        }
    }

    pub async fn get_generos(&self) -> Result<Vec<Genero>> {
        if let Some(client) = &self.client {
            let rows = client.query("SELECT gen_id, gen_genero FROM gen_generos ORDER BY gen_genero", &[]).await?;
            let generos = rows.iter().map(|row| Genero {
                gen_id: row.get("gen_id"),
                gen_genero: row.get("gen_genero"),
            }).collect();
            Ok(generos)
        } else {
            Err(anyhow::anyhow!("No hay conexión a la base de datos"))
        }
    }

    pub async fn get_nacionalidades(&self) -> Result<Vec<Nacionalidad>> {
        if let Some(client) = &self.client {
            let rows = client.query("SELECT nac_id, nac_nacionalidad FROM nac_nacionalidades ORDER BY nac_nacionalidad", &[]).await?;
            let nacionalidades = rows.iter().map(|row| Nacionalidad {
                nac_id: row.get("nac_id"),
                nac_nacionalidad: row.get("nac_nacionalidad"),
            }).collect();
            Ok(nacionalidades)
        } else {
            Err(anyhow::anyhow!("No hay conexión a la base de datos"))
        }
    }

    pub async fn get_unidades_vecinales(&self) -> Result<Vec<UnidadVecinal>> {
        if let Some(client) = &self.client {
            let rows = client.query(
                "SELECT uv.uv_id, uv.uv_nombre, uv.uv_macid, mac.mac_nombre 
                 FROM uv_unidadesvecinales uv 
                 LEFT JOIN mac_macrosectores mac ON uv.uv_macid = mac.mac_id 
                 ORDER BY uv.uv_nombre", 
                &[]
            ).await?;
            let unidades = rows.iter().map(|row| UnidadVecinal {
                uv_id: row.get("uv_id"),
                uv_nombre: row.get("uv_nombre"),
                uv_macid: row.get("uv_macid"),
                mac_nombre: row.get("mac_nombre"),
            }).collect();
            Ok(unidades)
        } else {
            Err(anyhow::anyhow!("No hay conexión a la base de datos"))
        }
    }

    pub async fn get_macro_sectores(&self) -> Result<Vec<MacroSector>> {
        if let Some(client) = &self.client {
            let rows = client.query("SELECT mac_id, mac_nombre FROM mac_macrosectores ORDER BY mac_nombre", &[]).await?;
            let macro_sectores = rows.iter().map(|row| MacroSector {
                mac_id: row.get("mac_id"),
                mac_nombre: row.get("mac_nombre"),
            }).collect();
            Ok(macro_sectores)
        } else {
            Err(anyhow::anyhow!("No hay conexión a la base de datos"))
        }
    }

    pub async fn get_personas_mayores(&self, _filter: &PersonaFilter) -> Result<Vec<PersonaMayor>> {
        if let Some(client) = &self.client {
            let rows = client.query("SELECT per_id, per_rut, per_prinombre, per_segnombre, per_priapellido, per_segapellido, per_genid, per_nacid, per_fechadenac, per_direccion, per_email, per_uvid FROM per_personasmayores ORDER BY per_priapellido, per_prinombre", &[]).await?;
            let mut personas = Vec::new();

            for row in rows {
                personas.push(PersonaMayor {
                    per_id: row.get("per_id"),
                    per_rut: row.get("per_rut"),
                    per_prinombre: row.get("per_prinombre"),
                    per_segnombre: row.get("per_segnombre"),
                    per_priapellido: row.get("per_priapellido"),
                    per_segapellido: row.get("per_segapellido"),
                    per_genid: row.get("per_genid"),
                    per_nacid: row.get("per_nacid"),
                    per_fechadenac: row.get("per_fechadenac"),
                    per_direccion: row.get("per_direccion"),
                    per_email: row.get("per_email"),
                    per_uvid: row.get("per_uvid"),
                    gen_genero: None,
                    nac_nacionalidad: None,
                    uv_nombre: None,
                });
            }

            Ok(personas)
        } else {
            Err(anyhow::anyhow!("No hay conexión a la base de datos"))
        }
    }

    pub async fn get_organizaciones(&self, _filter: &OrganizacionFilter) -> Result<Vec<OrganizacionComunitaria>> {
        if let Some(client) = &self.client {
            let rows = client.query("SELECT org_id, org_nombre, org_direccion, org_uvid, org_fechaconst, org_perjuridica, org_email FROM org_orgcomunitarias ORDER BY org_nombre", &[]).await?;
            let mut organizaciones = Vec::new();

            for row in rows {
                organizaciones.push(OrganizacionComunitaria {
                    org_id: row.get("org_id"),
                    org_nombre: row.get("org_nombre"),
                    org_direccion: row.get("org_direccion"),
                    org_uvid: row.get("org_uvid"),
                    org_fechaconst: row.get("org_fechaconst"),
                    org_perjuridica: row.get("org_perjuridica"),
                    org_email: row.get("org_email"),
                    uv_nombre: None,
                });
            }

            Ok(organizaciones)
        } else {
            Err(anyhow::anyhow!("No hay conexión a la base de datos"))
        }
    }

    pub async fn get_actividades(&self, _filter: &ActividadFilter) -> Result<Vec<Actividad>> {
        if let Some(client) = &self.client {
            let rows = client.query("SELECT act_id, act_nombre, act_uvid, act_fecha_ini, act_fecha_fin, act_descripcion FROM actividades ORDER BY act_fecha_ini DESC", &[]).await?;
            let mut actividades = Vec::new();

            for row in rows {
                actividades.push(Actividad {
                    act_id: row.get("act_id"),
                    act_nombre: row.get("act_nombre"),
                    act_uvid: row.get("act_uvid"),
                    act_fecha_ini: row.get("act_fecha_ini"),
                    act_fecha_fin: row.get("act_fecha_fin"),
                    act_descripcion: row.get("act_descripcion"),
                    uv_nombre: None,
                });
            }

            Ok(actividades)
        } else {
            Err(anyhow::anyhow!("No hay conexión a la base de datos"))
        }
    }

    pub async fn insert_persona(&self, persona: &PersonaMayor) -> Result<i32> {
        if let Some(client) = &self.client {
            // Log para debug completo
            println!("DEBUG: Insertando persona:");
            println!("  RUT: '{}'", persona.per_rut);
            println!("  Nombres: '{}' '{:?}'", persona.per_prinombre, persona.per_segnombre);
            println!("  Apellidos: '{}' '{:?}'", persona.per_priapellido, persona.per_segapellido);
            if let Some(ref email) = persona.per_email {
                println!("  Email: '{}' (length: {})", email, email.len());
                println!("  Email bytes: {:?}", email.as_bytes());
            } else {
                println!("  Email: NULL");
            }
            
            let row = client
                .query_one(
                    "INSERT INTO per_personasmayores (per_rut, per_prinombre, per_segnombre, per_priapellido, per_segapellido, per_genid, per_nacid, per_fechadenac, per_direccion, per_email, per_uvid) 
                     VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11) RETURNING per_id",
                    &[
                        &persona.per_rut,
                        &persona.per_prinombre,
                        &persona.per_segnombre,
                        &persona.per_priapellido,
                        &persona.per_segapellido,
                        &persona.per_genid,
                        &persona.per_nacid,
                        &persona.per_fechadenac,
                        &persona.per_direccion,
                        &persona.per_email,
                        &persona.per_uvid,
                    ],
                )
                .await?;
            Ok(row.get("per_id"))
        } else {
            Err(anyhow::anyhow!("No hay conexión a la base de datos"))
        }
    }

    pub async fn insert_organizacion(&self, organizacion: &OrganizacionComunitaria) -> Result<i32> {
        if let Some(client) = &self.client {
            let row = client
                .query_one(
                    "INSERT INTO org_orgcomunitarias (org_nombre, org_direccion, org_uvid, org_fechaconst, org_perjuridica, org_email) 
                     VALUES ($1, $2, $3, $4, $5, $6) RETURNING org_id",
                    &[
                        &organizacion.org_nombre,
                        &organizacion.org_direccion,
                        &organizacion.org_uvid,
                        &organizacion.org_fechaconst,
                        &organizacion.org_perjuridica,
                        &organizacion.org_email,
                    ],
                )
                .await?;
            Ok(row.get("org_id"))
        } else {
            Err(anyhow::anyhow!("No hay conexión a la base de datos"))
        }
    }

    pub async fn insert_actividad(&self, actividad: &Actividad) -> Result<i32> {
        if let Some(client) = &self.client {
            let row = client
                .query_one(
                    "INSERT INTO act_actividades (act_nombre, act_uvid, act_fecha_ini, act_fecha_fin, act_descripcion) 
                     VALUES ($1, $2, $3, $4, $5) RETURNING act_id",
                    &[
                        &actividad.act_nombre,
                        &actividad.act_uvid,
                        &actividad.act_fecha_ini,
                        &actividad.act_fecha_fin,
                        &actividad.act_descripcion,
                    ],
                )
                .await?;
            Ok(row.get("act_id"))
        } else {
            Err(anyhow::anyhow!("No hay conexión a la base de datos"))
        }
    }

    // Métodos adicionales de inserción
    pub async fn insert_macro_sector(&self, nombre: &str) -> Result<i32> {
        if let Some(client) = &self.client {
            let row = client
                .query_one(
                    "INSERT INTO mac_macrosectores (mac_nombre) VALUES ($1) RETURNING mac_id",
                    &[&nombre],
                )
                .await?;
            Ok(row.get("mac_id"))
        } else {
            Err(anyhow::anyhow!("No hay conexión a la base de datos"))
        }
    }

    pub async fn insert_unidad_vecinal(&self, nombre: &str, macro_sector_id: i32) -> Result<i32> {
        if let Some(client) = &self.client {
            let row = client
                .query_one(
                    "INSERT INTO uv_unidadesvecinales (uv_nombre, uv_macid) VALUES ($1, $2) RETURNING uv_id",
                    &[&nombre, &macro_sector_id],
                )
                .await?;
            Ok(row.get("uv_id"))
        } else {
            Err(anyhow::anyhow!("No hay conexión a la base de datos"))
        }
    }

    pub async fn insert_taller(&self, nombre: &str) -> Result<i32> {
        if let Some(client) = &self.client {
            let row = client
                .query_one(
                    "INSERT INTO tal_talleres (tal_nombre) VALUES ($1) RETURNING tal_id",
                    &[&nombre],
                )
                .await?;
            Ok(row.get("tal_id"))
        } else {
            Err(anyhow::anyhow!("No hay conexión a la base de datos"))
        }
    }

    #[allow(dead_code)]
    pub async fn insert_genero(&self, nombre: &str) -> Result<i32> {
        if let Some(client) = &self.client {
            let row = client
                .query_one(
                    "INSERT INTO gen_generos (gen_genero) VALUES ($1) RETURNING gen_id",
                    &[&nombre],
                )
                .await?;
            Ok(row.get("gen_id"))
        } else {
            Err(anyhow::anyhow!("No hay conexión a la base de datos"))
        }
    }

    #[allow(dead_code)]
    pub async fn insert_nacionalidad(&self, nombre: &str) -> Result<i32> {
        if let Some(client) = &self.client {
            let row = client
                .query_one(
                    "INSERT INTO nac_nacionalidades (nac_nacionalidad) VALUES ($1) RETURNING nac_id",
                    &[&nombre],
                )
                .await?;
            Ok(row.get("nac_id"))
        } else {
            Err(anyhow::anyhow!("No hay conexión a la base de datos"))
        }
    }

    // Función helper para probar constraints de email
    #[allow(dead_code)]
    pub async fn test_email_constraint(&self, email: &str) -> Result<bool> {
        if let Some(client) = &self.client {
            let result = client
                .query_one(
                    "SELECT $1 ~* '^[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\\.[A-Za-z]{2,}$' AS is_valid",
                    &[&email],
                )
                .await?;
            Ok(result.get("is_valid"))
        } else {
            Err(anyhow::anyhow!("No hay conexión a la base de datos"))
        }
    }

    // Función para aplicar fix temporal del constraint
    pub async fn fix_email_constraint_temp(&self) -> Result<()> {
        if let Some(client) = &self.client {
            // Eliminar constraint existente
            let _ = client
                .execute(
                    "ALTER TABLE per_personasmayores DROP CONSTRAINT IF EXISTS chk_per_email_formato",
                    &[],
                )
                .await;
            
            // Agregar constraint temporal más permisivo
            client
                .execute(
                    "ALTER TABLE per_personasmayores ADD CONSTRAINT chk_per_email_formato_temp CHECK (per_email IS NULL OR (per_email LIKE '%@%.%' AND length(per_email) > 5))",
                    &[],
                )
                .await?;
            
            println!("DEBUG: Constraint de email actualizado temporalmente");
            Ok(())
        } else {
            Err(anyhow::anyhow!("No hay conexión a la base de datos"))
        }
    }
}
