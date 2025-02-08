#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ContinuousDeploymentPolicyTrafficConfig {
    /// Determines which HTTP requests are sent to the staging distribution. See `single_header_config`.
    #[builder(into, default)]
    #[serde(rename = "singleHeaderConfig")]
    pub r#single_header_config: Box<Option<super::super::types::cloudfront::ContinuousDeploymentPolicyTrafficConfigSingleHeaderConfig>>,
    /// Contains the percentage of traffic to send to the staging distribution. See `single_weight_config`.
    #[builder(into, default)]
    #[serde(rename = "singleWeightConfig")]
    pub r#single_weight_config: Box<Option<super::super::types::cloudfront::ContinuousDeploymentPolicyTrafficConfigSingleWeightConfig>>,
    /// Type of traffic configuration. Valid values are `SingleWeight` and `SingleHeader`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
}
