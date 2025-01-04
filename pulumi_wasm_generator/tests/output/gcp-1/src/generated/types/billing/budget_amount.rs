#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct BudgetAmount {
    /// Configures a budget amount that is automatically set to 100% of
    /// last period's spend.
    /// Boolean. Set value to true to use. Do not set to false, instead
    /// use the `specified_amount` block.
    #[builder(into, default)]
    #[serde(rename = "lastPeriodAmount")]
    pub r#last_period_amount: Box<Option<bool>>,
    /// A specified amount to use as the budget. currencyCode is
    /// optional. If specified, it must match the currency of the
    /// billing account. The currencyCode is provided on output.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "specifiedAmount")]
    pub r#specified_amount: Box<Option<super::super::types::billing::BudgetAmountSpecifiedAmount>>,
}
