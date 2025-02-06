#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ApplicationFeatureSettings {
    /// Set to false to use the legacy health check instead of the readiness
    /// and liveness checks.
    #[builder(into)]
    #[serde(rename = "splitHealthChecks")]
    pub r#split_health_checks: Box<bool>,
}
