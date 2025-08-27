# Gestor Base de Datos Comunitaria

Una aplicación de escritorio desarrollada en Rust con egui para gestionar bases de datos comunitarias de forma eficiente y escalable.

## Características

- **Autenticación Segura**: Login con credenciales de base de datos PostgreSQL
- **Dashboard Interactivo**: Estadísticas en tiempo real y visualización de datos
- **Consultas Avanzadas**: Sistema de filtros potente para búsquedas precisas
- **Inserción de Datos**: Formularios intuitivos para todos los tipos de entidades
- **Gestión Territorial**: Manejo de macrosectores y unidades vecinales
- **Registro de Personas**: Gestión completa de personas mayores
- **Organizaciones**: Administración de organizaciones comunitarias
- **Actividades**: Seguimiento de eventos y actividades
- **Viajes**: Gestión de viajes comunitarios

## Tecnologías

- **Lenguaje**: Rust
- **Framework UI**: egui
- **Base de Datos**: PostgreSQL
- **Driver DB**: tokio-postgres
- **Runtime Async**: Tokio
- **Serialización**: Serde
- **Manejo de Fechas**: chrono

## Prerrequisitos

- Rust (versión 1.70 o superior)
- PostgreSQL (versión 12 o superior)
- Cargo (incluido con Rust)

## Instalación

1. **Clonar el repositorio**:

```bash
git clone <repository-url>
cd c-test
```

2. **Instalar dependencias**:

```bash
cargo build
```

3. **Configurar la base de datos**:

   - Crear una base de datos PostgreSQL
   - Ejecutar el script SQL `1query.sql` para crear las tablas

4. **Ejecutar la aplicación**:

```bash
cargo run
```

## Estructura de la Base de Datos (query de creación de entidades en fuente.)

La aplicación gestiona las siguientes entidades:

### Tablas Principales

- `mac_macrosectores`: Sectores geográficos principales
- `uv_unidadesvecinales`: Unidades vecinales por macrosector
- `per_personasmayores`: Registro de personas mayores
- `org_orgcomunitarias`: Organizaciones comunitarias
- `act_actividades`: Actividades y eventos
- `via_viajes`: Viajes comunitarios
- `tal_talleres`: Talleres disponibles
- `ben_beneficios`: Catálogo de beneficios

### Tablas de Catálogo

- `gen_generos`: Géneros
- `nac_nacionalidades`: Nacionalidades

### Tablas de Relación

- `per_beneficios`: Beneficios asignados a personas
- `per_org`: Relación personas-organizaciones
- `asis_talleres`: Asistencias a talleres
- `asis_actividades`: Asistencias a actividades
- `asis_viajes`: Asistencias a viajes

## Uso de la Aplicación

### 1. Login

- Ingrese la IP, puerto, usuario y contraseña de PostgreSQL
- La aplicación validará las credenciales y establecerá la conexión

### 2. Dashboard

- Visualice estadísticas generales del sistema
- Vea distribución de datos por macrosector
- Monitoree actividad del mes actual

### 3. Consultas

- Seleccione el tipo de entidad a consultar
- Aplique filtros específicos
- Visualice resultados en formato tabla

### 4. Inserciones

- Seleccione el tipo de registro a crear
- Complete los formularios correspondientes
- Guarde la información en la base de datos

## Estructura del Proyecto

```markdown
src/
├── main.rs # Punto de entrada
├── models.rs # Modelos de datos
├── database.rs # Conexión y operaciones DB
├── utils.rs # Utilidades y helpers
└── ui/
├── mod.rs # Módulo UI
├── app.rs # Aplicación principal
├── login.rs # Vista de login
├── dashboard.rs # Vista del dashboard
├── sidebar.rs # Menú lateral
├── queries.rs # Vista de consultas
├── insertions.rs # Vista de inserciones
├── about.rs # Vista de información
└── components.rs # Componentes reutilizables
```

## Configuración

### Variables de Entorno (Opcional)

```bash
DATABASE_URL=postgresql://usuario:password@localhost:5432/comunidad
RUST_LOG=info
```

### Configuración de Base de Datos

Por defecto, la aplicación intentará conectarse a:

- Host: localhost
- Puerto: 5432
- Usuario: postgres
- Base de datos: comunidad

## Compilación para Producción

```bash
# Compilación optimizada
cargo build --release

# El ejecutable estará en target/release/
```

## Licencia

Este proyecto está bajo la Licencia MIT - vea el archivo [LICENSE](LICENSE) para detalles.

## Soporte

Para reportar bugs o solicitar features, por favor contactar a Carlos Cortés.

## Contacto

- Proyecto: Gestor Base de Datos Dirección de Personas Mayores
- Versión: 1.0.0
- Año: 2025
