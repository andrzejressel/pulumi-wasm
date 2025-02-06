#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct VMwareClusterNetworkConfigStaticIpConfigIpBlockIp {
    /// Hostname of the machine. VM's name will be used if this field is empty.
    #[builder(into, default)]
    #[serde(rename = "hostname")]
    pub r#hostname: Box<Option<String>>,
    /// IP could be an IP address (like 1.2.3.4) or a CIDR (like 1.2.3.0/24).
    #[builder(into)]
    #[serde(rename = "ip")]
    pub r#ip: Box<String>,
}
