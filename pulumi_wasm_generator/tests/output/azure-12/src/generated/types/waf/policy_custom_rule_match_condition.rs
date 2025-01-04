#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct PolicyCustomRuleMatchCondition {
    /// A list of match values. This is **Required** when the `operator` is not `Any`.
    #[builder(into, default)]
    #[serde(rename = "matchValues")]
    pub r#match_values: Box<Option<Vec<String>>>,
    /// One or more `match_variables` blocks as defined below.
    #[builder(into)]
    #[serde(rename = "matchVariables")]
    pub r#match_variables: Box<Vec<super::super::types::waf::PolicyCustomRuleMatchConditionMatchVariable>>,
    /// Describes if this is negate condition or not
    #[builder(into, default)]
    #[serde(rename = "negationCondition")]
    pub r#negation_condition: Box<Option<bool>>,
    /// Describes operator to be matched. Possible values are `Any`, `IPMatch`, `GeoMatch`, `Equal`, `Contains`, `LessThan`, `GreaterThan`, `LessThanOrEqual`, `GreaterThanOrEqual`, `BeginsWith`, `EndsWith` and `Regex`.
    #[builder(into)]
    #[serde(rename = "operator")]
    pub r#operator: Box<String>,
    /// A list of transformations to do before the match is attempted. Possible values are `HtmlEntityDecode`, `Lowercase`, `RemoveNulls`, `Trim`, `Uppercase`, `UrlDecode` and `UrlEncode`.
    #[builder(into, default)]
    #[serde(rename = "transforms")]
    pub r#transforms: Box<Option<Vec<String>>>,
}
