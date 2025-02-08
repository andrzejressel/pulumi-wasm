#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetApplicationGatewayFrontendIpConfiguration {
    /// The ID of the Rewrite Rule Set
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Box<String>,
    /// The name of this Application Gateway.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The Static IP Address which is used.
    #[builder(into)]
    #[serde(rename = "privateIpAddress")]
    pub r#private_ip_address: Box<String>,
    /// The allocation method used for the Private IP Address.
    #[builder(into)]
    #[serde(rename = "privateIpAddressAllocation")]
    pub r#private_ip_address_allocation: Box<String>,
    /// The ID of the associated Private Link configuration.
    #[builder(into)]
    #[serde(rename = "privateLinkConfigurationId")]
    pub r#private_link_configuration_id: Box<String>,
    /// The name of the Private Link configuration in use by this Frontend IP Configuration.
    #[builder(into)]
    #[serde(rename = "privateLinkConfigurationName")]
    pub r#private_link_configuration_name: Box<String>,
    /// The ID of the Public IP Address which the Application Gateway will use.
    #[builder(into)]
    #[serde(rename = "publicIpAddressId")]
    pub r#public_ip_address_id: Box<String>,
    /// The ID of the subnet the private link configuration is connected to.
    #[builder(into)]
    #[serde(rename = "subnetId")]
    pub r#subnet_id: Box<String>,
}
