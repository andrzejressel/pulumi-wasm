#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AlertPolicyConditionConditionThreshold {
    /// Specifies the alignment of data points in
    /// individual time series as well as how to
    /// combine the retrieved time series together
    /// (such as when aggregating multiple streams
    /// on each resource to a single stream for each
    /// resource or when aggregating streams across
    /// all members of a group of resources).
    /// Multiple aggregations are applied in the
    /// order specified.This field is similar to the
    /// one in the MetricService.ListTimeSeries
    /// request. It is advisable to use the
    /// ListTimeSeries method when debugging this
    /// field.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "aggregations")]
    pub r#aggregations: Box<Option<Vec<super::super::types::monitoring::AlertPolicyConditionConditionThresholdAggregation>>>,
    /// The comparison to apply between the time
    /// series (indicated by filter and aggregation)
    /// and the threshold (indicated by
    /// threshold_value). The comparison is applied
    /// on each time series, with the time series on
    /// the left-hand side and the threshold on the
    /// right-hand side. Only COMPARISON_LT and
    /// COMPARISON_GT are supported currently.
    /// Possible values are: `COMPARISON_GT`, `COMPARISON_GE`, `COMPARISON_LT`, `COMPARISON_LE`, `COMPARISON_EQ`, `COMPARISON_NE`.
    #[builder(into)]
    #[serde(rename = "comparison")]
    pub r#comparison: Box<String>,
    /// Specifies the alignment of data points in
    /// individual time series selected by
    /// denominatorFilter as well as how to combine
    /// the retrieved time series together (such as
    /// when aggregating multiple streams on each
    /// resource to a single stream for each
    /// resource or when aggregating streams across
    /// all members of a group of resources).When
    /// computing ratios, the aggregations and
    /// denominator_aggregations fields must use the
    /// same alignment period and produce time
    /// series that have the same periodicity and
    /// labels.This field is similar to the one in
    /// the MetricService.ListTimeSeries request. It
    /// is advisable to use the ListTimeSeries
    /// method when debugging this field.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "denominatorAggregations")]
    pub r#denominator_aggregations: Box<Option<Vec<super::super::types::monitoring::AlertPolicyConditionConditionThresholdDenominatorAggregation>>>,
    /// A filter that identifies a time series that
    /// should be used as the denominator of a ratio
    /// that will be compared with the threshold. If
    /// a denominator_filter is specified, the time
    /// series specified by the filter field will be
    /// used as the numerator.The filter is similar
    /// to the one that is specified in the
    /// MetricService.ListTimeSeries request (that
    /// call is useful to verify the time series
    /// that will be retrieved / processed) and must
    /// specify the metric type and optionally may
    /// contain restrictions on resource type,
    /// resource labels, and metric labels. This
    /// field may not exceed 2048 Unicode characters
    /// in length.
    #[builder(into, default)]
    #[serde(rename = "denominatorFilter")]
    pub r#denominator_filter: Box<Option<String>>,
    /// The amount of time that a time series must
    /// violate the threshold to be considered
    /// failing. Currently, only values that are a
    /// multiple of a minute--e.g., 0, 60, 120, or
    /// 300 seconds--are supported. If an invalid
    /// value is given, an error will be returned.
    /// When choosing a duration, it is useful to
    /// keep in mind the frequency of the underlying
    /// time series data (which may also be affected
    /// by any alignments specified in the
    /// aggregations field); a good duration is long
    /// enough so that a single outlier does not
    /// generate spurious alerts, but short enough
    /// that unhealthy states are detected and
    /// alerted on quickly.
    #[builder(into)]
    #[serde(rename = "duration")]
    pub r#duration: Box<String>,
    /// A condition control that determines how
    /// metric-threshold conditions are evaluated when
    /// data stops arriving.
    /// Possible values are: `EVALUATION_MISSING_DATA_INACTIVE`, `EVALUATION_MISSING_DATA_ACTIVE`, `EVALUATION_MISSING_DATA_NO_OP`.
    #[builder(into, default)]
    #[serde(rename = "evaluationMissingData")]
    pub r#evaluation_missing_data: Box<Option<String>>,
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
    /// When this field is present, the `MetricThreshold`
    /// condition forecasts whether the time series is
    /// predicted to violate the threshold within the
    /// `forecastHorizon`. When this field is not set, the
    /// `MetricThreshold` tests the current value of the
    /// timeseries against the threshold.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "forecastOptions")]
    pub r#forecast_options: Box<Option<super::super::types::monitoring::AlertPolicyConditionConditionThresholdForecastOptions>>,
    /// A value against which to compare the time
    /// series.
    #[builder(into, default)]
    #[serde(rename = "thresholdValue")]
    pub r#threshold_value: Box<Option<f64>>,
    /// The number/percent of time series for which
    /// the comparison must hold in order for the
    /// condition to trigger. If unspecified, then
    /// the condition will trigger if the comparison
    /// is true for any of the time series that have
    /// been identified by filter and aggregations,
    /// or by the ratio, if denominator_filter and
    /// denominator_aggregations are specified.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "trigger")]
    pub r#trigger: Box<Option<super::super::types::monitoring::AlertPolicyConditionConditionThresholdTrigger>>,
}
