#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct HttpRouteRuleActionFaultInjectionPolicyDelay {
    /// Specify a fixed delay before forwarding the request.
    #[builder(into, default)]
    #[serde(rename = "fixedDelay")]
    pub r#fixed_delay: Box<Option<String>>,
    /// The percentage of traffic on which delay will be injected.
    #[builder(into, default)]
    #[serde(rename = "percentage")]
    pub r#percentage: Box<Option<i32>>,
}
