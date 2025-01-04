#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct SloWindowsBasedSliMetricMeanInRange {
    /// Range of numerical values. The computed good_service
    /// will be the count of values x in the Distribution such
    /// that range.min <= x <= range.max. inclusive of min and
    /// max. Open ranges can be defined by setting
    /// just one of min or max. Mean value `X` of `time_series`
    /// values should satisfy `range.min <= X <= range.max` for a
    /// good service.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "range")]
    pub r#range: Box<super::super::types::monitoring::SloWindowsBasedSliMetricMeanInRangeRange>,
    /// A [monitoring filter](https://cloud.google.com/monitoring/api/v3/filters)
    /// specifying the TimeSeries to use for evaluating window
    /// The provided TimeSeries must have ValueType = INT64 or
    /// ValueType = DOUBLE and MetricKind = GAUGE. Mean value `X`
    /// should satisfy `range.min <= X <= range.max`
    /// under good service.
    #[builder(into)]
    #[serde(rename = "timeSeries")]
    pub r#time_series: Box<String>,
}
