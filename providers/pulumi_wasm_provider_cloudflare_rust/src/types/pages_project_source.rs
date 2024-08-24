#[derive(serde::Serialize)]
pub struct PagesProjectSource {
    /// Configuration for the source of the Cloudflare Pages project.
    #[serde(rename = "config")]
    pub r#config: Box<Option<crate::types::PagesProjectSourceConfig>>,
    /// Project host type.
    #[serde(rename = "type")]
    pub r#type: Box<Option<String>>,
}
