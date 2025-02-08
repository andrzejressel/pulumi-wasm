#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetDataCollectionRuleDataSourcePrometheusForwarder {
    /// One or more `label_include_filter` blocks as defined above.
    #[builder(into)]
    #[serde(rename = "labelIncludeFilters")]
    pub r#label_include_filters: Box<Vec<super::super::types::monitoring::GetDataCollectionRuleDataSourcePrometheusForwarderLabelIncludeFilter>>,
    /// Specifies the name of the Data Collection Rule.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// Specifies a list of streams that this data source will be sent to. A stream indicates what schema will be used for this data and usually what table in Log Analytics the data will be sent to.
    #[builder(into)]
    #[serde(rename = "streams")]
    pub r#streams: Box<Vec<String>>,
}
