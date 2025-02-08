#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct DeploymentConfigTrafficRoutingConfig {
    /// The time based canary configuration information. If `type` is `TimeBasedLinear`, use `time_based_linear` instead.
    #[builder(into, default)]
    #[serde(rename = "timeBasedCanary")]
    pub r#time_based_canary: Box<Option<super::super::types::codedeploy::DeploymentConfigTrafficRoutingConfigTimeBasedCanary>>,
    /// The time based linear configuration information. If `type` is `TimeBasedCanary`, use `time_based_canary` instead.
    #[builder(into, default)]
    #[serde(rename = "timeBasedLinear")]
    pub r#time_based_linear: Box<Option<super::super::types::codedeploy::DeploymentConfigTrafficRoutingConfigTimeBasedLinear>>,
    /// Type of traffic routing config. One of `TimeBasedCanary`, `TimeBasedLinear`, `AllAtOnce`.
    #[builder(into, default)]
    #[serde(rename = "type")]
    pub r#type_: Box<Option<String>>,
}
