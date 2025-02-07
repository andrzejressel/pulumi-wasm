#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct LinuxVirtualMachineScaleSetNetworkInterfaceIpConfigurationPublicIpAddress {
    /// The Prefix which should be used for the Domain Name Label for each Virtual Machine Instance. Azure concatenates the Domain Name Label and Virtual Machine Index to create a unique Domain Name Label for each Virtual Machine.
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
    pub r#ip_tags: Box<Option<Vec<super::super::types::compute::LinuxVirtualMachineScaleSetNetworkInterfaceIpConfigurationPublicIpAddressIpTag>>>,
    /// The Name of the Public IP Address Configuration.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The ID of the Public IP Address Prefix from where Public IP Addresses should be allocated. Changing this forces a new resource to be created.
    /// 
    /// > **Note:** This functionality is in Preview and must be opted into via `az feature register --namespace Microsoft.Network --name AllowBringYourOwnPublicIpAddress` and then `az provider register -n Microsoft.Network`.
    #[builder(into, default)]
    #[serde(rename = "publicIpPrefixId")]
    pub r#public_ip_prefix_id: Box<Option<String>>,
    /// The Internet Protocol Version which should be used for this public IP address. Possible values are `IPv4` and `IPv6`. Defaults to `IPv4`. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "version")]
    pub r#version: Box<Option<String>>,
}
