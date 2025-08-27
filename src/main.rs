use eframe::egui;

mod database;
mod models;
mod ui;
mod utils;

use ui::app::App;

#[tokio::main]
async fn main() -> Result<(), eframe::Error> {
    env_logger::init();
    
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_min_inner_size([800.0, 600.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Gestor Base de Datos Comunitaria",
        options,
        Box::new(|cc| {
            // Configurar el tema
            cc.egui_ctx.set_visuals(egui::Visuals::dark());
            
            Ok(Box::new(App::new(cc)))
        }),
    )
}
