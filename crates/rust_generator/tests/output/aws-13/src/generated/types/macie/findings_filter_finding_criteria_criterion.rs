#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FindingsFilterFindingCriteriaCriterion {
    /// The value for the property exclusively matches (equals an exact match for) all the specified values. If you specify multiple values, Amazon Macie uses AND logic to join the values.
    #[builder(into, default)]
    #[serde(rename = "eqExactMatches")]
    pub r#eq_exact_matches: Box<Option<Vec<String>>>,
    /// The value for the property matches (equals) the specified value. If you specify multiple values, Amazon Macie uses OR logic to join the values.
    #[builder(into, default)]
    #[serde(rename = "eqs")]
    pub r#eqs: Box<Option<Vec<String>>>,
    /// The name of the field to be evaluated.
    #[builder(into)]
    #[serde(rename = "field")]
    pub r#field: Box<String>,
    /// The value for the property is greater than the specified value.
    #[builder(into, default)]
    #[serde(rename = "gt")]
    pub r#gt: Box<Option<String>>,
    /// The value for the property is greater than or equal to the specified value.
    #[builder(into, default)]
    #[serde(rename = "gte")]
    pub r#gte: Box<Option<String>>,
    /// The value for the property is less than the specified value.
    #[builder(into, default)]
    #[serde(rename = "lt")]
    pub r#lt: Box<Option<String>>,
    /// The value for the property is less than or equal to the specified value.
    #[builder(into, default)]
    #[serde(rename = "lte")]
    pub r#lte: Box<Option<String>>,
    /// The value for the property doesn't match (doesn't equal) the specified value. If you specify multiple values, Amazon Macie uses OR logic to join the values.
    #[builder(into, default)]
    #[serde(rename = "neqs")]
    pub r#neqs: Box<Option<Vec<String>>>,
}
