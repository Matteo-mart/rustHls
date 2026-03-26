use serde::Deserialize;

pub struct Stream {
    pub codec_type: String,
    pub lang: Option<String>,
    pub is_description: bool,
}

#[derive(Deserialize)]
pub struct FFprobeOutput {
    pub streams: Vec<FFprobeStream>,
}

#[derive(Deserialize)]
pub struct FFprobeStream {
    pub codec_type: String,
    #[serde(default)]
    pub disposition: Disposition,
    #[serde(default)]
    pub tags: std::collections::HashMap<String, String>,
}

#[derive(Deserialize, Default)]
pub struct Disposition {
    pub descriptions: Option<u8>,
}