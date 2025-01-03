#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetTaskExecutionNetworkConfiguration {
    /// Assign a public IP address to the ENI (Fargate launch type only). Valid values are `true` or `false`. Default `false`.
    /// 
    /// For more information, see the [Task Networking](https://docs.aws.amazon.com/AmazonECS/latest/developerguide/task-networking.html) documentation.
    #[builder(into, default)]
    #[serde(rename = "assignPublicIp")]
    pub r#assign_public_ip: Box<Option<bool>>,
    /// Security groups associated with the task or service. If you do not specify a security group, the default security group for the VPC is used.
    #[builder(into, default)]
    #[serde(rename = "securityGroups")]
    pub r#security_groups: Box<Option<Vec<String>>>,
    /// Subnets associated with the task or service.
    #[builder(into)]
    #[serde(rename = "subnets")]
    pub r#subnets: Box<Vec<String>>,
}
