#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetNodeGroupTaint {
    /// The effect of the taint.
    #[builder(into)]
    #[serde(rename = "effect")]
    pub r#effect: Box<String>,
    /// The key of the taint.
    #[builder(into)]
    #[serde(rename = "key")]
    pub r#key: Box<String>,
    /// The value of the taint.
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: Box<String>,
}