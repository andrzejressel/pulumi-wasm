#[derive(serde::Serialize)]
pub struct WorkerScriptWebassemblyBinding {
    #[serde(rename = "module")]
    pub r#module: Box<String>,
    #[serde(rename = "name")]
    pub r#name: Box<String>,
}
