#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct VpnServerConfigurationPolicyGroupPolicy {
    /// The name of the VPN Server Configuration Policy member.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The attribute type of the VPN Server Configuration Policy member. Possible values are `AADGroupId`, `CertificateGroupId` and `RadiusAzureGroupId`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
    /// The value of the attribute that is used for the VPN Server Configuration Policy member.
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: Box<String>,
}
