#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetUptimeCheckIPsUptimeCheckIp {
    /// The IP address from which the Uptime check originates. This is a fully specified IP address
    /// (not an IP address range). Most IP addresses, as of this publication, are in IPv4 format; however, one should not
    /// rely on the IP addresses being in IPv4 format indefinitely, and should support interpreting this field in either
    /// IPv4 or IPv6 format.
    #[builder(into)]
    #[serde(rename = "ipAddress")]
    pub r#ip_address: Box<String>,
    /// A more specific location within the region that typically encodes a particular city/town/metro
    /// (and its containing state/province or country) within the broader umbrella region category.
    #[builder(into)]
    #[serde(rename = "location")]
    pub r#location: Box<String>,
    /// A broad region category in which the IP address is located.
    #[builder(into)]
    #[serde(rename = "region")]
    pub r#region: Box<String>,
}
