use egui::ahash::HashMap;
use serde::Deserialize;
use std::path::PathBuf;

#[allow(dead_code)]
#[derive(Debug, Deserialize, Clone)]
pub struct YoloDataConfig {
    pub train: PathBuf,
    pub val: PathBuf,
    pub test: Option<PathBuf>,
    pub names: HashMap<u32, String>,
}
