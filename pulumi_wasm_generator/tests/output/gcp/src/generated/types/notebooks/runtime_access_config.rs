#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct RuntimeAccessConfig {
    /// The type of access mode this instance. For valid values, see
    /// `https://cloud.google.com/vertex-ai/docs/workbench/reference/
    /// rest/v1/projects.locations.runtimes#RuntimeAccessType`.
    #[builder(into, default)]
    #[serde(rename = "accessType")]
    pub r#access_type: Box<Option<String>>,
    /// (Output)
    /// The proxy endpoint that is used to access the runtime.
    #[builder(into, default)]
    #[serde(rename = "proxyUri")]
    pub r#proxy_uri: Box<Option<String>>,
    /// The owner of this runtime after creation. Format: `alias@example.com`.
    /// Currently supports one owner only.
    #[builder(into, default)]
    #[serde(rename = "runtimeOwner")]
    pub r#runtime_owner: Box<Option<String>>,
}
