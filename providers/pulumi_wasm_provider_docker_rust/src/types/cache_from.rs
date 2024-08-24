#[derive(serde::Serialize)]
pub struct CacheFrom {
    /// Specifies cached images
    #[serde(rename = "images")]
    pub r#images: Box<Option<Vec<String>>>,
}
