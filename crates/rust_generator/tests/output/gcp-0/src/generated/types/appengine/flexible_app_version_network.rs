#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct FlexibleAppVersionNetwork {
    /// List of ports, or port pairs, to forward from the virtual machine to the application container.
    #[builder(into, default)]
    #[serde(rename = "forwardedPorts")]
    pub r#forwarded_ports: Box<Option<Vec<String>>>,
    /// Prevent instances from receiving an ephemeral external IP address.
    /// Possible values are: `EXTERNAL`, `INTERNAL`.
    #[builder(into, default)]
    #[serde(rename = "instanceIpMode")]
    pub r#instance_ip_mode: Box<Option<String>>,
    /// Tag to apply to the instance during creation.
    #[builder(into, default)]
    #[serde(rename = "instanceTag")]
    pub r#instance_tag: Box<Option<String>>,
    /// Google Compute Engine network where the virtual machines are created. Specify the short name, not the resource path.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// Enable session affinity.
    #[builder(into, default)]
    #[serde(rename = "sessionAffinity")]
    pub r#session_affinity: Box<Option<bool>>,
    /// Google Cloud Platform sub-network where the virtual machines are created. Specify the short name, not the resource path.
    /// If the network that the instance is being created in is a Legacy network, then the IP address is allocated from the IPv4Range.
    /// If the network that the instance is being created in is an auto Subnet Mode Network, then only network name should be specified (not the subnetworkName) and the IP address is created from the IPCidrRange of the subnetwork that exists in that zone for that network.
    /// If the network that the instance is being created in is a custom Subnet Mode Network, then the subnetworkName must be specified and the IP address is created from the IPCidrRange of the subnetwork.
    /// If specified, the subnetwork must exist in the same region as the App Engine flexible environment application.
    #[builder(into, default)]
    #[serde(rename = "subnetwork")]
    pub r#subnetwork: Box<Option<String>>,
}
