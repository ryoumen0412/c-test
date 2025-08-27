-- Catálogo macro sectores
CREATE TABLE mac_macrosectores (
    mac_id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    mac_nombre VARCHAR(255) NOT NULL UNIQUE
);

-- Catálogo unidades vecinales (normalizado: cada UV pertenece a un macro sector)
CREATE TABLE uv_unidadesvecinales (
    uv_id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    uv_nombre VARCHAR(255) NOT NULL UNIQUE,
    uv_macid INT NOT NULL,
    CONSTRAINT fk_uv_mac FOREIGN KEY (uv_macid) REFERENCES mac_macrosectores (mac_id)
);

-- Catálogo géneros
CREATE TABLE gen_generos (
    gen_id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    gen_genero VARCHAR(255) NOT NULL UNIQUE
);

-- Catálogo nacionalidades
CREATE TABLE nac_nacionalidades (
    nac_id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    nac_nacionalidad VARCHAR(255) NOT NULL UNIQUE
);

-- Organizaciones comunitarias
CREATE TABLE org_orgcomunitarias (
    org_id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    org_nombre VARCHAR(255) NOT NULL UNIQUE,
    org_direccion VARCHAR(255) NOT NULL,
    org_uvid INT NOT NULL,
    org_fechaconst DATE NOT NULL,
    org_perjuridica VARCHAR(255) NOT NULL,
    org_email VARCHAR(255),
    CONSTRAINT fk_org_uv FOREIGN KEY (org_uvid) REFERENCES uv_unidadesvecinales (uv_id),
    -- Validación simple de email (no exhaustiva)
    CONSTRAINT chk_org_email_formato CHECK (org_email IS NULL OR org_email ~* '^[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\\.[A-Za-z]{2,}$')
);

-- Teléfonos organizaciones (multivalor normalizado)
CREATE TABLE org_telefonos (
    ot_id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    ot_orgid INT NOT NULL,
    ot_tipo VARCHAR(30) DEFAULT 'principal',
    ot_numero VARCHAR(20) NOT NULL,
    CONSTRAINT fk_ot_org FOREIGN KEY (ot_orgid) REFERENCES org_orgcomunitarias (org_id) ON DELETE CASCADE,
    CONSTRAINT uq_ot_org_tipo UNIQUE (ot_orgid, ot_tipo)
);

-- Centros comunitarios
CREATE TABLE cen_cencomunitarios (
    cen_id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    cen_nombre VARCHAR(255) NOT NULL UNIQUE,
    cen_direccion VARCHAR(255) NOT NULL,
    cen_uvid INT NOT NULL,
    CONSTRAINT fk_cen_uv FOREIGN KEY (cen_uvid) REFERENCES uv_unidadesvecinales (uv_id)
);

-- Personas mayores
CREATE TABLE per_personasmayores (
    per_id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    per_rut VARCHAR(12) NOT NULL UNIQUE,
    per_prinombre VARCHAR(255) NOT NULL,
    per_segnombre VARCHAR(255),
    per_priapellido VARCHAR(255) NOT NULL,
    per_segapellido VARCHAR(255),
    per_genid INT NOT NULL,
    per_nacid INT NOT NULL,
    per_fechadenac DATE NOT NULL,
    per_direccion VARCHAR(255) NOT NULL,
    per_email VARCHAR(255),
    per_uvid INT NOT NULL,
    CONSTRAINT fk_per_gen FOREIGN KEY (per_genid) REFERENCES gen_generos (gen_id),
    CONSTRAINT fk_per_nac FOREIGN KEY (per_nacid) REFERENCES nac_nacionalidades (nac_id),
    CONSTRAINT fk_per_uv FOREIGN KEY (per_uvid) REFERENCES uv_unidadesvecinales (uv_id),
    -- Formato básico de RUT chileno: 7-8 dígitos, guión y dígito verificador (0-9 o K)
    CONSTRAINT chk_per_rut_formato CHECK (per_rut ~ '^[0-9]{7,8}-[0-9Kk]$'),
    -- Email opcional; si no es NULL debe cumplir patrón simple
    CONSTRAINT chk_per_email_formato CHECK (per_email IS NULL OR per_email ~* '^[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\\.[A-Za-z]{2,}$')
);

