#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FirewallLogConfig {
    /// This field denotes whether to include or exclude metadata for firewall logs.
    /// Possible values are: `EXCLUDE_ALL_METADATA`, `INCLUDE_ALL_METADATA`.
    #[builder(into)]
    #[serde(rename = "metadata")]
    pub r#metadata: Box<String>,
}
