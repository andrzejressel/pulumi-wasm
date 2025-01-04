#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ServiceNetworkConfigurationEgressConfiguration {
    /// The type of egress configuration. Valid values are: `DEFAULT` and `VPC`.
    #[builder(into, default)]
    #[serde(rename = "egressType")]
    pub r#egress_type: Box<Option<String>>,
    /// The Amazon Resource Name (ARN) of the App Runner VPC connector that you want to associate with your App Runner service. Only valid when `EgressType = VPC`.
    #[builder(into, default)]
    #[serde(rename = "vpcConnectorArn")]
    pub r#vpc_connector_arn: Box<Option<String>>,
}
