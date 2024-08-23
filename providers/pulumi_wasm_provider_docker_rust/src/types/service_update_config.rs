#[derive(serde::Serialize)]
pub struct ServiceUpdateConfig {
    #[serde(rename = "delay")]
    pub r#delay: Box<Option<String>>,
    #[serde(rename = "failureAction")]
    pub r#failure_action: Box<Option<String>>,
    #[serde(rename = "maxFailureRatio")]
    pub r#max_failure_ratio: Box<Option<String>>,
    #[serde(rename = "monitor")]
    pub r#monitor: Box<Option<String>>,
    #[serde(rename = "order")]
    pub r#order: Box<Option<String>>,
    #[serde(rename = "parallelism")]
    pub r#parallelism: Box<Option<i32>>,
}
