#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetClusterIdentityOidc {
    /// Issuer URL for the OpenID Connect identity provider.
    #[builder(into)]
    #[serde(rename = "issuer")]
    pub r#issuer: Box<String>,
}