#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct FlowDefinitionHumanLoopActivationConfigHumanLoopActivationConditionsConfig {
    /// A JSON expressing use-case specific conditions declaratively. If any condition is matched, atomic tasks are created against the configured work team. For more information about how to structure the JSON, see [JSON Schema for Human Loop Activation Conditions in Amazon Augmented AI](https://docs.aws.amazon.com/sagemaker/latest/dg/a2i-human-fallback-conditions-json-schema.html).
    #[builder(into)]
    #[serde(rename = "humanLoopActivationConditions")]
    pub r#human_loop_activation_conditions: Box<String>,
}
