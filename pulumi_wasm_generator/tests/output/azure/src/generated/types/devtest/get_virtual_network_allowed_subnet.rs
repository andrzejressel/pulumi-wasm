#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetVirtualNetworkAllowedSubnet {
    /// Indicates if this subnet allows public IP addresses. Possible values are `Allow`, `Default` and `Deny`.
    #[builder(into)]
    #[serde(rename = "allowPublicIp")]
    pub r#allow_public_ip: Box<String>,
    /// The name of the subnet.
    #[builder(into)]
    #[serde(rename = "labSubnetName")]
    pub r#lab_subnet_name: Box<String>,
    /// The resource identifier for the subnet.
    #[builder(into)]
    #[serde(rename = "resourceId")]
    pub r#resource_id: Box<String>,
}