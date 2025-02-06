#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetListenerDefaultActionForwardTargetGroup {
    /// ARN of the listener. Required if `load_balancer_arn` and `port` is not set.
    #[builder(into)]
    #[serde(rename = "arn")]
    pub r#arn: Box<String>,
    #[builder(into)]
    #[serde(rename = "weight")]
    pub r#weight: Box<i32>,
}
