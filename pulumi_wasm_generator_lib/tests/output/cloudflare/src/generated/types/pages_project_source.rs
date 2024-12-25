#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
pub struct PagesProjectSource {
    /// Configuration for the source of the Cloudflare Pages project.
    #[builder(into, default)]
    #[serde(rename = "config")]
    pub r#config: Box<Option<super::types::PagesProjectSourceConfig>>,
    /// Project host type.
    #[builder(into, default)]
    #[serde(rename = "type")]
    pub r#type: Box<Option<String>>,
}
