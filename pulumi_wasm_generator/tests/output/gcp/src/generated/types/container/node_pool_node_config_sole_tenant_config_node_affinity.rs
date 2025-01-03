#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct NodePoolNodeConfigSoleTenantConfigNodeAffinity {
    /// .
    #[builder(into)]
    #[serde(rename = "key")]
    pub r#key: Box<String>,
    /// .
    #[builder(into)]
    #[serde(rename = "operator")]
    pub r#operator: Box<String>,
    /// .
    #[builder(into)]
    #[serde(rename = "values")]
    pub r#values: Box<Vec<String>>,
}
