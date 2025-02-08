#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct FlowDestinationFlowConfigDestinationConnectorPropertiesS3S3OutputFormatConfigPrefixConfig {
    /// Determines the level of granularity that's included in the prefix. Valid values are `YEAR`, `MONTH`, `DAY`, `HOUR`, and `MINUTE`.
    #[builder(into, default)]
    #[serde(rename = "prefixFormat")]
    pub r#prefix_format: Box<Option<String>>,
    /// Determines whether the destination file path includes either or both of the selected elements. Valid values are `EXECUTION_ID` and `SCHEMA_VERSION`
    #[builder(into, default)]
    #[serde(rename = "prefixHierarchies")]
    pub r#prefix_hierarchies: Box<Option<Vec<String>>>,
    /// Determines the format of the prefix, and whether it applies to the file name, file path, or both. Valid values are `FILENAME`, `PATH`, and `PATH_AND_FILENAME`.
    #[builder(into, default)]
    #[serde(rename = "prefixType")]
    pub r#prefix_type: Box<Option<String>>,
}
