#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct CacheFrom {
    /// Specifies cached images
    #[serde(rename = "images")]
    pub r#images: Box<Option<Vec<String>>>,
}
