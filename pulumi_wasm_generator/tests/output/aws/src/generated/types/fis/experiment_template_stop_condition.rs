#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ExperimentTemplateStopCondition {
    /// Source of the condition. One of `none`, `aws:cloudwatch:alarm`.
    #[builder(into)]
    #[serde(rename = "source")]
    pub r#source: Box<String>,
    /// ARN of the CloudWatch alarm. Required if the source is a CloudWatch alarm.
    #[builder(into, default)]
    #[serde(rename = "value")]
    pub r#value: Box<Option<String>>,
}
