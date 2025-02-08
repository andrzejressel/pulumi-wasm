#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetClusterNodePoolNodeConfigWorkloadMetadataConfig {
    /// Mode is the configuration for how to expose metadata to workloads running on the node.
    #[builder(into)]
    #[serde(rename = "mode")]
    pub r#mode: Box<String>,
}
