extern crate serde;
extern crate serde_json;

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Serializer as JsonSerializer;

pub type Dimension = f32;
pub type BoundingBox = Vec<Dimension>;
pub type FontFace = String;
pub type FontSize = String;
pub type FontFaces = Vec<FontFace>;
pub type FontSizes = Vec<FontSize>;
pub type Data = HashMap<FontFace, HashMap<FontSize, FontData>>;

#[derive(Debug, Serialize, Deserialize)]
pub struct FontData {
    pub boxes: Vec<BoundingBox>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FontConfig {
    pub faces: FontFaces,
    pub sizes: FontSizes,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharConfig {
    pub offset: String,
    pub range: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DataSet {
    pub error: Option<String>,
    pub config: DataSetConfig,
    pub data: Data,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DataSetConfig {
    pub font: FontConfig,
    pub char: CharConfig,
}
