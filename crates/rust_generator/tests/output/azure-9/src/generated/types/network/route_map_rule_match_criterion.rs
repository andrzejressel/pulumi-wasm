#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct RouteMapRuleMatchCriterion {
    /// A list of AS paths which this criterion matches.
    #[builder(into, default)]
    #[serde(rename = "asPaths")]
    pub r#as_paths: Box<Option<Vec<String>>>,
    /// A list of BGP communities which this criterion matches.
    #[builder(into, default)]
    #[serde(rename = "communities")]
    pub r#communities: Box<Option<Vec<String>>>,
    /// The match condition to apply the rule of the Route Map. Possible values are `Contains`, `Equals`, `NotContains`, `NotEquals` and `Unknown`.
    #[builder(into)]
    #[serde(rename = "matchCondition")]
    pub r#match_condition: Box<String>,
    /// A list of route prefixes which this criterion matches.
    #[builder(into, default)]
    #[serde(rename = "routePrefixes")]
    pub r#route_prefixes: Box<Option<Vec<String>>>,
}
