mod app;
mod colors;
mod project;
mod yolo;

use std::path::PathBuf;

use egui::{FontData, FontDefinitions, FontFamily};

// todo
// delete, undo

mod coco {
    use serde_json::Value;

    use crate::project::Project;

    #[derive(serde::Serialize)]
    pub struct CocoJson {
        pub info: Value,
        pub licenses: Value,
        pub images: Vec<CocoImage>,
    }

    #[derive(serde::Serialize)]
    pub struct CocoImage {
        id: u32,
        width: u32,
        height: u32,
        file_name: String,
    }

    #[derive(serde::Serialize)]
    pub struct CocoAnnotation {
        id: u32,
        image_id: u32,
        category_id: u32,
        bbox: [u32; 4],
    }

    #[derive(serde::Serialize)]
    pub struct CocoCategory {
        id: u32,
        name: String,
    }

    // impl From<&Project> for CocoJson {
    //     fn from(project: &Project) -> Self {
    //         let images: Vec<_> = project.images.iter().enumerate().map(|(id, image)| {
    //             let image_data = image::image_dimensions(&image.file_path).unwrap();
    //             let (width, height) = image_data;
    //             let file_name = image.file_path.clone().to_string_lossy().to_string();
    //             CocoImage {
    //                 id: id as u32,
    //                 width,
    //                 height,
    //                 file_name,
    //             }
    //         }).collect();

    //         let annotations = project
    //             .images
    //             .iter()
    //             .zip(images.iter())
    //             .map(|i| i.segments.iter().enumerate().map(|s| (i, s)))
    //             .flatten()
    //             .map(|(i, s)| {
    //                 CocoAnnotation
    //             });
    //     }
    // }
}

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
