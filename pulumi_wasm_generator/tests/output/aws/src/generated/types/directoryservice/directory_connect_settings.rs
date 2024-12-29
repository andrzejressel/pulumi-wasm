#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DirectoryConnectSettings {
    #[builder(into, default)]
    #[serde(rename = "availabilityZones")]
    pub r#availability_zones: Box<Option<Vec<String>>>,
    /// The IP addresses of the AD Connector servers.
    #[builder(into, default)]
    #[serde(rename = "connectIps")]
    pub r#connect_ips: Box<Option<Vec<String>>>,
    /// The DNS IP addresses of the domain to connect to.
    #[builder(into)]
    #[serde(rename = "customerDnsIps")]
    pub r#customer_dns_ips: Box<Vec<String>>,
    /// The username corresponding to the password provided.
    #[builder(into)]
    #[serde(rename = "customerUsername")]
    pub r#customer_username: Box<String>,
    /// The identifiers of the subnets for the directory servers (2 subnets in 2 different AZs).
    #[builder(into)]
    #[serde(rename = "subnetIds")]
    pub r#subnet_ids: Box<Vec<String>>,
    /// The identifier of the VPC that the directory is in.
    #[builder(into)]
    #[serde(rename = "vpcId")]
    pub r#vpc_id: Box<String>,
}
