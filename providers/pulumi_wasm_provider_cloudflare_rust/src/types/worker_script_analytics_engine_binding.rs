#[derive(serde::Serialize)]
pub struct WorkerScriptAnalyticsEngineBinding {
    #[serde(rename = "dataset")]
    pub r#dataset: Box<String>,
    #[serde(rename = "name")]
    pub r#name: Box<String>,
}
