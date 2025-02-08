#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct EdgeCacheServiceRoutingPathMatcherRouteRule {
    /// A human-readable description of the routeRule.
    #[builder(into, default)]
    #[serde(rename = "description")]
    pub r#description: Box<Option<String>>,
    /// The header actions, including adding & removing headers, for requests that match this route.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "headerAction")]
    pub r#header_action: Box<Option<super::super::types::networkservices::EdgeCacheServiceRoutingPathMatcherRouteRuleHeaderAction>>,
    /// The list of criteria for matching attributes of a request to this routeRule. This list has OR semantics: the request matches this routeRule when any of the matchRules are satisfied. However predicates
    /// within a given matchRule have AND semantics. All predicates within a matchRule must match for the request to match the rule.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "matchRules")]
    pub r#match_rules: Box<Vec<super::super::types::networkservices::EdgeCacheServiceRoutingPathMatcherRouteRuleMatchRule>>,
    /// The Origin resource that requests to this route should fetch from when a matching response is not in cache. Origins can be defined as short names ("my-origin") or fully-qualified resource URLs - e.g. "networkservices.googleapis.com/projects/my-project/global/edgecacheorigins/my-origin"
    /// Only one of origin or urlRedirect can be set.
    #[builder(into, default)]
    #[serde(rename = "origin")]
    pub r#origin: Box<Option<String>>,
    /// The priority of this route rule, where 1 is the highest priority.
    /// You cannot configure two or more routeRules with the same priority. Priority for each rule must be set to a number between 1 and 999 inclusive.
    /// Priority numbers can have gaps, which enable you to add or remove rules in the future without affecting the rest of the rules. For example, 1, 2, 3, 4, 5, 9, 12, 16 is a valid series of priority numbers
    /// to which you could add rules numbered from 6 to 8, 10 to 11, and 13 to 15 in the future without any impact on existing rules.
    #[builder(into)]
    #[serde(rename = "priority")]
    pub r#priority: Box<String>,
    /// In response to a matching path, the routeAction performs advanced routing actions like URL rewrites, header transformations, etc. prior to forwarding the request to the selected origin.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "routeAction")]
    pub r#route_action: Box<Option<super::super::types::networkservices::EdgeCacheServiceRoutingPathMatcherRouteRuleRouteAction>>,
    /// The URL redirect configuration for requests that match this route.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "urlRedirect")]
    pub r#url_redirect: Box<Option<super::super::types::networkservices::EdgeCacheServiceRoutingPathMatcherRouteRuleUrlRedirect>>,
}
