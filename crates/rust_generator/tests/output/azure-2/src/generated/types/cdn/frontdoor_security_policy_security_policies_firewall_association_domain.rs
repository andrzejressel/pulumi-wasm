#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct FrontdoorSecurityPolicySecurityPoliciesFirewallAssociationDomain {
    /// Is the Front Door Custom Domain/Endpoint activated?
    #[builder(into, default)]
    #[serde(rename = "active")]
    pub r#active: Box<Option<bool>>,
    /// The Resource Id of the **Front Door Custom Domain** or **Front Door Endpoint** that should be bound to this Front Door Security Policy. Changing this forces a new Front Door Security Policy to be created.
    #[builder(into)]
    #[serde(rename = "cdnFrontdoorDomainId")]
    pub r#cdn_frontdoor_domain_id: Box<String>,
}
