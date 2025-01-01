#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetOrchestratedVirtualMachineScaleSetNetworkInterfaceIpConfigurationPublicIpAddress {
    /// The domain name label for the DNS settings.
    #[builder(into)]
    #[serde(rename = "domainNameLabel")]
    pub r#domain_name_label: Box<String>,
    /// The idle timeout in minutes.
    #[builder(into)]
    #[serde(rename = "idleTimeoutInMinutes")]
    pub r#idle_timeout_in_minutes: Box<i32>,
    /// A list of `ip_tag` blocks as defined below.
    #[builder(into)]
    #[serde(rename = "ipTags")]
    pub r#ip_tags: Box<Vec<super::super::types::compute::GetOrchestratedVirtualMachineScaleSetNetworkInterfaceIpConfigurationPublicIpAddressIpTag>>,
    /// The name of this Orchestrated Virtual Machine Scale Set.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The ID of the public IP prefix.
    #[builder(into)]
    #[serde(rename = "publicIpPrefixId")]
    pub r#public_ip_prefix_id: Box<String>,
    /// The Internet Protocol Version of the public IP address.
    #[builder(into)]
    #[serde(rename = "version")]
    pub r#version: Box<String>,
}
