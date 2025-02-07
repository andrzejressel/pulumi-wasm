#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetBudgetSubscriptionTimePeriod {
    /// The end date for the budget.
    #[builder(into)]
    #[serde(rename = "endDate")]
    pub r#end_date: Box<String>,
    /// The start date for the budget.
    #[builder(into)]
    #[serde(rename = "startDate")]
    pub r#start_date: Box<String>,
}
