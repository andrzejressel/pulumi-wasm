#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct FrontdoorSecurityPolicySecurityPoliciesFirewall {
    /// An `association` block as defined below. Changing this forces a new Front Door Security Policy to be created.
    #[builder(into)]
    #[serde(rename = "association")]
    pub r#association: Box<super::super::types::cdn::FrontdoorSecurityPolicySecurityPoliciesFirewallAssociation>,
    /// The Resource Id of the Front Door Firewall Policy that should be linked to this Front Door Security Policy. Changing this forces a new Front Door Security Policy to be created.
    #[builder(into)]
    #[serde(rename = "cdnFrontdoorFirewallPolicyId")]
    pub r#cdn_frontdoor_firewall_policy_id: Box<String>,
}
