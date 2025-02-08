#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct OrchestratedVirtualMachineScaleSetNetworkInterfaceIpConfigurationPublicIpAddress {
    /// The Prefix which should be used for the Domain Name Label for each Virtual Machine Instance. Azure concatenates the Domain Name Label and Virtual Machine Index to create a unique Domain Name Label for each Virtual Machine. Valid values must be between `1` and `26` characters long, start with a lower case letter, end with a lower case letter or number and contains only `a-z`, `0-9` and `hyphens`.
    #[builder(into, default)]
    #[serde(rename = "domainNameLabel")]
    pub r#domain_name_label: Box<Option<String>>,
    /// The Idle Timeout in Minutes for the Public IP Address. Possible values are in the range `4` to `32`.
    #[builder(into, default)]
    #[serde(rename = "idleTimeoutInMinutes")]
    pub r#idle_timeout_in_minutes: Box<Option<i32>>,
    /// One or more `ip_tag` blocks as defined above. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "ipTags")]
    pub r#ip_tags: Box<Option<Vec<super::super::types::compute::OrchestratedVirtualMachineScaleSetNetworkInterfaceIpConfigurationPublicIpAddressIpTag>>>,
    /// The Name of the Public IP Address Configuration.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The ID of the Public IP Address Prefix from where Public IP Addresses should be allocated. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "publicIpPrefixId")]
    pub r#public_ip_prefix_id: Box<Option<String>>,
    /// Specifies what Public IP Address SKU the Public IP Address should be provisioned as. Possible vaules include `Basic_Regional`, `Basic_Global`, `Standard_Regional` or `Standard_Global`. For more information about Public IP Address SKU's and their capabilities, please see the [product documentation](https://docs.microsoft.com/azure/virtual-network/ip-services/public-ip-addresses#sku). Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "skuName")]
    pub r#sku_name: Box<Option<String>>,
    /// The Internet Protocol Version which should be used for this public IP address. Possible values are `IPv4` and `IPv6`. Defaults to `IPv4`. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "version")]
    pub r#version: Box<Option<String>>,
}
