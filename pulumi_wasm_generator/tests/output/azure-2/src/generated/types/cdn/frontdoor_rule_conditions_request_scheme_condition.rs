#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FrontdoorRuleConditionsRequestSchemeCondition {
    /// The requests protocol to match. Possible values include `HTTP` or `HTTPS`.
    #[builder(into, default)]
    #[serde(rename = "matchValues")]
    pub r#match_values: Box<Option<String>>,
    /// If `true` operator becomes the opposite of its value. Possible values `true` or `false`. Defaults to `false`. Details can be found in the `Condition Operator List` below.
    #[builder(into, default)]
    #[serde(rename = "negateCondition")]
    pub r#negate_condition: Box<Option<bool>>,
    /// Possible value `Equal`. Defaults to `Equal`.
    #[builder(into, default)]
    #[serde(rename = "operator")]
    pub r#operator: Box<Option<String>>,
}
