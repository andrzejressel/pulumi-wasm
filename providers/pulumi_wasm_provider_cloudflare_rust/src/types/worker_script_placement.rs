#[derive(serde::Serialize)]
pub struct WorkerScriptPlacement {
    #[serde(rename = "mode")]
    pub r#mode: Box<String>,
}
