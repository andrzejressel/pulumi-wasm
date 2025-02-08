#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ClusterWorkloadAltsConfig {
    /// Whether the alts handshaker should be enabled or not for direct-path. Requires Workload Identity (workloadPool) must be non-empty).
    #[builder(into)]
    #[serde(rename = "enableAlts")]
    pub r#enable_alts: Box<bool>,
}
