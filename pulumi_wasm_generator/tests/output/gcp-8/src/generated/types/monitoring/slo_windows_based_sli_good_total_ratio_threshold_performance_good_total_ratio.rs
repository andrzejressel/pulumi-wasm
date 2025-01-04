#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct SloWindowsBasedSliGoodTotalRatioThresholdPerformanceGoodTotalRatio {
    /// A TimeSeries [monitoring filter](https://cloud.google.com/monitoring/api/v3/filters)
    /// quantifying bad service provided, either demanded service that
    /// was not provided or demanded service that was of inadequate
    /// quality. Exactly two of
    /// good, bad, or total service filter must be defined (where
    /// good + bad = total is assumed)
    /// Must have ValueType = DOUBLE or ValueType = INT64 and
    /// must have MetricKind = DELTA or MetricKind = CUMULATIVE.
    #[builder(into, default)]
    #[serde(rename = "badServiceFilter")]
    pub r#bad_service_filter: Box<Option<String>>,
    /// A TimeSeries [monitoring filter](https://cloud.google.com/monitoring/api/v3/filters)
    /// quantifying good service provided. Exactly two of
    /// good, bad, or total service filter must be defined (where
    /// good + bad = total is assumed)
    /// Must have ValueType = DOUBLE or ValueType = INT64 and
    /// must have MetricKind = DELTA or MetricKind = CUMULATIVE.
    #[builder(into, default)]
    #[serde(rename = "goodServiceFilter")]
    pub r#good_service_filter: Box<Option<String>>,
    /// A TimeSeries [monitoring filter](https://cloud.google.com/monitoring/api/v3/filters)
    /// quantifying total demanded service. Exactly two of
    /// good, bad, or total service filter must be defined (where
    /// good + bad = total is assumed)
    /// Must have ValueType = DOUBLE or ValueType = INT64 and
    /// must have MetricKind = DELTA or MetricKind = CUMULATIVE.
    #[builder(into, default)]
    #[serde(rename = "totalServiceFilter")]
    pub r#total_service_filter: Box<Option<String>>,
}
