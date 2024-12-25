#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
pub struct WorkerScriptKvNamespaceBinding {
    /// The global variable for the binding in your Worker code.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// ID of the KV namespace you want to use.
    #[builder(into)]
    #[serde(rename = "namespaceId")]
    pub r#namespace_id: Box<String>,
}
