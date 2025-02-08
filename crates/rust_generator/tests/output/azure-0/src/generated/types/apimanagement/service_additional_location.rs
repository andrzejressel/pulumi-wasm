#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ServiceAdditionalLocation {
    /// The number of compute units in this region. Defaults to the capacity of the main region.
    #[builder(into, default)]
    #[serde(rename = "capacity")]
    pub r#capacity: Box<Option<i32>>,
    /// Only valid for an Api Management service deployed in multiple locations. This can be used to disable the gateway in this additional location.
    #[builder(into, default)]
    #[serde(rename = "gatewayDisabled")]
    pub r#gateway_disabled: Box<Option<bool>>,
    /// The URL of the Regional Gateway for the API Management Service in the specified region.
    #[builder(into, default)]
    #[serde(rename = "gatewayRegionalUrl")]
    pub r#gateway_regional_url: Box<Option<String>>,
    /// The name of the Azure Region in which the API Management Service should be expanded to.
    #[builder(into)]
    #[serde(rename = "location")]
    pub r#location: Box<String>,
    /// The Private IP addresses of the API Management Service. Available only when the API Manager instance is using Virtual Network mode.
    #[builder(into, default)]
    #[serde(rename = "privateIpAddresses")]
    pub r#private_ip_addresses: Box<Option<Vec<String>>>,
    /// ID of a standard SKU IPv4 Public IP.
    /// 
    /// > **NOTE:** Availability zones and custom public IPs are only supported in the Premium tier.
    #[builder(into, default)]
    #[serde(rename = "publicIpAddressId")]
    pub r#public_ip_address_id: Box<Option<String>>,
    /// Public Static Load Balanced IP addresses of the API Management service in the additional location. Available only for Basic, Standard and Premium SKU.
    #[builder(into, default)]
    #[serde(rename = "publicIpAddresses")]
    pub r#public_ip_addresses: Box<Option<Vec<String>>>,
    /// A `virtual_network_configuration` block as defined below. Required when `virtual_network_type` is `External` or `Internal`.
    #[builder(into, default)]
    #[serde(rename = "virtualNetworkConfiguration")]
    pub r#virtual_network_configuration: Box<Option<super::super::types::apimanagement::ServiceAdditionalLocationVirtualNetworkConfiguration>>,
    /// A list of availability zones.
    #[builder(into, default)]
    #[serde(rename = "zones")]
    pub r#zones: Box<Option<Vec<String>>>,
}
