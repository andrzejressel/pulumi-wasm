#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct BudgetBudgetFilterCustomPeriod {
    /// Optional. The end date of the time period. Budgets with elapsed end date won't be processed.
    /// If unset, specifies to track all usage incurred since the startDate.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "endDate")]
    pub r#end_date: Box<Option<super::super::types::billing::BudgetBudgetFilterCustomPeriodEndDate>>,
    /// A start date is required. The start date must be after January 1, 2017.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "startDate")]
    pub r#start_date: Box<super::super::types::billing::BudgetBudgetFilterCustomPeriodStartDate>,
}
