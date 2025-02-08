#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetInstanceAutoscalingConfigAsymmetricAutoscalingOption {
    /// A nested object resource.
    #[builder(into)]
    #[serde(rename = "overrides")]
    pub r#overrides: Box<Vec<super::super::types::spanner::GetInstanceAutoscalingConfigAsymmetricAutoscalingOptionOverride>>,
    /// A nested object resource.
    #[builder(into)]
    #[serde(rename = "replicaSelections")]
    pub r#replica_selections: Box<Vec<super::super::types::spanner::GetInstanceAutoscalingConfigAsymmetricAutoscalingOptionReplicaSelection>>,
}
