#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct PolicyRestorePolicy {
    /// May only be set to true. If set, then the default Policy is restored.
    #[builder(into)]
    #[serde(rename = "default")]
    pub r#default: Box<bool>,
}
