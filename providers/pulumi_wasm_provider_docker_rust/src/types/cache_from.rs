#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug)]
#[builder(finish_fn = build_struct)]
pub struct CacheFrom {
    /// Specifies cached images
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "images")]
    pub r#images: Box<Option<Vec<String>>>,
}
