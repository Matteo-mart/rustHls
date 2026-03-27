use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct FFprobeOutput {
    pub streams: Vec<Stream>,
}

#[derive(Deserialize)]
pub struct Stream {
    // pub index: u32,
    pub codec_type: String,
    #[serde(default)]
    pub tags: HashMap<String, String>,
    // pub id: Option<String>,
    #[serde(default)]
    pub disposition: Disposition,
}

#[derive(Deserialize, Default)]
pub struct Disposition {
    #[serde(default)]
    pub descriptions: u8,
    #[serde(default)]
    pub default: u8,
    #[serde(default)]
    pub captions: u8,
}