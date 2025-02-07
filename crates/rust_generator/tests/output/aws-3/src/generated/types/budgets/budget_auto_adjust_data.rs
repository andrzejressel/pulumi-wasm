#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct BudgetAutoAdjustData {
    /// (Required) - The string that defines whether your budget auto-adjusts based on historical or forecasted data. Valid values: `FORECAST`,`HISTORICAL`
    #[builder(into)]
    #[serde(rename = "autoAdjustType")]
    pub r#auto_adjust_type: Box<String>,
    /// (Optional) - Configuration block of Historical Options. Required for `auto_adjust_type` of `HISTORICAL` Configuration block that defines the historical data that your auto-adjusting budget is based on.
    #[builder(into, default)]
    #[serde(rename = "historicalOptions")]
    pub r#historical_options: Box<Option<super::super::types::budgets::BudgetAutoAdjustDataHistoricalOptions>>,
    /// (Optional) - The last time that your budget was auto-adjusted.
    #[builder(into, default)]
    #[serde(rename = "lastAutoAdjustTime")]
    pub r#last_auto_adjust_time: Box<Option<String>>,
}
