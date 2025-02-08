#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetServiceAdditionalLocation {
    /// Specifies the number of units associated with this API Management service.
    #[builder(into)]
    #[serde(rename = "capacity")]
    pub r#capacity: Box<i32>,
    /// Gateway URL of the API Management service in the Region.
    #[builder(into)]
    #[serde(rename = "gatewayRegionalUrl")]
    pub r#gateway_regional_url: Box<String>,
    /// The location name of the additional region among Azure Data center regions.
    #[builder(into)]
    #[serde(rename = "location")]
    pub r#location: Box<String>,
    /// Private IP addresses of the API Management service in the additional location, for instances using virtual network mode.
    #[builder(into)]
    #[serde(rename = "privateIpAddresses")]
    pub r#private_ip_addresses: Box<Vec<String>>,
    /// ID of the standard SKU IPv4 Public IP. Available only for Premium SKU deployed in a virtual network.
    #[builder(into)]
    #[serde(rename = "publicIpAddressId")]
    pub r#public_ip_address_id: Box<String>,
    /// Public Static Load Balanced IP addresses of the API Management service in the additional location. Available only for Basic, Standard and Premium SKU.
    #[builder(into)]
    #[serde(rename = "publicIpAddresses")]
    pub r#public_ip_addresses: Box<Vec<String>>,
    /// List of the availability zones where API Management is deployed in the additional region exists.
    #[builder(into)]
    #[serde(rename = "zones")]
    pub r#zones: Box<Vec<String>>,
}
