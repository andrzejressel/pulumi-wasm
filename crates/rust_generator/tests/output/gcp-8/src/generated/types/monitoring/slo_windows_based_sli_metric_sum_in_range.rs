#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct SloWindowsBasedSliMetricSumInRange {
    /// Range of numerical values. The computed good_service
    /// will be the count of values x in the Distribution such
    /// that range.min <= x <= range.max. inclusive of min and
    /// max. Open ranges can be defined by setting
    /// just one of min or max. Summed value `X` should satisfy
    /// `range.min <= X <= range.max` for a good window.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "range")]
    pub r#range: Box<super::super::types::monitoring::SloWindowsBasedSliMetricSumInRangeRange>,
    /// A [monitoring filter](https://cloud.google.com/monitoring/api/v3/filters)
    /// specifying the TimeSeries to use for evaluating window
    /// quality. The provided TimeSeries must have
    /// ValueType = INT64 or ValueType = DOUBLE and
    /// MetricKind = GAUGE.
    /// Summed value `X` should satisfy
    /// `range.min <= X <= range.max` for a good window.
    #[builder(into)]
    #[serde(rename = "timeSeries")]
    pub r#time_series: Box<String>,
}
