#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct LaunchMetricMonitorMetricDefinition {
    /// Specifies the entity, such as a user or session, that does an action that causes a metric value to be recorded. An example is `userDetails.userID`.
    #[builder(into)]
    #[serde(rename = "entityIdKey")]
    pub r#entity_id_key: Box<String>,
    /// Specifies The EventBridge event pattern that defines how the metric is recorded.
    #[builder(into, default)]
    #[serde(rename = "eventPattern")]
    pub r#event_pattern: Box<Option<String>>,
    /// Specifies the name for the metric.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// Specifies a label for the units that the metric is measuring.
    #[builder(into, default)]
    #[serde(rename = "unitLabel")]
    pub r#unit_label: Box<Option<String>>,
    /// Specifies the value that is tracked to produce the metric.
    #[builder(into)]
    #[serde(rename = "valueKey")]
    pub r#value_key: Box<String>,
}
