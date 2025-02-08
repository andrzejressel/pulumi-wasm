#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetDataCollectionRuleDataFlow {
    /// The built-in transform to transform stream data.
    #[builder(into)]
    #[serde(rename = "builtInTransform")]
    pub r#built_in_transform: Box<String>,
    /// Specifies a list of destination names. A `azure_monitor_metrics` data source only allows for stream of kind `Microsoft-InsightsMetrics`.
    #[builder(into)]
    #[serde(rename = "destinations")]
    pub r#destinations: Box<Vec<String>>,
    /// The output stream of the transform. Only required if the data flow changes data to a different stream.
    #[builder(into)]
    #[serde(rename = "outputStream")]
    pub r#output_stream: Box<String>,
    /// Specifies a list of streams that this data source will be sent to. A stream indicates what schema will be used for this data and usually what table in Log Analytics the data will be sent to.
    #[builder(into)]
    #[serde(rename = "streams")]
    pub r#streams: Box<Vec<String>>,
    /// The KQL query to transform stream data.
    #[builder(into)]
    #[serde(rename = "transformKql")]
    pub r#transform_kql: Box<String>,
}
