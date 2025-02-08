#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct BudgetCostTypes {
    /// A boolean value whether to include credits in the cost budget. Defaults to `true`
    #[builder(into, default)]
    #[serde(rename = "includeCredit")]
    pub r#include_credit: Box<Option<bool>>,
    /// Whether a budget includes discounts. Defaults to `true`
    #[builder(into, default)]
    #[serde(rename = "includeDiscount")]
    pub r#include_discount: Box<Option<bool>>,
    /// A boolean value whether to include other subscription costs in the cost budget. Defaults to `true`
    #[builder(into, default)]
    #[serde(rename = "includeOtherSubscription")]
    pub r#include_other_subscription: Box<Option<bool>>,
    /// A boolean value whether to include recurring costs in the cost budget. Defaults to `true`
    #[builder(into, default)]
    #[serde(rename = "includeRecurring")]
    pub r#include_recurring: Box<Option<bool>>,
    /// A boolean value whether to include refunds in the cost budget. Defaults to `true`
    #[builder(into, default)]
    #[serde(rename = "includeRefund")]
    pub r#include_refund: Box<Option<bool>>,
    /// A boolean value whether to include subscriptions in the cost budget. Defaults to `true`
    #[builder(into, default)]
    #[serde(rename = "includeSubscription")]
    pub r#include_subscription: Box<Option<bool>>,
    /// A boolean value whether to include support costs in the cost budget. Defaults to `true`
    #[builder(into, default)]
    #[serde(rename = "includeSupport")]
    pub r#include_support: Box<Option<bool>>,
    /// A boolean value whether to include tax in the cost budget. Defaults to `true`
    #[builder(into, default)]
    #[serde(rename = "includeTax")]
    pub r#include_tax: Box<Option<bool>>,
    /// A boolean value whether to include upfront costs in the cost budget. Defaults to `true`
    #[builder(into, default)]
    #[serde(rename = "includeUpfront")]
    pub r#include_upfront: Box<Option<bool>>,
    /// Whether a budget uses the amortized rate. Defaults to `false`
    #[builder(into, default)]
    #[serde(rename = "useAmortized")]
    pub r#use_amortized: Box<Option<bool>>,
    /// A boolean value whether to use blended costs in the cost budget. Defaults to `false`
    #[builder(into, default)]
    #[serde(rename = "useBlended")]
    pub r#use_blended: Box<Option<bool>>,
}
