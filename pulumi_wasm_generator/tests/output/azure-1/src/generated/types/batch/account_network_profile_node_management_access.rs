#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AccountNetworkProfileNodeManagementAccess {
    /// Specifies the default action for the node management access. Possible values are `Allow` and `Deny`. Defaults to `Deny`.
    #[builder(into, default)]
    #[serde(rename = "defaultAction")]
    pub r#default_action: Box<Option<String>>,
    /// One or more `ip_rule` blocks as defined below.
    #[builder(into, default)]
    #[serde(rename = "ipRules")]
    pub r#ip_rules: Box<Option<Vec<super::super::types::batch::AccountNetworkProfileNodeManagementAccessIpRule>>>,
}
