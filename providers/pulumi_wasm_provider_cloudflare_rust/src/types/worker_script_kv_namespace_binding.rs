#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct WorkerScriptKvNamespaceBinding {
    /// The global variable for the binding in your Worker code.
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// ID of the KV namespace you want to use.
    #[serde(rename = "namespaceId")]
    pub r#namespace_id: Box<String>,
}
