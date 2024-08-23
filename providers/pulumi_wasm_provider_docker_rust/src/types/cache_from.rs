#[derive(serde::Serialize)]
pub struct CacheFrom {
    #[serde(rename = "images")]
    pub r#images: Box<Option<Vec<String>>>,
}
