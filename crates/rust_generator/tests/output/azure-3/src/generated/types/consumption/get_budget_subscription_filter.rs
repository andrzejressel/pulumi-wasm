#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetBudgetSubscriptionFilter {
    /// A `dimension` block as defined above.
    #[builder(into)]
    #[serde(rename = "dimensions")]
    pub r#dimensions: Box<Vec<super::super::types::consumption::GetBudgetSubscriptionFilterDimension>>,
    /// A `not` block as defined below.
    #[builder(into)]
    #[serde(rename = "nots")]
    pub r#nots: Box<Vec<super::super::types::consumption::GetBudgetSubscriptionFilterNot>>,
    /// A `tag` block as defined below.
    #[builder(into)]
    #[serde(rename = "tags")]
    pub r#tags: Box<Vec<super::super::types::consumption::GetBudgetSubscriptionFilterTag>>,
}
