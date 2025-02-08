#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ScheduledQueryRulesAlertV2Criteria {
    /// A `dimension` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "dimensions")]
    pub r#dimensions: Box<Option<Vec<super::super::types::monitoring::ScheduledQueryRulesAlertV2CriteriaDimension>>>,
    /// A `failing_periods` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "failingPeriods")]
    pub r#failing_periods: Box<Option<super::super::types::monitoring::ScheduledQueryRulesAlertV2CriteriaFailingPeriods>>,
    /// Specifies the column containing the metric measure number.
    /// 
    /// > **Note** `metric_measure_column` is required if `time_aggregation_method` is `Average`, `Maximum`, `Minimum`, or `Total`. And `metric_measure_column` can not be specified if `time_aggregation_method` is `Count`.
    #[builder(into, default)]
    #[serde(rename = "metricMeasureColumn")]
    pub r#metric_measure_column: Box<Option<String>>,
    /// Specifies the criteria operator. Possible values are `Equal`, `GreaterThan`, `GreaterThanOrEqual`, `LessThan`,and `LessThanOrEqual`.
    #[builder(into)]
    #[serde(rename = "operator")]
    pub r#operator: Box<String>,
    /// The query to run on logs. The results returned by this query are used to populate the alert.
    #[builder(into)]
    #[serde(rename = "query")]
    pub r#query: Box<String>,
    /// Specifies the column containing the resource ID. The content of the column must be an uri formatted as resource ID.
    #[builder(into, default)]
    #[serde(rename = "resourceIdColumn")]
    pub r#resource_id_column: Box<Option<String>>,
    /// Specifies the criteria threshold value that activates the alert.
    #[builder(into)]
    #[serde(rename = "threshold")]
    pub r#threshold: Box<f64>,
    /// The type of aggregation to apply to the data points in aggregation granularity. Possible values are `Average`, `Count`, `Maximum`, `Minimum`,and `Total`.
    #[builder(into)]
    #[serde(rename = "timeAggregationMethod")]
    pub r#time_aggregation_method: Box<String>,
}
