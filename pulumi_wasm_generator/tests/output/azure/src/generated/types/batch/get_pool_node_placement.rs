#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetPoolNodePlacement {
    /// The placement policy for allocating nodes in the pool.
    #[builder(into)]
    #[serde(rename = "policy")]
    pub r#policy: Box<String>,
}