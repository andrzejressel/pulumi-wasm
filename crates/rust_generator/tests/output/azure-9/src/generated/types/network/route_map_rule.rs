#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct RouteMapRule {
    /// An `action` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "actions")]
    pub r#actions: Box<Option<Vec<super::super::types::network::RouteMapRuleAction>>>,
    /// A `match_criterion` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "matchCriterions")]
    pub r#match_criterions: Box<Option<Vec<super::super::types::network::RouteMapRuleMatchCriterion>>>,
    /// The unique name for the rule.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The next step after the rule is evaluated. Possible values are `Continue`, `Terminate` and `Unknown`. Defaults to `Unknown`.
    #[builder(into, default)]
    #[serde(rename = "nextStepIfMatched")]
    pub r#next_step_if_matched: Box<Option<String>>,
}
