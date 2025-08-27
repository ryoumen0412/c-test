use eframe::egui;

pub struct AppleMusicStyle;

impl AppleMusicStyle {
    // Colores principales - inspirados en Apple Music pero en azul
    pub const PRIMARY_BLUE: egui::Color32 = egui::Color32::from_rgb(0, 122, 255);        // Azul principal de Apple
    pub const SECONDARY_BLUE: egui::Color32 = egui::Color32::from_rgb(30, 144, 255);     // Azul más claro
    pub const DARK_BLUE: egui::Color32 = egui::Color32::from_rgb(0, 100, 210);           // Azul oscuro
    pub const BACKGROUND_DARK: egui::Color32 = egui::Color32::from_rgb(25, 25, 28);      // Fondo oscuro
    pub const BACKGROUND_LIGHT: egui::Color32 = egui::Color32::from_rgb(40, 40, 45);     // Fondo claro
    pub const SIDEBAR_BG: egui::Color32 = egui::Color32::from_rgb(30, 30, 35);           // Fondo sidebar
    pub const TEXT_PRIMARY: egui::Color32 = egui::Color32::WHITE;                        // Texto principal
    pub const TEXT_SECONDARY: egui::Color32 = egui::Color32::from_rgb(160, 160, 160);    // Texto secundario
    pub const ACCENT_BLUE: egui::Color32 = egui::Color32::from_rgb(10, 132, 255);        // Azul de acento
    pub const HOVER_BLUE: egui::Color32 = egui::Color32::from_rgb(20, 140, 255);         // Azul hover
    pub const CARD_BG: egui::Color32 = egui::Color32::from_rgb(35, 35, 40);              // Fondo de tarjetas

    pub fn apply_style(ctx: &egui::Context) {
        let mut style = (*ctx.style()).clone();
        
        // Configurar colores globales
        style.visuals.dark_mode = true;
        style.visuals.panel_fill = Self::BACKGROUND_DARK;
        style.visuals.window_fill = Self::BACKGROUND_LIGHT;
        style.visuals.extreme_bg_color = Self::SIDEBAR_BG;
        
        // Botones
        style.visuals.widgets.inactive.bg_fill = Self::CARD_BG;
        style.visuals.widgets.inactive.fg_stroke = egui::Stroke::new(1.0, Self::TEXT_SECONDARY);
        style.visuals.widgets.hovered.bg_fill = Self::HOVER_BLUE;
        style.visuals.widgets.hovered.fg_stroke = egui::Stroke::new(1.0, Self::TEXT_PRIMARY);
        style.visuals.widgets.active.bg_fill = Self::PRIMARY_BLUE;
        style.visuals.widgets.active.fg_stroke = egui::Stroke::new(1.0, Self::TEXT_PRIMARY);
        
        // Bordes redondeados estilo Apple
        style.visuals.widgets.inactive.rounding = egui::Rounding::same(8.0);
        style.visuals.widgets.hovered.rounding = egui::Rounding::same(8.0);
        style.visuals.widgets.active.rounding = egui::Rounding::same(8.0);
        
        // Espaciado
        style.spacing.button_padding = egui::vec2(16.0, 12.0);
        style.spacing.item_spacing = egui::vec2(12.0, 8.0);
        style.spacing.window_margin = egui::Margin::same(0.0);
        style.spacing.menu_margin = egui::Margin::same(8.0);
        
        // Scrollbars estilo Apple
        style.visuals.widgets.noninteractive.bg_fill = Self::SIDEBAR_BG;
        
        ctx.set_style(style);
    }

    // Estilo para botón principal (estilo Apple Music)
    pub fn primary_button() -> egui::Button<'static> {
        egui::Button::new("Button")
            .fill(Self::PRIMARY_BLUE)
            .rounding(egui::Rounding::same(20.0))
            .stroke(egui::Stroke::NONE)
            .min_size(egui::vec2(100.0, 40.0))
    }

    // Estilo para botón secundario
    pub fn secondary_button() -> egui::Button<'static> {
        egui::Button::new("Button")
            .fill(Self::CARD_BG)
            .rounding(egui::Rounding::same(8.0))
            .stroke(egui::Stroke::new(1.0, Self::SECONDARY_BLUE))
            .min_size(egui::vec2(80.0, 32.0))
    }

    // Frame estilo tarjeta Apple Music
    pub fn card_frame() -> egui::Frame {
        egui::Frame::none()
            .fill(Self::CARD_BG)
            .rounding(egui::Rounding::same(12.0))
            .inner_margin(egui::Margin::same(16.0))
            .shadow(egui::Shadow {
                offset: egui::vec2(0.0, 2.0),
                blur: 8.0,
                spread: 0.0,
                color: egui::Color32::from_black_alpha(50),
            })
    }

    // Frame para sidebar estilo Apple Music
    pub fn sidebar_frame() -> egui::Frame {
        egui::Frame::none()
            .fill(Self::SIDEBAR_BG)
            .inner_margin(egui::Margin::same(12.0))
    }

    // Header estilo Apple Music
    pub fn header_text(text: &str) -> egui::RichText {
        egui::RichText::new(text)
            .size(24.0)
            .strong()
            .color(Self::TEXT_PRIMARY)
    }

    // Texto secundario
    pub fn secondary_text(text: &str) -> egui::RichText {
        egui::RichText::new(text)
            .size(14.0)
            .color(Self::TEXT_SECONDARY)
    }

    // Botón de navegación del sidebar
    pub fn nav_button(text: &str, is_active: bool) -> egui::Button<'static> {
        let bg_color = if is_active { Self::PRIMARY_BLUE } else { egui::Color32::TRANSPARENT };
        let text_color = if is_active { Self::TEXT_PRIMARY } else { Self::TEXT_SECONDARY };
        
        egui::Button::new(egui::RichText::new(text).color(text_color))
            .fill(bg_color)
            .rounding(egui::Rounding::same(8.0))
            .stroke(egui::Stroke::NONE)
            .min_size(egui::vec2(160.0, 36.0))
    }
}
