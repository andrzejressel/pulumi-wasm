#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct HttpRouteRule {
    /// The detailed rule defining how to route matched traffic.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "action")]
    pub r#action: Box<Option<super::super::types::networkservices::HttpRouteRuleAction>>,
    /// A list of matches define conditions used for matching the rule against incoming HTTP requests. Each match is independent, i.e. this rule will be matched if ANY one of the matches is satisfied.
    /// If no matches field is specified, this rule will unconditionally match traffic.
    /// If a default rule is desired to be configured, add a rule with no matches specified to the end of the rules list.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "matches")]
    pub r#matches: Box<Option<Vec<super::super::types::networkservices::HttpRouteRuleMatch>>>,
}
