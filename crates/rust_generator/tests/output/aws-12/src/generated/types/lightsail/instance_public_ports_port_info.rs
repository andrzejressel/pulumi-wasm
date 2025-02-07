#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct InstancePublicPortsPortInfo {
    /// Set of CIDR aliases that define access for a preconfigured range of IP addresses.
    #[builder(into, default)]
    #[serde(rename = "cidrListAliases")]
    pub r#cidr_list_aliases: Box<Option<Vec<String>>>,
    /// Set of CIDR blocks.
    #[builder(into, default)]
    #[serde(rename = "cidrs")]
    pub r#cidrs: Box<Option<Vec<String>>>,
    /// First port in a range of open ports on an instance.
    #[builder(into)]
    #[serde(rename = "fromPort")]
    pub r#from_port: Box<i32>,
    #[builder(into, default)]
    #[serde(rename = "ipv6Cidrs")]
    pub r#ipv_6_cidrs: Box<Option<Vec<String>>>,
    /// IP protocol name. Valid values are `tcp`, `all`, `udp`, and `icmp`.
    #[builder(into)]
    #[serde(rename = "protocol")]
    pub r#protocol: Box<String>,
    /// Last port in a range of open ports on an instance.
    /// 
    /// The following arguments are optional:
    #[builder(into)]
    #[serde(rename = "toPort")]
    pub r#to_port: Box<i32>,
}
