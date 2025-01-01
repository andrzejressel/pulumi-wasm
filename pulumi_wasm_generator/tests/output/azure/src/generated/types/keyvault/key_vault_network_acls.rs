#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct KeyVaultNetworkAcls {
    /// Specifies which traffic can bypass the network rules. Possible values are `AzureServices` and `None`.
    #[builder(into)]
    #[serde(rename = "bypass")]
    pub r#bypass: Box<String>,
    /// The Default Action to use when no rules match from `ip_rules` / `virtual_network_subnet_ids`. Possible values are `Allow` and `Deny`.
    #[builder(into)]
    #[serde(rename = "defaultAction")]
    pub r#default_action: Box<String>,
    /// One or more IP Addresses, or CIDR Blocks which should be able to access the Key Vault.
    #[builder(into, default)]
    #[serde(rename = "ipRules")]
    pub r#ip_rules: Box<Option<Vec<String>>>,
    /// One or more Subnet IDs which should be able to access this Key Vault.
    #[builder(into, default)]
    #[serde(rename = "virtualNetworkSubnetIds")]
    pub r#virtual_network_subnet_ids: Box<Option<Vec<String>>>,
}
