#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct PrivateCloudNsx {
    /// Fully qualified domain name of the appliance.
    #[builder(into, default)]
    #[serde(rename = "fqdn")]
    pub r#fqdn: Box<Option<String>>,
    /// Internal IP address of the appliance.
    #[builder(into, default)]
    #[serde(rename = "internalIp")]
    pub r#internal_ip: Box<Option<String>>,
    /// State of the appliance.
    /// Possible values are: `ACTIVE`, `CREATING`.
    #[builder(into, default)]
    #[serde(rename = "state")]
    pub r#state: Box<Option<String>>,
    /// Version of the appliance.
    #[builder(into, default)]
    #[serde(rename = "version")]
    pub r#version: Box<Option<String>>,
}
