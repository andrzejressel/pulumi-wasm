#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetInstanceNetwork {
    /// The network connect mode of the Filestore instance.
    /// If not provided, the connect mode defaults to
    /// DIRECT_PEERING. Default value: "DIRECT_PEERING" Possible values: ["DIRECT_PEERING", "PRIVATE_SERVICE_ACCESS"]
    #[builder(into)]
    #[serde(rename = "connectMode")]
    pub r#connect_mode: Box<String>,
    /// A list of IPv4 or IPv6 addresses.
    #[builder(into)]
    #[serde(rename = "ipAddresses")]
    pub r#ip_addresses: Box<Vec<String>>,
    /// IP versions for which the instance has
    /// IP addresses assigned. Possible values: ["ADDRESS_MODE_UNSPECIFIED", "MODE_IPV4", "MODE_IPV6"]
    #[builder(into)]
    #[serde(rename = "modes")]
    pub r#modes: Box<Vec<String>>,
    /// The name of the GCE VPC network to which the
    /// instance is connected.
    #[builder(into)]
    #[serde(rename = "network")]
    pub r#network: Box<String>,
    /// A /29 CIDR block that identifies the range of IP
    /// addresses reserved for this instance.
    #[builder(into)]
    #[serde(rename = "reservedIpRange")]
    pub r#reserved_ip_range: Box<String>,
}
