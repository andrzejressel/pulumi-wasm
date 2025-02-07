#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct EndpointDeliveryRuleCookiesCondition {
    /// List of values for the cookie. This is required if `operator` is not `Any`.
    #[builder(into, default)]
    #[serde(rename = "matchValues")]
    pub r#match_values: Box<Option<Vec<String>>>,
    /// Defaults to `false`.
    #[builder(into, default)]
    #[serde(rename = "negateCondition")]
    pub r#negate_condition: Box<Option<bool>>,
    /// Valid values are `Any`, `BeginsWith`, `Contains`, `EndsWith`, `Equal`, `GreaterThan`, `GreaterThanOrEqual`, `LessThan` and `LessThanOrEqual`.
    #[builder(into)]
    #[serde(rename = "operator")]
    pub r#operator: Box<String>,
    /// Name of the cookie.
    #[builder(into)]
    #[serde(rename = "selector")]
    pub r#selector: Box<String>,
    /// A list of transforms. Valid values are `Lowercase` and `Uppercase`.
    #[builder(into, default)]
    #[serde(rename = "transforms")]
    pub r#transforms: Box<Option<Vec<String>>>,
}
