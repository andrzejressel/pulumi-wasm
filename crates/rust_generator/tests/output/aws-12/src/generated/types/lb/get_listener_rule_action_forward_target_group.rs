#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetListenerRuleActionForwardTargetGroup {
    /// ARN of the Listener Rule.
    /// Either `arn` or `listener_arn` must be set.
    #[builder(into)]
    #[serde(rename = "arn")]
    pub r#arn: Box<String>,
    /// Weight of the target group.
    #[builder(into)]
    #[serde(rename = "weight")]
    pub r#weight: Box<f64>,
}
