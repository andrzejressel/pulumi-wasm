#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct SloWindowsBasedSliGoodTotalRatioThresholdPerformance {
    /// Used when good_service is defined by a count of values aggregated in a
    /// Distribution that fall into a good range. The total_service is the
    /// total count of all values aggregated in the Distribution.
    /// Defines a distribution TimeSeries filter and thresholds used for
    /// measuring good service and total service.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "distributionCut")]
    pub r#distribution_cut: Box<Option<super::super::types::monitoring::SloWindowsBasedSliGoodTotalRatioThresholdPerformanceDistributionCut>>,
    /// A means to compute a ratio of `good_service` to `total_service`.
    /// Defines computing this ratio with two TimeSeries [monitoring filters](https://cloud.google.com/monitoring/api/v3/filters)
    /// Must specify exactly two of good, bad, and total service filters.
    /// The relationship good_service + bad_service = total_service
    /// will be assumed.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "goodTotalRatio")]
    pub r#good_total_ratio: Box<Option<super::super::types::monitoring::SloWindowsBasedSliGoodTotalRatioThresholdPerformanceGoodTotalRatio>>,
}
