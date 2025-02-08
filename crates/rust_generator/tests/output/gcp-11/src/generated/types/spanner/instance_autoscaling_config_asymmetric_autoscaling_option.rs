#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct InstanceAutoscalingConfigAsymmetricAutoscalingOption {
    /// A nested object resource.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "overrides")]
    pub r#overrides: Box<super::super::types::spanner::InstanceAutoscalingConfigAsymmetricAutoscalingOptionOverrides>,
    /// A nested object resource.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "replicaSelection")]
    pub r#replica_selection: Box<super::super::types::spanner::InstanceAutoscalingConfigAsymmetricAutoscalingOptionReplicaSelection>,
}
