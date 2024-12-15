#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
pub struct PagesProjectSource {
    /// Configuration for the source of the Cloudflare Pages project.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "config")]
    pub r#config: Box<Option<crate::types::PagesProjectSourceConfig>>,
    /// Project host type.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "type")]
    pub r#type: Box<Option<String>>,
}
