#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct DeliveryPipelineConditionPipelineReadyCondition {
    /// True if the Pipeline is in a valid state. Otherwise at least one condition in `PipelineCondition` is in an invalid state. Iterate over those conditions and see which condition(s) has status = false to find out what is wrong with the Pipeline.
    #[builder(into, default)]
    #[serde(rename = "status")]
    pub r#status: Box<Option<bool>>,
    /// Output only. Most recent time at which the pipeline was updated.
    #[builder(into, default)]
    #[serde(rename = "updateTime")]
    pub r#update_time: Box<Option<String>>,
}
