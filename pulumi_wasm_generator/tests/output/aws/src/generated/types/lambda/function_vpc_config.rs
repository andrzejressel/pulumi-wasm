#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FunctionVpcConfig {
    /// Allows outbound IPv6 traffic on VPC functions that are connected to dual-stack subnets. Default is `false`.
    #[builder(into, default)]
    #[serde(rename = "ipv6AllowedForDualStack")]
    pub r#ipv_6_allowed_for_dual_stack: Box<Option<bool>>,
    /// List of security group IDs associated with the Lambda function.
    #[builder(into)]
    #[serde(rename = "securityGroupIds")]
    pub r#security_group_ids: Box<Vec<String>>,
    /// List of subnet IDs associated with the Lambda function.
    #[builder(into)]
    #[serde(rename = "subnetIds")]
    pub r#subnet_ids: Box<Vec<String>>,
    /// ID of the VPC.
    #[builder(into, default)]
    #[serde(rename = "vpcId")]
    pub r#vpc_id: Box<Option<String>>,
}