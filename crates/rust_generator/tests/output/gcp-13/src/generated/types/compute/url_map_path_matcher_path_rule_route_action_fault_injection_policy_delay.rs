#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct UrlMapPathMatcherPathRuleRouteActionFaultInjectionPolicyDelay {
    /// Specifies the value of the fixed delay interval.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "fixedDelay")]
    pub r#fixed_delay: Box<super::super::types::compute::UrlMapPathMatcherPathRuleRouteActionFaultInjectionPolicyDelayFixedDelay>,
    /// The percentage of traffic (connections/operations/requests) on which delay will be introduced as part of fault injection.
    /// The value must be between 0.0 and 100.0 inclusive.
    #[builder(into)]
    #[serde(rename = "percentage")]
    pub r#percentage: Box<f64>,
}
