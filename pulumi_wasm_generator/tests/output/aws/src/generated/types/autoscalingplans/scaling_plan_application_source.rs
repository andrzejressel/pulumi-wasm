#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ScalingPlanApplicationSource {
    /// ARN of a AWS CloudFormation stack.
    #[builder(into, default)]
    #[serde(rename = "cloudformationStackArn")]
    pub r#cloudformation_stack_arn: Box<Option<String>>,
    /// Set of tags.
    #[builder(into, default)]
    #[serde(rename = "tagFilters")]
    pub r#tag_filters: Box<Option<Vec<super::super::types::autoscalingplans::ScalingPlanApplicationSourceTagFilter>>>,
}