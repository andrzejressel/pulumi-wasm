#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetUserAlternateIdentifierExternalId {
    /// The identifier issued to this resource by an external identity provider.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Box<String>,
    /// The issuer for an external identifier.
    #[builder(into)]
    #[serde(rename = "issuer")]
    pub r#issuer: Box<String>,
}
