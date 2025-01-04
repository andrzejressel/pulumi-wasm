#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct BudgetAutoAdjustDataHistoricalOptions {
    /// (Required) - The number of budget periods included in the moving-average calculation that determines your auto-adjusted budget amount.
    #[builder(into)]
    #[serde(rename = "budgetAdjustmentPeriod")]
    pub r#budget_adjustment_period: Box<i32>,
    /// (Optional) - The integer that describes how many budget periods in your BudgetAdjustmentPeriod are included in the calculation of your current budget limit. If the first budget period in your BudgetAdjustmentPeriod has no cost data, then that budget period isn’t included in the average that determines your budget limit. You can’t set your own LookBackAvailablePeriods. The value is automatically calculated from the `budget_adjustment_period` and your historical cost data.
    #[builder(into, default)]
    #[serde(rename = "lookbackAvailablePeriods")]
    pub r#lookback_available_periods: Box<Option<i32>>,
}
