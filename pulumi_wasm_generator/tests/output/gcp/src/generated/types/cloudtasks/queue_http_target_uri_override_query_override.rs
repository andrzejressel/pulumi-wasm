#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct QueueHttpTargetUriOverrideQueryOverride {
    /// The query parameters (e.g., qparam1=123&qparam2=456). Default is an empty string.
    #[builder(into, default)]
    #[serde(rename = "queryParams")]
    pub r#query_params: Box<Option<String>>,
}
