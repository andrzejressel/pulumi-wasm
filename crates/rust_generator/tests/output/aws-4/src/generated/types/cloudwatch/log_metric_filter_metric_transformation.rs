#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct LogMetricFilterMetricTransformation {
    /// The value to emit when a filter pattern does not match a log event. Conflicts with `dimensions`.
    #[builder(into, default)]
    #[serde(rename = "defaultValue")]
    pub r#default_value: Box<Option<String>>,
    /// Map of fields to use as dimensions for the metric. Up to 3 dimensions are allowed. Conflicts with `default_value`.
    #[builder(into, default)]
    #[serde(rename = "dimensions")]
    pub r#dimensions: Box<Option<std::collections::HashMap<String, String>>>,
    /// The name of the CloudWatch metric to which the monitored log information should be published (e.g., `ErrorCount`)
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The destination namespace of the CloudWatch metric.
    #[builder(into)]
    #[serde(rename = "namespace")]
    pub r#namespace: Box<String>,
    /// The unit to assign to the metric. If you omit this, the unit is set as `None`.
    #[builder(into, default)]
    #[serde(rename = "unit")]
    pub r#unit: Box<Option<String>>,
    /// What to publish to the metric. For example, if you're counting the occurrences of a particular term like "Error", the value will be "1" for each occurrence. If you're counting the bytes transferred the published value will be the value in the log event.
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: Box<String>,
}
