#[derive(serde::Serialize)]
pub struct WorkerScriptQueueBinding {
    #[serde(rename = "binding")]
    pub r#binding: Box<String>,
    #[serde(rename = "queue")]
    pub r#queue: Box<String>,
}
