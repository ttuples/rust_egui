#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod app;

fn main() -> eframe::Result {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])
            .with_min_inner_size([400.0, 300.0]),
            // .with_icon(
            //     eframe::icon_data::from_png_bytes(&include_bytes!("../assets/icon-256.png")[..])
            //         .expect("Failed to load icon"),
            // ),
        ..Default::default()
    };
    eframe::run_native(
        "egui template",
        native_options,
        Box::new(|cc| Ok(Box::new(app::App::new(cc)))),
    )
}
