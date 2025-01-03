#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ServiceMode {
    /// When `true`, tasks will run on every worker node. Conflicts with `replicated`
    #[builder(into, default)]
    #[serde(rename = "global")]
    pub r#global: Box<Option<bool>>,
    /// The replicated service mode
    #[builder(into, default)]
    #[serde(rename = "replicated")]
    pub r#replicated: Box<Option<super::types::ServiceModeReplicated>>,
}
