#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetEnvironmentConfigMasterAuthorizedNetworksConfigCidrBlock {
    /// cidr_block must be specified in CIDR notation.
    #[builder(into)]
    #[serde(rename = "cidrBlock")]
    pub r#cidr_block: Box<String>,
    /// display_name is a field for users to identify CIDR blocks.
    #[builder(into)]
    #[serde(rename = "displayName")]
    pub r#display_name: Box<String>,
}
