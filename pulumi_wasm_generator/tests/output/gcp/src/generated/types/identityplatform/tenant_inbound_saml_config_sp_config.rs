#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct TenantInboundSamlConfigSpConfig {
    /// Callback URI where responses from IDP are handled. Must start with `https://`.
    #[builder(into)]
    #[serde(rename = "callbackUri")]
    pub r#callback_uri: Box<String>,
    /// (Output)
    /// The IDP's certificate data to verify the signature in the SAMLResponse issued by the IDP.
    /// Structure is documented below.
    /// 
    /// 
    /// <a name="nested_sp_certificates"></a>The `sp_certificates` block contains:
    #[builder(into, default)]
    #[serde(rename = "spCertificates")]
    pub r#sp_certificates: Box<Option<Vec<super::super::types::identityplatform::TenantInboundSamlConfigSpConfigSpCertificate>>>,
    /// Unique identifier for all SAML entities.
    #[builder(into)]
    #[serde(rename = "spEntityId")]
    pub r#sp_entity_id: Box<String>,
}
