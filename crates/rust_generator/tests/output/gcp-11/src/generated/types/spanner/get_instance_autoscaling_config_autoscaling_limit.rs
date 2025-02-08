#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetInstanceAutoscalingConfigAutoscalingLimit {
    /// Specifies maximum number of nodes allocated to the instance. If set, this number
    /// should be greater than or equal to min_nodes.
    #[builder(into)]
    #[serde(rename = "maxNodes")]
    pub r#max_nodes: Box<i32>,
    /// Specifies maximum number of processing units allocated to the instance.
    /// If set, this number should be multiples of 1000 and be greater than or equal to
    /// min_processing_units.
    #[builder(into)]
    #[serde(rename = "maxProcessingUnits")]
    pub r#max_processing_units: Box<i32>,
    /// Specifies number of nodes allocated to the instance. If set, this number
    /// should be greater than or equal to 1.
    #[builder(into)]
    #[serde(rename = "minNodes")]
    pub r#min_nodes: Box<i32>,
    /// Specifies minimum number of processing units allocated to the instance.
    /// If set, this number should be multiples of 1000.
    #[builder(into)]
    #[serde(rename = "minProcessingUnits")]
    pub r#min_processing_units: Box<i32>,
}
