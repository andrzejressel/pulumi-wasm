#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetDirectoryConnectSetting {
    #[builder(into)]
    #[serde(rename = "availabilityZones")]
    pub r#availability_zones: Box<Vec<String>>,
    /// IP addresses of the AD Connector servers.
    #[builder(into)]
    #[serde(rename = "connectIps")]
    pub r#connect_ips: Box<Vec<String>>,
    /// DNS IP addresses of the domain to connect to.
    #[builder(into)]
    #[serde(rename = "customerDnsIps")]
    pub r#customer_dns_ips: Box<Vec<String>>,
    /// Username corresponding to the password provided.
    #[builder(into)]
    #[serde(rename = "customerUsername")]
    pub r#customer_username: Box<String>,
    /// Identifiers of the subnets for the connector servers (2 subnets in 2 different AZs).
    #[builder(into)]
    #[serde(rename = "subnetIds")]
    pub r#subnet_ids: Box<Vec<String>>,
    /// ID of the VPC that the connector is in.
    #[builder(into)]
    #[serde(rename = "vpcId")]
    pub r#vpc_id: Box<String>,
}
