#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct BudgetActionSubscriber {
    /// The address that AWS sends budget notifications to, either an SNS topic or an email.
    #[builder(into)]
    #[serde(rename = "address")]
    pub r#address: Box<String>,
    /// The type of notification that AWS sends to a subscriber. Valid values are `SNS` or `EMAIL`.
    #[builder(into)]
    #[serde(rename = "subscriptionType")]
    pub r#subscription_type: Box<String>,
}