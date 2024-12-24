#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
pub struct ZeroTrustRiskBehaviorBehavior {
    /// Whether this risk behavior type is enabled.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<bool>,
    /// Name of this risk behavior type
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// Risk level. Available values: `low`, `medium`, `high`
    #[builder(into)]
    #[serde(rename = "riskLevel")]
    pub r#risk_level: Box<String>,
}
