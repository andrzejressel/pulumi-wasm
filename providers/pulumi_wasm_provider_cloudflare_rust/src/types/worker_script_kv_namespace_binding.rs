#[derive(serde::Serialize)]
pub struct WorkerScriptKvNamespaceBinding {
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    #[serde(rename = "namespaceId")]
    pub r#namespace_id: Box<String>,
}
