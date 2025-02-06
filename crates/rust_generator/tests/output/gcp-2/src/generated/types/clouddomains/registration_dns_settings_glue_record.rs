#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct RegistrationDnsSettingsGlueRecord {
    /// Required. Domain name of the host in Punycode format.
    #[builder(into)]
    #[serde(rename = "hostName")]
    pub r#host_name: Box<String>,
    /// List of IPv4 addresses corresponding to this host in the standard decimal format (e.g. 198.51.100.1).
    /// At least one of ipv4_address and ipv6_address must be set.
    #[builder(into, default)]
    #[serde(rename = "ipv4Addresses")]
    pub r#ipv_4_addresses: Box<Option<Vec<String>>>,
    /// List of IPv4 addresses corresponding to this host in the standard decimal format (e.g. 198.51.100.1).
    /// At least one of ipv4_address and ipv6_address must be set.
    #[builder(into, default)]
    #[serde(rename = "ipv6Addresses")]
    pub r#ipv_6_addresses: Box<Option<Vec<String>>>,
}
