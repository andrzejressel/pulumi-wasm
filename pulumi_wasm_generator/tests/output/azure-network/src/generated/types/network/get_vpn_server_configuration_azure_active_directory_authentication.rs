#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetVpnServerConfigurationAzureActiveDirectoryAuthentication {
    /// The Audience which should be used for authentication.
    #[builder(into)]
    #[serde(rename = "audience")]
    pub r#audience: Box<String>,
    /// The Issuer which should be used for authentication.
    #[builder(into)]
    #[serde(rename = "issuer")]
    pub r#issuer: Box<String>,
    /// The Tenant which should be used for authentication.
    #[builder(into)]
    #[serde(rename = "tenant")]
    pub r#tenant: Box<String>,
}