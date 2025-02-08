#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct DeliveryPipelineCondition {
    /// Details around the Pipeline's overall status.
    #[builder(into, default)]
    #[serde(rename = "pipelineReadyConditions")]
    pub r#pipeline_ready_conditions: Box<Option<Vec<super::super::types::clouddeploy::DeliveryPipelineConditionPipelineReadyCondition>>>,
    /// Details around targets enumerated in the pipeline.
    #[builder(into, default)]
    #[serde(rename = "targetsPresentConditions")]
    pub r#targets_present_conditions: Box<Option<Vec<super::super::types::clouddeploy::DeliveryPipelineConditionTargetsPresentCondition>>>,
    /// Details on the whether the targets enumerated in the pipeline are of the same type.
    #[builder(into, default)]
    #[serde(rename = "targetsTypeConditions")]
    pub r#targets_type_conditions: Box<Option<Vec<super::super::types::clouddeploy::DeliveryPipelineConditionTargetsTypeCondition>>>,
}
