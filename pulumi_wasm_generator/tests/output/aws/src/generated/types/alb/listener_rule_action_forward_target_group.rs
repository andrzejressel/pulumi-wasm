#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ListenerRuleActionForwardTargetGroup {
    /// The Amazon Resource Name (ARN) of the target group.
    #[builder(into)]
    #[serde(rename = "arn")]
    pub r#arn: Box<String>,
    /// The weight. The range is 0 to 999.
    #[builder(into, default)]
    #[serde(rename = "weight")]
    pub r#weight: Box<Option<i32>>,
}