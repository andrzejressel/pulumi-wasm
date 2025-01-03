#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FilterFindingCriteriaCriterion {
    /// List of string values to be evaluated.
    #[builder(into, default)]
    #[serde(rename = "equals")]
    pub r#equals: Box<Option<Vec<String>>>,
    /// The name of the field to be evaluated. The full list of field names can be found in [AWS documentation](https://docs.aws.amazon.com/guardduty/latest/ug/guardduty_filter-findings.html#filter_criteria).
    #[builder(into)]
    #[serde(rename = "field")]
    pub r#field: Box<String>,
    /// A value to be evaluated. Accepts either an integer or a date in [RFC 3339 format](https://tools.ietf.org/html/rfc3339#section-5.8).
    #[builder(into, default)]
    #[serde(rename = "greaterThan")]
    pub r#greater_than: Box<Option<String>>,
    /// A value to be evaluated. Accepts either an integer or a date in [RFC 3339 format](https://tools.ietf.org/html/rfc3339#section-5.8).
    #[builder(into, default)]
    #[serde(rename = "greaterThanOrEqual")]
    pub r#greater_than_or_equal: Box<Option<String>>,
    /// A value to be evaluated. Accepts either an integer or a date in [RFC 3339 format](https://tools.ietf.org/html/rfc3339#section-5.8).
    #[builder(into, default)]
    #[serde(rename = "lessThan")]
    pub r#less_than: Box<Option<String>>,
    /// A value to be evaluated. Accepts either an integer or a date in [RFC 3339 format](https://tools.ietf.org/html/rfc3339#section-5.8).
    #[builder(into, default)]
    #[serde(rename = "lessThanOrEqual")]
    pub r#less_than_or_equal: Box<Option<String>>,
    /// List of string values to be evaluated.
    #[builder(into, default)]
    #[serde(rename = "notEquals")]
    pub r#not_equals: Box<Option<Vec<String>>>,
}
