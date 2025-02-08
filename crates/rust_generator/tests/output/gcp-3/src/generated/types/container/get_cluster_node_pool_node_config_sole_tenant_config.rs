#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetClusterNodePoolNodeConfigSoleTenantConfig {
    /// .
    #[builder(into)]
    #[serde(rename = "nodeAffinities")]
    pub r#node_affinities: Box<Vec<super::super::types::container::GetClusterNodePoolNodeConfigSoleTenantConfigNodeAffinity>>,
}
