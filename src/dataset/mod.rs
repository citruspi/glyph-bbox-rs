use std::collections::HashMap;

pub type Dimension = f32;
pub type BoundingBox = Vec<Dimension>;
pub type FontFace = String;
pub type FontSize = String;
pub type FontFaces = Vec<FontFace>;
pub type FontSizes = Vec<FontSize>;
pub type Data = HashMap<FontFace, HashMap<FontSize, FontData>>;

pub struct FontData {
    pub boxes: Vec<BoundingBox>,
}

pub struct FontConfig {
    pub faces: FontFaces,
    pub sizes: FontSizes,
}

pub struct CharConfig {
    pub offset: String,
    pub range: String,
}

pub struct DataSet {
    pub error: Option<String>,
    pub config: DataSetConfig,
    pub data: Data,
}

pub struct DataSetConfig {
    pub font: FontConfig,
    pub char: CharConfig,
}
