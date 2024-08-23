#[derive(serde::Serialize)]
pub struct ServiceMode {
    #[serde(rename = "global")]
    pub r#global: Box<Option<bool>>,
    #[serde(rename = "replicated")]
    pub r#replicated: Box<Option<crate::types::ServiceModeReplicated>>,
}
