#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct WorkspaceServerlessCompute {
    /// Should serverless compute nodes deployed in a custom Virtual Network have public IP addresses enabled for a workspace with private endpoint? Defaults to `false`.
    /// 
    /// > **Note:** `public_ip_enabled` cannot be updated from `true` to `false` when `subnet_id` is not set. `public_ip_enabled` must be set to `true` if `subnet_id` is not set and when `public_network_access_enabled` is `false`.
    #[builder(into, default)]
    #[serde(rename = "publicIpEnabled")]
    pub r#public_ip_enabled: Box<Option<bool>>,
    /// The ID of an existing Virtual Network Subnet in which the serverless compute nodes should be deployed to.
    #[builder(into, default)]
    #[serde(rename = "subnetId")]
    pub r#subnet_id: Box<Option<String>>,
}
