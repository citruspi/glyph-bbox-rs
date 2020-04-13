extern crate serde;
extern crate serde_json;

use std::collections::HashMap;
use std::fs::File;
use std::io::{Error, Write};

use serde::{Deserialize, Serialize};
use serde_json::Serializer as JsonSerializer;

pub type Index = i32;
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
    pub offset: Index,
    pub range: Index,
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

#[derive(Debug, Deserialize)]
pub struct WriteOptions {
    pub filename: String,
    pub format: Format,
}

#[derive(Debug, Deserialize)]
pub enum Format {
    JSON,
}

impl DataSet {
    pub fn write(&self, opts: WriteOptions) -> Result<(), Error> {
        let mut buf = Vec::new();

        match opts.format {
            Format::JSON => self.serialize(&mut JsonSerializer::new(&mut buf)).unwrap(),
        }

        let mut file = File::create(opts.filename).unwrap();

        file.write_all(buf.as_ref())
    }
}
