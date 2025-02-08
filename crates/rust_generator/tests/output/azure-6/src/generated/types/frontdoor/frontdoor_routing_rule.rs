#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct FrontdoorRoutingRule {
    /// Protocol schemes to match for the Backend Routing Rule. Possible values are `Http` and `Https`.
    #[builder(into)]
    #[serde(rename = "acceptedProtocols")]
    pub r#accepted_protocols: Box<Vec<String>>,
    /// `Enable` or `Disable` use of this Backend Routing Rule. Permitted values are `true` or `false`. Defaults to `true`.
    #[builder(into, default)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<Option<bool>>,
    /// A `forwarding_configuration` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "forwardingConfiguration")]
    pub r#forwarding_configuration: Box<Option<super::super::types::frontdoor::FrontdoorRoutingRuleForwardingConfiguration>>,
    /// The names of the `frontend_endpoint` blocks within this resource to associate with this `routing_rule`.
    #[builder(into)]
    #[serde(rename = "frontendEndpoints")]
    pub r#frontend_endpoints: Box<Vec<String>>,
    /// The ID of the FrontDoor.
    #[builder(into, default)]
    #[serde(rename = "id")]
    pub r#id: Box<Option<String>>,
    /// Specifies the name of the Routing Rule.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The route patterns for the Backend Routing Rule.
    #[builder(into)]
    #[serde(rename = "patternsToMatches")]
    pub r#patterns_to_matches: Box<Vec<String>>,
    /// A `redirect_configuration` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "redirectConfiguration")]
    pub r#redirect_configuration: Box<Option<super::super::types::frontdoor::FrontdoorRoutingRuleRedirectConfiguration>>,
}
