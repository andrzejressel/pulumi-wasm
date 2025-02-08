#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct FrontdoorSecurityPolicySecurityPoliciesFirewallAssociation {
    /// One or more `domain` blocks as defined below. Changing this forces a new Front Door Security Policy to be created.
    #[builder(into)]
    #[serde(rename = "domains")]
    pub r#domains: Box<Vec<super::super::types::cdn::FrontdoorSecurityPolicySecurityPoliciesFirewallAssociationDomain>>,
    /// The list of paths to match for this firewall policy. Possible value includes `/*`. Changing this forces a new Front Door Security Policy to be created.
    #[builder(into)]
    #[serde(rename = "patternsToMatch")]
    pub r#patterns_to_match: Box<String>,
}
