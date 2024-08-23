#[derive(serde::Serialize)]
pub struct PagesProjectSource {
    #[serde(rename = "config")]
    pub r#config: Box<Option<crate::types::PagesProjectSourceConfig>>,
    #[serde(rename = "type")]
    pub r#type: Box<Option<String>>,
}
