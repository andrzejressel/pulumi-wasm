#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct BudgetAmountSpecifiedAmount {
    /// The 3-letter currency code defined in ISO 4217.
    #[builder(into, default)]
    #[serde(rename = "currencyCode")]
    pub r#currency_code: Box<Option<String>>,
    /// Number of nano (10^-9) units of the amount.
    /// The value must be between -999,999,999 and +999,999,999
    /// inclusive. If units is positive, nanos must be positive or
    /// zero. If units is zero, nanos can be positive, zero, or
    /// negative. If units is negative, nanos must be negative or
    /// zero. For example $-1.75 is represented as units=-1 and
    /// nanos=-750,000,000.
    /// 
    /// - - -
    #[builder(into, default)]
    #[serde(rename = "nanos")]
    pub r#nanos: Box<Option<i32>>,
    /// The whole units of the amount. For example if currencyCode
    /// is "USD", then 1 unit is one US dollar.
    #[builder(into, default)]
    #[serde(rename = "units")]
    pub r#units: Box<Option<String>>,
}
