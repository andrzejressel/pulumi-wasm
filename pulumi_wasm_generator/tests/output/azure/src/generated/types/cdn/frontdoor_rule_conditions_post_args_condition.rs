#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FrontdoorRuleConditionsPostArgsCondition {
    /// One or more string or integer values(e.g. "1") representing the value of the `POST` argument to match. If multiple values are specified, they're evaluated using `OR` logic.
    #[builder(into, default)]
    #[serde(rename = "matchValues")]
    pub r#match_values: Box<Option<Vec<String>>>,
    /// If `true` operator becomes the opposite of its value. Possible values `true` or `false`. Defaults to `false`. Details can be found in the `Condition Operator List` below.
    #[builder(into, default)]
    #[serde(rename = "negateCondition")]
    pub r#negate_condition: Box<Option<bool>>,
    /// A Conditional operator. Possible values include `Any`, `Equal`, `Contains`, `BeginsWith`, `EndsWith`, `LessThan`, `LessThanOrEqual`, `GreaterThan`, `GreaterThanOrEqual` or `RegEx`. Details can be found in the `Condition Operator List` below.
    #[builder(into)]
    #[serde(rename = "operator")]
    pub r#operator: Box<String>,
    /// A string value representing the name of the `POST` argument.
    #[builder(into)]
    #[serde(rename = "postArgsName")]
    pub r#post_args_name: Box<String>,
    /// A Conditional operator. Possible values include `Lowercase`, `RemoveNulls`, `Trim`, `Uppercase`, `UrlDecode` or `UrlEncode`. Details can be found in the `Condition Transform List` below.
    #[builder(into, default)]
    #[serde(rename = "transforms")]
    pub r#transforms: Box<Option<Vec<String>>>,
}