#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetBudgetCostType {
    /// A boolean value whether to include credits in the cost budget. Defaults to `true`.
    #[builder(into)]
    #[serde(rename = "includeCredit")]
    pub r#include_credit: Box<bool>,
    /// Whether a budget includes discounts. Defaults to `true`.
    #[builder(into)]
    #[serde(rename = "includeDiscount")]
    pub r#include_discount: Box<bool>,
    /// A boolean value whether to include other subscription costs in the cost budget. Defaults to `true`.
    #[builder(into)]
    #[serde(rename = "includeOtherSubscription")]
    pub r#include_other_subscription: Box<bool>,
    /// A boolean value whether to include recurring costs in the cost budget. Defaults to `true`.
    #[builder(into)]
    #[serde(rename = "includeRecurring")]
    pub r#include_recurring: Box<bool>,
    /// A boolean value whether to include refunds in the cost budget. Defaults to `true`.
    #[builder(into)]
    #[serde(rename = "includeRefund")]
    pub r#include_refund: Box<bool>,
    /// A boolean value whether to include subscriptions in the cost budget. Defaults to `true`.
    #[builder(into)]
    #[serde(rename = "includeSubscription")]
    pub r#include_subscription: Box<bool>,
    /// A boolean value whether to include support costs in the cost budget. Defaults to `true`.
    #[builder(into)]
    #[serde(rename = "includeSupport")]
    pub r#include_support: Box<bool>,
    /// A boolean value whether to include tax in the cost budget. Defaults to `true`.
    #[builder(into)]
    #[serde(rename = "includeTax")]
    pub r#include_tax: Box<bool>,
    /// A boolean value whether to include upfront costs in the cost budget. Defaults to `true`.
    #[builder(into)]
    #[serde(rename = "includeUpfront")]
    pub r#include_upfront: Box<bool>,
    /// Whether a budget uses the amortized rate. Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "useAmortized")]
    pub r#use_amortized: Box<bool>,
    /// A boolean value whether to use blended costs in the cost budget. Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "useBlended")]
    pub r#use_blended: Box<bool>,
}
