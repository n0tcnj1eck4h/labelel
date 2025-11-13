mod app;
mod colors;
mod project;
mod yolo;

use std::path::PathBuf;

use egui::{FontData, FontDefinitions, FontFamily};

// todo
// delete, undo

fn main() {
    let file = std::env::args_os().nth(1);
    eframe::run_native(
        "Labelel",
        eframe::NativeOptions {
            vsync: false,
            ..Default::default()
        },
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            let mut app = Box::<app::App>::default();
            if let Some(file) = file {
                app.open_project(PathBuf::from(&file));
            }
            let mut fonts = FontDefinitions::default();

            fonts.font_data.insert(
                "awesome".to_owned(),
                std::sync::Arc::new(FontData::from_static(include_bytes!(
                    "../Font Awesome 7 Free-Solid-900.otf"
                ))),
            );

            fonts
                .families
                .get_mut(&FontFamily::Proportional)
                .unwrap()
                .push("awesome".to_owned());

            fonts
                .families
                .get_mut(&FontFamily::Monospace)
                .unwrap()
                .push("awesome".to_owned());

            cc.egui_ctx.set_fonts(fonts);
            Ok(app)
        }),
    )
    .unwrap();
}
