#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
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
