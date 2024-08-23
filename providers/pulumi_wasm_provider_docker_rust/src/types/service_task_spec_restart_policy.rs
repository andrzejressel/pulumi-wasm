#[derive(serde::Serialize)]
pub struct ServiceTaskSpecRestartPolicy {
    #[serde(rename = "condition")]
    pub r#condition: Box<Option<String>>,
    #[serde(rename = "delay")]
    pub r#delay: Box<Option<String>>,
    #[serde(rename = "maxAttempts")]
    pub r#max_attempts: Box<Option<i32>>,
    #[serde(rename = "window")]
    pub r#window: Box<Option<String>>,
}
