#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetInstanceAutoscalingConfigAsymmetricAutoscalingOptionOverrideAutoscalingLimit {
    /// The maximum number of nodes for this specific replica.
    #[builder(into)]
    #[serde(rename = "maxNodes")]
    pub r#max_nodes: Box<i32>,
    /// The minimum number of nodes for this specific replica.
    #[builder(into)]
    #[serde(rename = "minNodes")]
    pub r#min_nodes: Box<i32>,
}
