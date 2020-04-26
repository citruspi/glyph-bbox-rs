#[macro_use]
extern crate serde_derive;

pub mod dataset {
    extern crate serde;
    extern crate serde_json;

    use std::collections::HashMap;
    use std::fs::{read_to_string, File};
    use std::io::{Error, Write};

    use serde::Serialize;
    use serde_json::Serializer as JsonSerializer;

    pub type Index = i32;
    pub type Dimension = f32;
    pub type BoundingBox = Vec<Dimension>;
    pub type FontFace = String;
    pub type FontSize = String;
    pub type FontFaces = Vec<FontFace>;
    pub type FontSizes = Vec<FontSize>;
    pub type Data = HashMap<FontFace, HashMap<FontSize, FontData>>;

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct FontData {
        pub boxes: Vec<BoundingBox>,
        pub signals: Signals,
    }

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct Signals {
        mean: BoundingBox,
    }

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct FontConfig {
        pub faces: FontFaces,
        pub sizes: FontSizes,
    }

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct CharConfig {
        pub offset: Index,
        pub range: Index,
    }

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct DataSet {
        pub error: Option<String>,
        pub config: DataSetConfig,
        pub data: Data,
    }

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct DataSetConfig {
        pub font: FontConfig,
        pub char: CharConfig,
        pub signals: CharConfig,
    }

    #[derive(Debug, Deserialize)]
    pub struct WriteOptions {
        pub filename: String,
        pub format: Format,
    }

    pub type ReadOptions = WriteOptions;

    #[derive(Debug, Deserialize)]
    pub enum Format {
        JSON,
    }

    pub struct BoundingBoxRenderOptions {
        pub face: FontFace,
        pub size: FontSize,
    }

    impl DataSet {
        pub fn from_file(opts: ReadOptions) -> DataSet {
            match opts.format {
                Format::JSON => {
                    let json_file_str = read_to_string(opts.filename).expect("file not found");

                    DataSet::from_json_string(&json_file_str)
                }
            }
        }

        pub fn from_json_string(s: &String) -> DataSet {
            serde_json::from_str(&s).expect("error while reading json")
        }

        pub fn write(&self, opts: WriteOptions) -> Result<(), Error> {
            let mut buf = Vec::new();

            match opts.format {
                Format::JSON => self.serialize(&mut JsonSerializer::new(&mut buf)).unwrap(),
            }

            let mut file = File::create(opts.filename).unwrap();

            file.write_all(buf.as_ref())
        }

        pub fn bounding_box(&self, s: &str, opts: BoundingBoxRenderOptions) -> Option<BoundingBox> {
            if !&self.data.contains_key(opts.face.as_str())
                || !&self.data[opts.face.as_str()].contains_key(opts.size.as_str())
            {
                ()
            }

            let mut width: Dimension = 0.0;
            let mut height: Dimension = 0.0;

            let mut buf = [0; 2];

            for c in s.chars() {
                let char_box = match &self.data[opts.face.as_str()][opts.size.as_str()]
                    .boxes
                    .get(c.encode_utf16(&mut buf)[0] as usize)
                {
                    Some(val) => val.to_vec(),
                    None => self.data[opts.face.as_str()][opts.size.as_str()]
                        .signals
                        .mean
                        .to_vec(),
                };

                width += char_box[0];

                if char_box[1] > height {
                    height = char_box[1];
                }
            }

            Some(vec![width, height])
        }
    }
}
