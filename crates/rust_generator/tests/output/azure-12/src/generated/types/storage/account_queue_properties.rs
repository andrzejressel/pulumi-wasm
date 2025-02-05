#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AccountQueueProperties {
    /// A `cors_rule` block as defined above.
    #[builder(into, default)]
    #[serde(rename = "corsRules")]
    pub r#cors_rules: Box<Option<Vec<super::super::types::storage::AccountQueuePropertiesCorsRule>>>,
    /// A `hour_metrics` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "hourMetrics")]
    pub r#hour_metrics: Box<Option<super::super::types::storage::AccountQueuePropertiesHourMetrics>>,
    /// A `logging` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "logging")]
    pub r#logging: Box<Option<super::super::types::storage::AccountQueuePropertiesLogging>>,
    /// A `minute_metrics` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "minuteMetrics")]
    pub r#minute_metrics: Box<Option<super::super::types::storage::AccountQueuePropertiesMinuteMetrics>>,
}
