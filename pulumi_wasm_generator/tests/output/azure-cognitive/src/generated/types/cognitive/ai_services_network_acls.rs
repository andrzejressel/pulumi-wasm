#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AiServicesNetworkAcls {
    /// The Default Action to use when no rules match from `ip_rules` / `virtual_network_rules`. Possible values are `Allow` and `Deny`.
    #[builder(into)]
    #[serde(rename = "defaultAction")]
    pub r#default_action: Box<String>,
    /// One or more IP Addresses, or CIDR Blocks which should be able to access the AI Services Account.
    #[builder(into, default)]
    #[serde(rename = "ipRules")]
    pub r#ip_rules: Box<Option<Vec<String>>>,
    /// A `virtual_network_rules` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "virtualNetworkRules")]
    pub r#virtual_network_rules: Box<Option<Vec<super::super::types::cognitive::AiServicesNetworkAclsVirtualNetworkRule>>>,
}