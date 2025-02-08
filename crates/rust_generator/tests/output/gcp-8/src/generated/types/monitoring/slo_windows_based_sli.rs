#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct SloWindowsBasedSli {
    /// A TimeSeries [monitoring filter](https://cloud.google.com/monitoring/api/v3/filters)
    /// with ValueType = BOOL. The window is good if any true values
    /// appear in the window. One of `good_bad_metric_filter`,
    /// `good_total_ratio_threshold`, `metric_mean_in_range`,
    /// `metric_sum_in_range` must be set for `windows_based_sli`.
    #[builder(into, default)]
    #[serde(rename = "goodBadMetricFilter")]
    pub r#good_bad_metric_filter: Box<Option<String>>,
    /// Criterion that describes a window as good if its performance is
    /// high enough. One of `good_bad_metric_filter`,
    /// `good_total_ratio_threshold`, `metric_mean_in_range`,
    /// `metric_sum_in_range` must be set for `windows_based_sli`.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "goodTotalRatioThreshold")]
    pub r#good_total_ratio_threshold: Box<Option<super::super::types::monitoring::SloWindowsBasedSliGoodTotalRatioThreshold>>,
    /// Criterion that describes a window as good if the metric's value
    /// is in a good range, *averaged* across returned streams.
    /// One of `good_bad_metric_filter`,
    /// `good_total_ratio_threshold`, `metric_mean_in_range`,
    /// `metric_sum_in_range` must be set for `windows_based_sli`.
    /// Average value X of `time_series` should satisfy
    /// `range.min <= X <= range.max` for a good window.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "metricMeanInRange")]
    pub r#metric_mean_in_range: Box<Option<super::super::types::monitoring::SloWindowsBasedSliMetricMeanInRange>>,
    /// Criterion that describes a window as good if the metric's value
    /// is in a good range, *summed* across returned streams.
    /// Summed value `X` of `time_series` should satisfy
    /// `range.min <= X <= range.max` for a good window.
    /// One of `good_bad_metric_filter`,
    /// `good_total_ratio_threshold`, `metric_mean_in_range`,
    /// `metric_sum_in_range` must be set for `windows_based_sli`.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "metricSumInRange")]
    pub r#metric_sum_in_range: Box<Option<super::super::types::monitoring::SloWindowsBasedSliMetricSumInRange>>,
    /// Duration over which window quality is evaluated, given as a
    /// duration string "{X}s" representing X seconds. Must be an
    /// integer fraction of a day and at least 60s.
    #[builder(into, default)]
    #[serde(rename = "windowPeriod")]
    pub r#window_period: Box<Option<String>>,
}