-- Teléfonos personas mayores (multivalor normalizado)
CREATE TABLE per_telefonos (
    pt_id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    pt_perid INT NOT NULL,
    pt_tipo VARCHAR(30) DEFAULT 'principal',
    pt_numero VARCHAR(20) NOT NULL,
    CONSTRAINT fk_pt_per FOREIGN KEY (pt_perid) REFERENCES per_personasmayores (per_id) ON DELETE CASCADE,
    -- Acepta números con + inicial y espacios / guiones internos
    CONSTRAINT chk_pt_numero_formato CHECK (pt_numero ~ '^[0-9+][0-9 -]{5,19}$'),
    -- Evita duplicados lógicos de un tipo por persona
    CONSTRAINT uq_pt_per_tipo UNIQUE (pt_perid, pt_tipo)
);

-- Talleres
CREATE TABLE tal_talleres (
    tal_id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    tal_nombre VARCHAR(255) NOT NULL UNIQUE
);

-- Actividades
CREATE TABLE act_actividades (
    act_id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    act_nombre VARCHAR(255) NOT NULL,
    act_uvid INT NOT NULL,
    act_fecha_ini DATE NOT NULL,
    act_fecha_fin DATE,
    act_descripcion TEXT,
    CONSTRAINT fk_act_uv FOREIGN KEY (act_uvid) REFERENCES uv_unidadesvecinales (uv_id),
    CONSTRAINT uq_act_nombre_fecha_uv UNIQUE (act_nombre, act_fecha_ini, act_uvid),
    CONSTRAINT chk_act_fechas CHECK (act_fecha_fin IS NULL OR act_fecha_fin >= act_fecha_ini)
);

-- Viajes
CREATE TABLE via_viajes (
    via_id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    via_nombre VARCHAR(255) NOT NULL,
    via_destino VARCHAR(255) NOT NULL,
    via_fecha_salida DATE NOT NULL,
    via_fecha_regreso DATE,
    via_uvid INT NOT NULL,
    -- Clave natural ampliada (mac derivable por uv)
    CONSTRAINT uq_via_nombre_salida_uv UNIQUE (via_nombre, via_fecha_salida, via_uvid),
    CONSTRAINT fk_via_uv FOREIGN KEY (via_uvid) REFERENCES uv_unidadesvecinales (uv_id),
    CONSTRAINT chk_via_fechas CHECK (via_fecha_regreso IS NULL OR via_fecha_regreso >= via_fecha_salida)
);

-- Beneficios catálogo
CREATE TABLE ben_beneficios (
    ben_id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    ben_codigo VARCHAR(50) NOT NULL UNIQUE,
    ben_descripcion VARCHAR(255) NOT NULL
);

-- Beneficios asignados
CREATE TABLE per_beneficios (
    pb_perid INT NOT NULL,
    pb_benid INT NOT NULL,
    pb_fecha_asignacion DATE NOT NULL DEFAULT CURRENT_DATE,
    PRIMARY KEY (pb_perid, pb_benid),
    CONSTRAINT fk_pb_per FOREIGN KEY (pb_perid) REFERENCES per_personasmayores (per_id) ON DELETE CASCADE,
    CONSTRAINT fk_pb_ben FOREIGN KEY (pb_benid) REFERENCES ben_beneficios (ben_id) ON DELETE CASCADE
);

-- Relación personas-organizaciones
CREATE TABLE per_org (
    po_perid INT NOT NULL,
    po_orgid INT NOT NULL,
    PRIMARY KEY (po_perid, po_orgid),
    CONSTRAINT fk_po_per FOREIGN KEY (po_perid) REFERENCES per_personasmayores (per_id) ON DELETE CASCADE,
    CONSTRAINT fk_po_org FOREIGN KEY (po_orgid) REFERENCES org_orgcomunitarias (org_id) ON DELETE CASCADE
);

-- Solicitudes org-centro
CREATE TABLE soli_cen (
    soli_orgid INT NOT NULL,
    soli_cenid INT NOT NULL,
    soli_fecha DATE NOT NULL DEFAULT CURRENT_DATE,
    -- Permitimos historial de solicitudes distinguiendo por fecha
    PRIMARY KEY (soli_orgid, soli_cenid, soli_fecha),
    CONSTRAINT fk_soli_org FOREIGN KEY (soli_orgid) REFERENCES org_orgcomunitarias (org_id) ON DELETE CASCADE,
    CONSTRAINT fk_soli_cen FOREIGN KEY (soli_cenid) REFERENCES cen_cencomunitarios (cen_id) ON DELETE CASCADE
);

