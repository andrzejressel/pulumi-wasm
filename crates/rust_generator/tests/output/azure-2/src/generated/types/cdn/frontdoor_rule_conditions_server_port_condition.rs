#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct FrontdoorRuleConditionsServerPortCondition {
    /// A list of one or more integer values(e.g. "1") representing the value of the client port to match. Possible values include `80` or `443`. If multiple values are specified, they're evaluated using `OR` logic.
    #[builder(into)]
    #[serde(rename = "matchValues")]
    pub r#match_values: Box<Vec<String>>,
    /// If `true` operator becomes the opposite of its value. Possible values `true` or `false`. Defaults to `false`. Details can be found in the `Condition Operator List` below.
    #[builder(into, default)]
    #[serde(rename = "negateCondition")]
    pub r#negate_condition: Box<Option<bool>>,
    /// A Conditional operator. Possible values include `Any`, `Equal`, `Contains`, `BeginsWith`, `EndsWith`, `LessThan`, `LessThanOrEqual`, `GreaterThan`, `GreaterThanOrEqual` or `RegEx`. Details can be found in the `Condition Operator List` below.
    #[builder(into)]
    #[serde(rename = "operator")]
    pub r#operator: Box<String>,
}
