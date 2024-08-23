#[derive(serde::Serialize)]
pub struct ServiceConvergeConfig {
    #[serde(rename = "delay")]
    pub r#delay: Box<Option<String>>,
    #[serde(rename = "timeout")]
    pub r#timeout: Box<Option<String>>,
}
