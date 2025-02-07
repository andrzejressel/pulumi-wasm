#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct InsightFiltersFindingProviderFieldsCriticality {
    /// The equal-to condition to be applied to a single field when querying for findings, provided as a String.
    #[builder(into, default)]
    #[serde(rename = "eq")]
    pub r#eq: Box<Option<String>>,
    /// The greater-than-equal condition to be applied to a single field when querying for findings, provided as a String.
    #[builder(into, default)]
    #[serde(rename = "gte")]
    pub r#gte: Box<Option<String>>,
    /// The less-than-equal condition to be applied to a single field when querying for findings, provided as a String.
    #[builder(into, default)]
    #[serde(rename = "lte")]
    pub r#lte: Box<Option<String>>,
}
