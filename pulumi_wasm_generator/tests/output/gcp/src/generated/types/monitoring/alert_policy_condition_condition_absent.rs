#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AlertPolicyConditionConditionAbsent {
    /// Specifies the alignment of data points in
    /// individual time series as well as how to
    /// combine the retrieved time series together
    /// (such as when aggregating multiple streams
    /// on each resource to a single stream for each
    /// resource or when aggregating streams across
    /// all members of a group of resources).
    /// Multiple aggregations are applied in the
    /// order specified.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "aggregations")]
    pub r#aggregations: Box<Option<Vec<super::super::types::monitoring::AlertPolicyConditionConditionAbsentAggregation>>>,
    /// The amount of time that a time series must
    /// fail to report new data to be considered
    /// failing. Currently, only values that are a
    /// multiple of a minute--e.g. 60s, 120s, or 300s
    /// --are supported.
    #[builder(into)]
    #[serde(rename = "duration")]
    pub r#duration: Box<String>,
    /// A filter that identifies which time series
    /// should be compared with the threshold.The
    /// filter is similar to the one that is
    /// specified in the
    /// MetricService.ListTimeSeries request (that
    /// call is useful to verify the time series
    /// that will be retrieved / processed) and must
    /// specify the metric type and optionally may
    /// contain restrictions on resource type,
    /// resource labels, and metric labels. This
    /// field may not exceed 2048 Unicode characters
    /// in length.
    #[builder(into, default)]
    #[serde(rename = "filter")]
    pub r#filter: Box<Option<String>>,
    /// The number/percent of time series for which
    /// the comparison must hold in order for the
    /// condition to trigger. If unspecified, then
    /// the condition will trigger if the comparison
    /// is true for any of the time series that have
    /// been identified by filter and aggregations.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "trigger")]
    pub r#trigger: Box<Option<super::super::types::monitoring::AlertPolicyConditionConditionAbsentTrigger>>,
}
