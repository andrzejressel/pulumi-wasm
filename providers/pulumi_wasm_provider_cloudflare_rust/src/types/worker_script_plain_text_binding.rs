#[derive(serde::Serialize)]
pub struct WorkerScriptPlainTextBinding {
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    #[serde(rename = "text")]
    pub r#text: Box<String>,
}
