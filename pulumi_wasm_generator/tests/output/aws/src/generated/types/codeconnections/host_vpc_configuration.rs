#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct HostVpcConfiguration {
    /// ID of the security group or security groups associated with the Amazon VPC connected to the infrastructure where your provider type is installed.
    #[builder(into)]
    #[serde(rename = "securityGroupIds")]
    pub r#security_group_ids: Box<Vec<String>>,
    /// The ID of the subnet or subnets associated with the Amazon VPC connected to the infrastructure where your provider type is installed.
    #[builder(into)]
    #[serde(rename = "subnetIds")]
    pub r#subnet_ids: Box<Vec<String>>,
    /// The value of the Transport Layer Security (TLS) certificate associated with the infrastructure where your provider type is installed.
    #[builder(into, default)]
    #[serde(rename = "tlsCertificate")]
    pub r#tls_certificate: Box<Option<String>>,
    /// The ID of the Amazon VPC connected to the infrastructure where your provider type is installed.
    #[builder(into)]
    #[serde(rename = "vpcId")]
    pub r#vpc_id: Box<String>,
}
