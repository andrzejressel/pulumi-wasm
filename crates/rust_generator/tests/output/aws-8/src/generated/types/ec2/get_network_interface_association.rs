#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetNetworkInterfaceAssociation {
    /// Allocation ID.
    #[builder(into)]
    #[serde(rename = "allocationId")]
    pub r#allocation_id: Box<String>,
    /// Association ID.
    #[builder(into)]
    #[serde(rename = "associationId")]
    pub r#association_id: Box<String>,
    /// Carrier IP address associated with the network interface. This attribute is only set when the network interface is in a subnet which is associated with a Wavelength Zone.
    #[builder(into)]
    #[serde(rename = "carrierIp")]
    pub r#carrier_ip: Box<String>,
    /// Customer-owned IP address.
    #[builder(into)]
    #[serde(rename = "customerOwnedIp")]
    pub r#customer_owned_ip: Box<String>,
    /// ID of the Elastic IP address owner.
    #[builder(into)]
    #[serde(rename = "ipOwnerId")]
    pub r#ip_owner_id: Box<String>,
    /// Public DNS name.
    #[builder(into)]
    #[serde(rename = "publicDnsName")]
    pub r#public_dns_name: Box<String>,
    /// Address of the Elastic IP address bound to the network interface.
    #[builder(into)]
    #[serde(rename = "publicIp")]
    pub r#public_ip: Box<String>,
}
