#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct SloRequestBasedSliDistributionCut {
    /// A TimeSeries [monitoring filter](https://cloud.google.com/monitoring/api/v3/filters)
    /// aggregating values to quantify the good service provided.
    /// Must have ValueType = DISTRIBUTION and
    /// MetricKind = DELTA or MetricKind = CUMULATIVE.
    #[builder(into)]
    #[serde(rename = "distributionFilter")]
    pub r#distribution_filter: Box<String>,
    /// Range of numerical values. The computed good_service
    /// will be the count of values x in the Distribution such
    /// that range.min <= x <= range.max. inclusive of min and
    /// max. Open ranges can be defined by setting
    /// just one of min or max.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "range")]
    pub r#range: Box<super::super::types::monitoring::SloRequestBasedSliDistributionCutRange>,
}
