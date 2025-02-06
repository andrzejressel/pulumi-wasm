#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct MetricDescriptorLabel {
    /// A human-readable description for the label.
    #[builder(into, default)]
    #[serde(rename = "description")]
    pub r#description: Box<Option<String>>,
    /// The key for this label. The key must not exceed 100 characters. The first character of the key must be an upper- or lower-case letter, the remaining characters must be letters, digits or underscores, and the key must match the regular expression [a-zA-Z][a-zA-Z0-9_]*
    #[builder(into)]
    #[serde(rename = "key")]
    pub r#key: Box<String>,
    /// The type of data that can be assigned to the label.
    /// Default value is `STRING`.
    /// Possible values are: `STRING`, `BOOL`, `INT64`.
    #[builder(into, default)]
    #[serde(rename = "valueType")]
    pub r#value_type: Box<Option<String>>,
}
