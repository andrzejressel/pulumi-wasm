#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct AiFeatureStoreOnlineServingConfigScaling {
    /// The maximum number of nodes to scale up to. Must be greater than minNodeCount, and less than or equal to 10 times of 'minNodeCount'.
    #[builder(into)]
    #[serde(rename = "maxNodeCount")]
    pub r#max_node_count: Box<i32>,
    /// The minimum number of nodes to scale down to. Must be greater than or equal to 1.
    #[builder(into)]
    #[serde(rename = "minNodeCount")]
    pub r#min_node_count: Box<i32>,
}
