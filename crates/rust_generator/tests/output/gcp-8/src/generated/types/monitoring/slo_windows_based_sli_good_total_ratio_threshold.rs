#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct SloWindowsBasedSliGoodTotalRatioThreshold {
    /// Basic SLI to evaluate to judge window quality.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "basicSliPerformance")]
    pub r#basic_sli_performance: Box<Option<super::super::types::monitoring::SloWindowsBasedSliGoodTotalRatioThresholdBasicSliPerformance>>,
    /// Request-based SLI to evaluate to judge window quality.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "performance")]
    pub r#performance: Box<Option<super::super::types::monitoring::SloWindowsBasedSliGoodTotalRatioThresholdPerformance>>,
    /// If window performance >= threshold, the window is counted
    /// as good.
    #[builder(into, default)]
    #[serde(rename = "threshold")]
    pub r#threshold: Box<Option<f64>>,
}
