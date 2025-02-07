#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct RecordSetRoutingPolicyWrrHealthCheckedTargets {
    /// The list of internal load balancers to health check.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "internalLoadBalancers")]
    pub r#internal_load_balancers: Box<Vec<super::super::types::dns::RecordSetRoutingPolicyWrrHealthCheckedTargetsInternalLoadBalancer>>,
}
