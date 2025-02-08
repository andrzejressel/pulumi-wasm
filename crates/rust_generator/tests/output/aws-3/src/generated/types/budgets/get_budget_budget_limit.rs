#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetBudgetBudgetLimit {
    /// The cost or usage amount that's associated with a budget forecast, actual spend, or budget threshold. Length Constraints: Minimum length of `1`. Maximum length of `2147483647`.
    #[builder(into)]
    #[serde(rename = "amount")]
    pub r#amount: Box<String>,
    /// The unit of measurement that's used for the budget forecast, actual spend, or budget threshold, such as USD or GBP. Length Constraints: Minimum length of `1`. Maximum length of `2147483647`.
    #[builder(into)]
    #[serde(rename = "unit")]
    pub r#unit: Box<String>,
}
