#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct BudgetThresholdRule {
    /// The type of basis used to determine if spend has passed
    /// the threshold.
    /// Default value is `CURRENT_SPEND`.
    /// Possible values are: `CURRENT_SPEND`, `FORECASTED_SPEND`.
    #[builder(into, default)]
    #[serde(rename = "spendBasis")]
    pub r#spend_basis: Box<Option<String>>,
    /// Send an alert when this threshold is exceeded. This is a
    /// 1.0-based percentage, so 0.5 = 50%. Must be >= 0.
    #[builder(into)]
    #[serde(rename = "thresholdPercent")]
    pub r#threshold_percent: Box<f64>,
}
