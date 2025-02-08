#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ContinuousDeploymentPolicyTrafficConfigSingleWeightConfig {
    /// Session stickiness provides the ability to define multiple requests from a single viewer as a single session. This prevents the potentially inconsistent experience of sending some of a given user's requests to the staging distribution, while others are sent to the primary distribution. Define the session duration using TTL values. See `session_stickiness_config`.
    #[builder(into, default)]
    #[serde(rename = "sessionStickinessConfig")]
    pub r#session_stickiness_config: Box<Option<super::super::types::cloudfront::ContinuousDeploymentPolicyTrafficConfigSingleWeightConfigSessionStickinessConfig>>,
    /// The percentage of traffic to send to a staging distribution, expressed as a decimal number between `0` and `.15`.
    #[builder(into)]
    #[serde(rename = "weight")]
    pub r#weight: Box<f64>,
}
