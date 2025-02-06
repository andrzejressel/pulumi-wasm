#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct InsightFiltersThreatIntelIndicatorLastObservedAt {
    /// A configuration block of the date range for the date filter. See date_range below for more details.
    #[builder(into, default)]
    #[serde(rename = "dateRange")]
    pub r#date_range: Box<Option<super::super::types::securityhub::InsightFiltersThreatIntelIndicatorLastObservedAtDateRange>>,
    /// An end date for the date filter. Required with `start` if `date_range` is not specified.
    #[builder(into, default)]
    #[serde(rename = "end")]
    pub r#end: Box<Option<String>>,
    /// A start date for the date filter. Required with `end` if `date_range` is not specified.
    #[builder(into, default)]
    #[serde(rename = "start")]
    pub r#start: Box<Option<String>>,
}
