#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ResolverInboundEndpointIpConfigurations {
    /// Private IP address of the IP configuration.
    #[builder(into, default)]
    #[serde(rename = "privateIpAddress")]
    pub r#private_ip_address: Box<Option<String>>,
    /// Private IP address allocation method. Allowed value is `Dynamic` and `Static`. Defaults to `Dynamic`.
    #[builder(into, default)]
    #[serde(rename = "privateIpAllocationMethod")]
    pub r#private_ip_allocation_method: Box<Option<String>>,
    /// The subnet ID of the IP configuration.
    #[builder(into)]
    #[serde(rename = "subnetId")]
    pub r#subnet_id: Box<String>,
}