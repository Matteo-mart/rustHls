use std::collections::HashMap;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ConvertToHlsOutput {
    pub playlist_file: String,
}

#[derive(Deserialize)]
pub struct FfprobeOutput {
    pub streams: Vec<Stream>,
}

#[derive(Deserialize)]
pub struct Stream {
    pub codec_type: String,
    #[serde(default)]
    pub tags: HashMap<String, String>,
    #[serde(default)]
    pub disposition_descriptions: i32,
    // #[serde(default)]
    // pub disposition_default: i32,
    // #[serde(default)]
    // pub disposition_captions: i32,
}

#[derive(Debug, Clone)]
pub struct Variant {
    pub bandwidth: u64,
    pub resolution: String,
    pub codecs: String,
    pub uri: String,
}