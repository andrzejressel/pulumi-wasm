#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct MetricMetricDescriptorLabel {
    /// A human-readable description for the label.
    #[builder(into, default)]
    #[serde(rename = "description")]
    pub r#description: Box<Option<String>>,
    /// The label key.
    #[builder(into)]
    #[serde(rename = "key")]
    pub r#key: Box<String>,
    /// The type of data that can be assigned to the label.
    /// Default value is `STRING`.
    /// Possible values are: `BOOL`, `INT64`, `STRING`.
    #[builder(into, default)]
    #[serde(rename = "valueType")]
    pub r#value_type: Box<Option<String>>,
}