-- Registro de mantenimientos
CREATE TABLE reg_registromantenimientos (
    reg_id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    reg_cenid INT NOT NULL,
    reg_servicio VARCHAR(255) NOT NULL,
    reg_fecha DATE NOT NULL,
    reg_detalle VARCHAR(255),
    CONSTRAINT fk_reg_cen FOREIGN KEY (reg_cenid) REFERENCES cen_cencomunitarios (cen_id) ON DELETE CASCADE
);
-- Evita duplicados lógicos de mismo servicio en la misma fecha para un centro
ALTER TABLE reg_registromantenimientos
    ADD CONSTRAINT uq_reg_cen_serv_fecha UNIQUE (reg_cenid, reg_servicio, reg_fecha);

-- Asistencias a talleres
CREATE TABLE asis_talleres (
    asistal_id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    asis_perid INT NOT NULL,
    asis_talid INT NOT NULL,
    asis_fecha TIMESTAMP NOT NULL DEFAULT NOW(),
    CONSTRAINT fk_asistal_per FOREIGN KEY (asis_perid) REFERENCES per_personasmayores (per_id) ON DELETE CASCADE,
    CONSTRAINT fk_asistal_tal FOREIGN KEY (asis_talid) REFERENCES tal_talleres (tal_id) ON DELETE CASCADE,
    -- Una asistencia lógica por persona/taller (independiente de timestamp)
    CONSTRAINT uq_asistal UNIQUE (asis_perid, asis_talid)
);

-- Asistencias a actividades
CREATE TABLE asis_actividades (
    asisact_id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    asis_perid INT NOT NULL,
    asis_actid INT NOT NULL,
    asis_fecha TIMESTAMP NOT NULL DEFAULT NOW(),
    CONSTRAINT fk_asisact_per FOREIGN KEY (asis_perid) REFERENCES per_personasmayores (per_id) ON DELETE CASCADE,
    CONSTRAINT fk_asisact_act FOREIGN KEY (asis_actid) REFERENCES act_actividades (act_id) ON DELETE CASCADE,
    CONSTRAINT uq_asisact UNIQUE (asis_perid, asis_actid)
);

-- Asistencias a viajes
CREATE TABLE asis_viajes (
    asisvia_id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    asis_perid INT NOT NULL,
    asis_viaid INT NOT NULL,
    asis_fecha TIMESTAMP NOT NULL DEFAULT NOW(),
    CONSTRAINT fk_asisvia_per FOREIGN KEY (asis_perid) REFERENCES per_personasmayores (per_id) ON DELETE CASCADE,
    CONSTRAINT fk_asisvia_via FOREIGN KEY (asis_viaid) REFERENCES via_viajes (via_id) ON DELETE CASCADE,
    CONSTRAINT uq_asisvia UNIQUE (asis_perid, asis_viaid)
);

-- Índices apoyo
-- Índices por UV (MAC es derivable desde UV)
CREATE INDEX idx_act_fecha ON act_actividades (act_fecha_ini);
-- Índice compuesto por UV y fecha (MAC derivable)
CREATE INDEX idx_act_uv_fecha ON act_actividades (act_uvid, act_fecha_ini);
CREATE INDEX idx_via_salida ON via_viajes (via_fecha_salida);
CREATE INDEX idx_asistal_per_fecha ON asis_talleres (asis_perid, asis_fecha);
CREATE INDEX idx_asisact_per_fecha ON asis_actividades (asis_perid, asis_fecha);
CREATE INDEX idx_asisvia_per_fecha ON asis_viajes (asis_perid, asis_fecha);
-- Índices adicionales para consultas por evento
CREATE INDEX idx_asistal_tal_fecha ON asis_talleres (asis_talid, asis_fecha);
CREATE INDEX idx_asisact_act_fecha ON asis_actividades (asis_actid, asis_fecha);
CREATE INDEX idx_asisvia_via_fecha ON asis_viajes (asis_viaid, asis_fecha);
-- Índices FK para tablas de teléfonos
CREATE INDEX idx_pt_per ON per_telefonos (pt_perid);
CREATE INDEX idx_ot_org ON org_telefonos (ot_orgid);
-- Índice compuesto frecuente por macro sector y unidad vecinal
-- Índice por UV (antes MAC+UV, ahora redundancia eliminada)
CREATE INDEX idx_per_uvid ON per_personasmayores (per_uvid);

-- Índice soporte para joins UV -> MAC
CREATE INDEX idx_uv_mac ON uv_unidadesvecinales (uv_macid);

-- Fin
