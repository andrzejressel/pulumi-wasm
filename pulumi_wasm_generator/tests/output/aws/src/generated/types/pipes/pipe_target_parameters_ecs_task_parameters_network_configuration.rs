#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct PipeTargetParametersEcsTaskParametersNetworkConfiguration {
    /// Use this structure to specify the VPC subnets and security groups for the task, and whether a public IP address is to be used. This structure is relevant only for ECS tasks that use the awsvpc network mode. Detailed below.
    #[builder(into, default)]
    #[serde(rename = "awsVpcConfiguration")]
    pub r#aws_vpc_configuration: Box<Option<super::super::types::pipes::PipeTargetParametersEcsTaskParametersNetworkConfigurationAwsVpcConfiguration>>,
}
