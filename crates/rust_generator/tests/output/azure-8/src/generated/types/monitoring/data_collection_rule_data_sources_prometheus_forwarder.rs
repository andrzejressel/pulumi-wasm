#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct DataCollectionRuleDataSourcesPrometheusForwarder {
    /// One or more `label_include_filter` blocks as defined above.
    #[builder(into, default)]
    #[serde(rename = "labelIncludeFilters")]
    pub r#label_include_filters: Box<Option<Vec<super::super::types::monitoring::DataCollectionRuleDataSourcesPrometheusForwarderLabelIncludeFilter>>>,
    /// The name which should be used for this data source. This name should be unique across all data sources regardless of type within the Data Collection Rule.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// Specifies a list of streams that this data source will be sent to. A stream indicates what schema will be used for this data and usually what table in Log Analytics the data will be sent to. Possible value is `Microsoft-PrometheusMetrics`.
    #[builder(into)]
    #[serde(rename = "streams")]
    pub r#streams: Box<Vec<String>>,
}
