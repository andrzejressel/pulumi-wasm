#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetClusterNodePoolNodeConfigSoleTenantConfig {
    /// .
    #[builder(into)]
    #[serde(rename = "nodeAffinities")]
    pub r#node_affinities: Box<Vec<super::super::types::container::GetClusterNodePoolNodeConfigSoleTenantConfigNodeAffinity>>,
}
