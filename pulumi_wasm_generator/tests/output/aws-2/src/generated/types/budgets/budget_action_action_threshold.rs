#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct BudgetActionActionThreshold {
    /// The type of threshold for a notification. Valid values are `PERCENTAGE` or `ABSOLUTE_VALUE`.
    #[builder(into)]
    #[serde(rename = "actionThresholdType")]
    pub r#action_threshold_type: Box<String>,
    /// The threshold of a notification.
    #[builder(into)]
    #[serde(rename = "actionThresholdValue")]
    pub r#action_threshold_value: Box<f64>,
}
