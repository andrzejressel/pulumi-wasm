#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct InputSecurityGroupWhitelistRule {
    /// The IPv4 CIDR that's whitelisted.
    #[builder(into)]
    #[serde(rename = "cidr")]
    pub r#cidr: Box<String>,
}