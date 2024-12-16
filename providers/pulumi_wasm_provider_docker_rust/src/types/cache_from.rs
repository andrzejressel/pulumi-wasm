//! Contains a list of images to reference when building using a cache

#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
pub struct CacheFrom {
    /// Specifies cached images
    #[builder(into, default)]
    #[serde(rename = "images")]
    pub r#images: Box<Option<Vec<String>>>,
}
