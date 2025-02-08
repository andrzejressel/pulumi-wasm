#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct NetworkInterfaceIpConfiguration {
    /// The Frontend IP Configuration ID of a Gateway SKU Load Balancer.
    #[builder(into, default)]
    #[serde(rename = "gatewayLoadBalancerFrontendIpConfigurationId")]
    pub r#gateway_load_balancer_frontend_ip_configuration_id: Box<Option<String>>,
    /// A name used for this IP Configuration.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// Is this the Primary IP Configuration? Must be `true` for the first `ip_configuration` when multiple are specified. Defaults to `false`.
    #[builder(into, default)]
    #[serde(rename = "primary")]
    pub r#primary: Box<Option<bool>>,
    /// The first private IP address of the network interface.
    #[builder(into, default)]
    #[serde(rename = "privateIpAddress")]
    pub r#private_ip_address: Box<Option<String>>,
    /// The allocation method used for the Private IP Address. Possible values are `Dynamic` and `Static`.
    /// 
    /// > **Note:** `Dynamic` means "An IP is automatically assigned during creation of this Network Interface"; `Static` means "User supplied IP address will be used"
    #[builder(into)]
    #[serde(rename = "privateIpAddressAllocation")]
    pub r#private_ip_address_allocation: Box<String>,
    /// The IP Version to use. Possible values are `IPv4` or `IPv6`. Defaults to `IPv4`.
    #[builder(into, default)]
    #[serde(rename = "privateIpAddressVersion")]
    pub r#private_ip_address_version: Box<Option<String>>,
    /// Reference to a Public IP Address to associate with this NIC
    #[builder(into, default)]
    #[serde(rename = "publicIpAddressId")]
    pub r#public_ip_address_id: Box<Option<String>>,
    /// The ID of the Subnet where this Network Interface should be located in.
    /// 
    /// > **Note:** This is required when `private_ip_address_version` is set to `IPv4`.
    #[builder(into, default)]
    #[serde(rename = "subnetId")]
    pub r#subnet_id: Box<Option<String>>,
}
