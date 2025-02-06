#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetMetastoreServiceScalingConfigAutoscalingConfig {
    /// Defines whether autoscaling is enabled. The default value is false.
    #[builder(into)]
    #[serde(rename = "autoscalingEnabled")]
    pub r#autoscaling_enabled: Box<bool>,
    /// Represents the limit configuration of a metastore service.
    #[builder(into)]
    #[serde(rename = "limitConfigs")]
    pub r#limit_configs: Box<Vec<super::super::types::dataproc::GetMetastoreServiceScalingConfigAutoscalingConfigLimitConfig>>,
}
