#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct PipeTargetParametersEcsTaskParametersNetworkConfigurationAwsVpcConfiguration {
    /// Specifies whether the task's elastic network interface receives a public IP address. You can specify ENABLED only when LaunchType in EcsParameters is set to FARGATE. Valid Values: ENABLED, DISABLED.
    #[builder(into, default)]
    #[serde(rename = "assignPublicIp")]
    pub r#assign_public_ip: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "securityGroups")]
    pub r#security_groups: Box<Option<Vec<String>>>,
    #[builder(into, default)]
    #[serde(rename = "subnets")]
    pub r#subnets: Box<Option<Vec<String>>>,
}
