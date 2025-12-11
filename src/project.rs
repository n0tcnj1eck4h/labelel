use crate::app::SegmentDrag;
use crate::colors::COLORS;
use crate::yolo::YoloDataConfig;

use egui::ahash::HashMap;
use serde_yaml::Number;
use serde_yaml::Value;
use std::fs;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Write;
use std::path::PathBuf;

#[derive(Default)]
pub struct Label {
    pub name: String,
    pub color: egui::Color32,
}

pub struct Segment {
    pub center: egui::Pos2,
    pub size: egui::Vec2,
    pub label_id: u32,
}

pub struct Image {
    pub file_path: PathBuf,
    pub file_name: String,
    pub labels_file_path: PathBuf,
    pub segments: Vec<Segment>,
    pub uri: String,
}

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum Tool {
    Stamp,
    Drag,
    Edit,
}

pub struct Project {
    pub original_yaml: Value,
    pub labels: HashMap<u32, Label>,
    pub images: Vec<Image>,
    pub image_index: usize,
    pub label_id: Option<u32>,
    pub rect_size: egui::Vec2,
    pub tool: Tool,
    pub drag_start_pos: Option<egui::Pos2>,
    pub edit_drag: Option<SegmentDrag>,
    pub add_label_modal: Option<(u32, String)>,
    pub yaml_file_path: PathBuf,
}

impl Project {
    pub fn load(yaml_file_path: PathBuf) -> anyhow::Result<Project> {
        let contents = fs::read_to_string(&yaml_file_path)?;
        // let yaml: YoloDataConfig = serde_yaml::from_str(&contents)?;
        let yaml: Value = serde_yaml::from_str(&contents)?;
        dbg!(&yaml);
        let yolo: YoloDataConfig = serde_yaml::from_value(yaml.clone())?;

        let base = yaml_file_path.parent().unwrap();
        dbg!(&base);
        let train_dir_path = base.join(&yolo.train);
        dbg!(&train_dir_path);
        let mut images = vec![];
        for file in fs::read_dir(train_dir_path)? {
            let file = file?;
            if file.file_type()?.is_file()
                && (file.path().to_string_lossy().ends_with(".png")
                    || file.path().to_string_lossy().ends_with(".jpg"))
            {
                let mut labels_file_path =
                    PathBuf::from(file.path().to_string_lossy().replace("images", "labels"));

                labels_file_path.set_extension("txt");

                let mut segments = vec![];
                if let Ok(file) = File::open(&labels_file_path) {
                    let reader = BufReader::new(file);
                    for line in reader.lines() {
                        let line = line?;
                        let parts: Vec<_> = line.split(' ').collect();
                        segments.push(Segment {
                            center: egui::Pos2 {
                                x: parts[1].parse()?,
                                y: parts[2].parse()?,
                            },
                            size: egui::Vec2 {
                                x: parts[3].parse()?,
                                y: parts[4].parse()?,
                            },
                            label_id: parts[0].parse()?,
                        });
                    }
                };

                let file_name = file
                    .path()
                    .file_name()
                    .unwrap()
                    .to_string_lossy()
                    .to_string();
                let mut uri = "file://".to_string();
                uri.push_str(&file.path().to_string_lossy());

                images.push(Image {
                    uri,
                    file_name,
                    labels_file_path,
                    file_path: file.path(),
                    segments,
                });
            }
        }

        let mut labels = HashMap::default();
        for (&index, name) in &yolo.names {
            labels.insert(
                index,
                Label {
                    name: name.clone(),
                    color: COLORS[index as usize % COLORS.len()],
                },
            );
        }

        images.sort_by(|a, b| a.file_path.file_name().cmp(&b.file_path.file_name()));

        Ok(Project {
            yaml_file_path,
            original_yaml: yaml,
            images,
            labels,
            image_index: 0,
            rect_size: egui::Vec2::splat(64.0),
            label_id: None,
            tool: Tool::Stamp,
            drag_start_pos: None,
            edit_drag: None,
            add_label_modal: None,
        })
    }

    pub fn save(&mut self) -> anyhow::Result<()> {
        let nc = self.labels.len();
        let v = Value::Mapping(
            self.labels
                .iter()
                .map(|(id, label)| (Value::Number(Number::from(*id)), label.name.clone().into()))
                .collect(),
        );
        self.original_yaml["names"] = v;
        self.original_yaml["nc"] = nc.into();
        let mut file = File::create(&self.yaml_file_path)?;
        write!(
            &mut file,
            "{}",
            &serde_yaml::to_string(&self.original_yaml)?
        )?;

        for image in &self.images {
            if image.segments.is_empty() {
                continue;
            }
            let path = &image.labels_file_path;
            let mut file = File::create(path)?;
            for segment in &image.segments {
                writeln!(
                    &mut file,
                    "{} {} {} {} {}",
                    segment.label_id,
                    segment.center.x.clamp(0.0, 1.0),
                    segment.center.y.clamp(0.0, 1.0),
                    segment.size.x.clamp(0.0, 1.0),
                    segment.size.y.clamp(0.0, 1.0)
                )?;
            }
            println!("Saved {:?}", path);
        }

        Ok(())
    }

    pub fn advance(&mut self) {
        self.image_index = self
            .images
            .len()
            .saturating_sub(1)
            .min(self.image_index + 1);
    }

    pub fn back(&mut self) {
        self.image_index = self.image_index.saturating_sub(1);
    }
}
